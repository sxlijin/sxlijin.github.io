use anyhow::Context;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use serde::Serialize;
use std::{net::SocketAddr, path::PathBuf};
use tokio::{net::TcpListener, sync::broadcast};
use tokio_stream::StreamExt;
// use tower_http::trace::TraceLayer;
use notify::{Event, EventKind, FsEventWatcher, RecursiveMode, Watcher};
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
            Ok(msg) => axum::response::sse::Event::default().json_data(msg),
            Err(e) => {
                tracing::error!("Error in SSE stream: {}", e);
                Err(axum::Error::new(e))
            }
        });

    axum::response::sse::Sse::new(stream)
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
fn setup_file_watcher() -> Result<
    (FsEventWatcher, broadcast::Receiver<FileChangeEvent>),
    Box<dyn std::error::Error + Send + Sync>,
> {
    // Create a broadcast channel for SSE
    let (tx, rx) = broadcast::channel(100);

    // Create a file watcher
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            if let EventKind::Modify(_) = event.kind {
                let paths: Vec<PathBuf> = event.paths;
                if !paths.is_empty() {
                    let message = FileChangeEvent { paths };
                    // If there are no receivers, the message is dropped: that's fine.
                    let _ = tx.send(message);
                }
            }
        }
    })
    .context("Failed to create watcher")?;

    // Add the _site directory to the watcher
    watcher
        .watch(&PathBuf::from("_site"), RecursiveMode::Recursive)
        .context("Failed to watch _site directory")?;

    Ok((watcher, rx))
}

// Create the router with all routes
fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/_debug/reload", get(sse_handler))
        // .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("_site"))
        .with_state(state)
}

// Start the server
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    setup_tracing();

    // Setup file watcher and get the receiver
    let (_watcher, rx) = setup_file_watcher()?;
    let state = AppState { rx };

    // Build the router
    let app = create_router(state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Starting Axum server on {}", addr);

    axum::serve(TcpListener::bind(addr).await?, app.into_make_service()).await?;

    Ok(())
}

// Main function to run the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    start_server(3000).await
}
