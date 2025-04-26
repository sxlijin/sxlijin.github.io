use anyhow::{Context, Result};
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use notify::{Event, FsEventWatcher, RecursiveMode, Watcher};
use sam_website::build_content;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    ops::DerefMut,
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    net::TcpListener,
    select,
    sync::{broadcast, Mutex},
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, Serialize)]
struct FileChangeEvent {
    paths: Vec<PathBuf>,
}
struct AppState {
    rx: broadcast::Receiver<FileChangeEvent>,
    latest_build_timestamp: Arc<Mutex<RebuildEvent>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            rx: self.rx.resubscribe(),
            latest_build_timestamp: self.latest_build_timestamp.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct RebuildEvent {
    build_timestamp: SystemTime,
}

async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|ws| async move { ws_handler_impl(state, ws).await })
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
enum WebsocketRequest {
    GetBuildStaleness {
        build_timestamp: build_content::BuildTimestamp,
    },
}

async fn ws_handler_impl(mut state: AppState, mut ws: WebSocket) {
    tracing::info!("WebSocket connection established");

    loop {
        select! {
            Some(Ok(axum::extract::ws::Message::Text(ws_req))) = ws.recv() => {
                tracing::info!("Received websocket request: {}", ws_req);
                if let Err(e) = ws
                    .send(axum::extract::ws::Message::Text(
                        serde_json::to_string(&RebuildEvent { build_timestamp: state.latest_build_timestamp.lock().await.build_timestamp })
                            .expect("Failed to serialize message")
                            .into(),
                    ))
                    .await
                {
                    tracing::error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
            Ok(site_changed) = state.rx.recv() => {
                if let Err(e) = ws
                    .send(axum::extract::ws::Message::Text(
                        serde_json::to_string(&site_changed)
                            .expect("Failed to serialize message")
                            .into(),
                    ))
                    .await
                {
                    tracing::error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
            else => break,
        }
    }
    tracing::error!("File change event channel is closed");
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

fn setup_hot_rebuild() -> Result<(
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
async fn start_server(port: u16) -> Result<()> {
    // Initialize tracing
    setup_tracing();

    // Setup file watcher and get the receiver
    let (_watcher, _tx, rx) = setup_hot_refresh()?;
    let (_rebuild_watcher, tx2, mut rx2) = setup_hot_rebuild()?;
    let state = AppState {
        rx,
        latest_build_timestamp: Arc::new(Mutex::new(RebuildEvent {
            build_timestamp: UNIX_EPOCH,
        })),
    };
    let _ = tx2.send(FileChangeEvent {
        paths: vec![PathBuf::from("pages")],
    });

    let latest_build_timestamp = state.latest_build_timestamp.clone();
    tokio::spawn(async move {
        while let Ok(_msg) = rx2.recv().await {
            match sam_website::build_content::build_all() {
                Ok(build_timestamp) => {
                    let mut latest_build_timestamp = latest_build_timestamp.lock().await;
                    std::mem::swap(
                        latest_build_timestamp.deref_mut(),
                        &mut RebuildEvent { build_timestamp },
                    );
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
        .route("/_debug/reload", get(ws_handler))
        .layer(TraceLayer::new_for_http())
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
