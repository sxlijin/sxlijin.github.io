use axum::{extract::State, response::IntoResponse, routing::get, Router};
use serde::Serialize;
use std::{net::SocketAddr, path::PathBuf, time::Duration};
use tokio::{net::TcpListener, sync::broadcast};
use tokio_stream::StreamExt;
// use tower_http::trace::TraceLayer;
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

// Start the server
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a broadcast channel for SSE
    let (tx, rx) = broadcast::channel(100);
    let state = AppState { rx };

    // Clone the sender for the background task
    let tx_clone = tx;

    // Spawn a background task to send notifications every second
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            let message = FileChangeEvent {
                paths: vec![PathBuf::from("test.txt")],
            };

            // Ignore errors if there are no receivers
            let _ = tx_clone.send(message);
        }
    });

    // Build the router
    let app = Router::new()
        .route("/_debug/reload", get(sse_handler))
        // .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Starting Axum server on {}", addr);

    axum::serve(TcpListener::bind(addr).await?, app.into_make_service()).await?;

    Ok(())
}

// Main function to run the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server(3000).await
}
