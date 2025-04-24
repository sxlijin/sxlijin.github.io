use anyhow::{Context, Result};
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Serialize;
use std::{net::SocketAddr, path::PathBuf};
use tokio::{net::TcpListener, sync::broadcast};
use tokio_stream::StreamExt;
// use tower_http::trace::TraceLayer;
use notify::{Event, FsEventWatcher, RecursiveMode, Watcher};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, Serialize)]
struct FileChangeEvent {
    paths: Vec<PathBuf>,
}
struct AppState {
    rx: broadcast::Receiver<FileChangeEvent>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            rx: self.rx.resubscribe(),
        }
    }
}

// Handler for the SSE endpoint
async fn sse_handler(State(state): State<AppState>) -> impl IntoResponse {
    if state.rx.is_closed() {
        tracing::error!("SSE channel is closed");
    }

    let stream =
        tokio_stream::wrappers::BroadcastStream::new(state.rx).map(|result| match result {
            Ok(msg) => {
                tracing::info!("SSE event: {:?}", msg);
                axum::response::sse::Event::default().json_data(msg)
            }
            Err(e) => {
                tracing::error!("Error in SSE stream: {}", e);
                Err(axum::Error::new(e))
            }
        });

    axum::response::sse::Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keep-alive-text"),
    )
}

async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|ws| async move { ws_handler_impl(state, ws).await })
}

// Handler for the WebSocket endpoint
async fn ws_handler_impl(mut state: AppState, mut ws: WebSocket) {
    if state.rx.is_closed() {
        tracing::error!("SSE channel is closed");
    }

    // let _ = ws.send(axum::extract::ws::Message::Text(
    //     serde_json::to_string(&FileChangeEvent {
    //         paths: vec![PathBuf::from("initial-event")],
    //     })
    //     .expect("Failed to serialize message")
    //     .into(),
    // ));

    while let Ok(msg) = state.rx.recv().await {
        if let Err(e) = ws
            .send(axum::extract::ws::Message::Text(
                serde_json::to_string(&msg)
                    .expect("Failed to serialize message")
                    .into(),
            ))
            .await
        {
            tracing::error!("Failed to send WebSocket message: {}", e);
            break;
        }
    }
}

// Setup tracing for the application
fn setup_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// Setup file watcher and return the broadcast receiver
fn setup_hot_refresh() -> Result<(
    FsEventWatcher,
    broadcast::Sender<FileChangeEvent>,
    broadcast::Receiver<FileChangeEvent>,
)> {
    // Create a broadcast channel for SSE
    let (tx, rx) = broadcast::channel(100);
    let tx2 = tx.clone();
    // Create a file watcher
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            let paths: Vec<PathBuf> = event.paths;
            if !paths.is_empty() {
                let message = FileChangeEvent { paths };
                // If there are no receivers, the message is dropped: that's fine.
                let tx_result = tx.send(message);
                match tx_result {
                    Ok(_) => {
                        tracing::debug!("Sent message to SSE channel");
                    }
                    Err(e) => {
                        tracing::error!("Failed to send message to SSE channel: {}", e);
                    }
                }
            }
        }
    })
    .context("Failed to create watcher")?;

    // Add the _site directory to the watcher
    watcher
        .watch(&PathBuf::from("_site"), RecursiveMode::Recursive)
        .context("Failed to watch _site directory")?;

    Ok((watcher, tx2, rx))
}

pub fn setup_hot_rebuild() -> Result<(
    FsEventWatcher,
    broadcast::Sender<FileChangeEvent>,
    broadcast::Receiver<FileChangeEvent>,
)> {
    let (tx, rx) = broadcast::channel(100);
    let tx2 = tx.clone();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            let paths: Vec<PathBuf> = event.paths;
            if !paths.is_empty() {
                let message = FileChangeEvent { paths };
                // If there are no receivers, the message is dropped: that's fine.
                let tx_result = tx.send(message);
                match tx_result {
                    Ok(_) => {
                        tracing::debug!("Sent message to SSE channel");
                    }
                    Err(e) => {
                        tracing::error!("Failed to send message to SSE channel: {}", e);
                    }
                }
            }
        }
    })
    .context("Failed to create watcher")?;

    watcher
        .watch(&PathBuf::from("pages"), RecursiveMode::Recursive)
        .context("Failed to watch pages directory")?;

    Ok((watcher, tx2, rx))
}

// Start the server
pub async fn start_server(port: u16) -> Result<()> {
    // Initialize tracing
    setup_tracing();

    // Setup file watcher and get the receiver
    let (_watcher, _tx, rx) = setup_hot_refresh()?;
    let state = AppState { rx };
    let (_rebuild_watcher, tx2, mut rx2) = setup_hot_rebuild()?;
    let _ = tx2.send(FileChangeEvent {
        paths: vec![PathBuf::from("pages")],
    });

    tokio::spawn(async move {
        while let Ok(_msg) = rx2.recv().await {
            match sam_website::build_content::build_all() {
                Ok(_) => {
                    tracing::info!("Rebuild successful");
                }
                Err(e) => {
                    tracing::error!("Rebuild failed: {:?}", e);
                }
            }
        }
    });

    // Build the router
    let app = Router::new()
        .route("/_debug/reload", get(sse_handler))
        .route("/_debug/reload2", get(ws_handler))
        // .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("_site"))
        .with_state(state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Starting Axum server on {}", addr);

    axum::serve(TcpListener::bind(addr).await?, app.into_make_service()).await?;
    //  for entry in app.foo
    let _w = _watcher;

    Ok(())
}

// Main function to run the server
#[tokio::main]
async fn main() -> Result<()> {
    std::fs::create_dir_all("_site")?;
    start_server(3000).await?;
    Ok(())
}
