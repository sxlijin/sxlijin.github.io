use crate::{BuildSummary, build_website};
use anyhow::{Context, Result};
use axum::{Router, routing::get};
use notify::{Event, FsEventWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{Mutex, broadcast},
};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Debug, Clone, Serialize)]
pub struct FileChangeEvent {
    paths: Vec<PathBuf>,
}
pub struct HotReloadServerState {
    pub rx: broadcast::Receiver<FileChangeEvent>,
    pub latest_build_summary: Arc<Mutex<BuildSummary>>,
}

impl Clone for HotReloadServerState {
    fn clone(&self) -> Self {
        HotReloadServerState {
            rx: self.rx.resubscribe(),
            latest_build_summary: self.latest_build_summary.clone(),
        }
    }
}

// Setup file watcher and return the broadcast receiver
fn setup_hot_refresh() -> Result<(FsEventWatcher, broadcast::Receiver<FileChangeEvent>)> {
    // Create a broadcast channel for SSE
    let (tx, rx) = broadcast::channel(100);
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

    Ok((watcher, rx))
}

fn setup_hot_rebuild() -> Result<(FsEventWatcher, broadcast::Receiver<BuildSummary>)> {
    let (tx, rx) = broadcast::channel(100);
    let mut watcher =
        notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
            Err(e) => {
                tracing::warn!("Failed to watch pages directory: {}", e);
            }
            Ok(_) => match build_website() {
                Ok(build_summary) => {
                    let _ = tx.send(build_summary);
                }
                Err(e) => tracing::error!("Failed to build: {}", e),
            },
        })
        .context("Failed to create watcher")?;

    // TOOD: how to watch multiple directories?
    watcher
        .watch(&PathBuf::from("pages"), RecursiveMode::Recursive)
        .context("Failed to watch pages directory")?;

    Ok((watcher, rx))
}

// Start the server
pub async fn start_server(port: u16) -> Result<()> {
    // Watch for changes in various website source directories and rebuild the website.
    let (_rebuild_watcher, mut hot_rebuild_rx) = setup_hot_rebuild()?;

    // Watch for changes in the website output directory and push that change to the client.
    let (_refresh_watcher, hot_refresh_rx) = setup_hot_refresh()?;

    // Initial website build.
    let build_summary = build_website()?;

    let state = HotReloadServerState {
        rx: hot_refresh_rx,
        latest_build_summary: Arc::new(Mutex::new(build_summary)),
    };

    // Keep the latest build summary in sync - we use this for stale clients to decide if they need to refresh.
    let latest_build_summary = state.latest_build_summary.clone();
    tokio::spawn(async move {
        while let Ok(build_summary) = hot_rebuild_rx.recv().await {
            *latest_build_summary.lock().await = build_summary;
        }
    });

    let app = Router::new()
        // Clients connect here to get notified when they need to refresh.
        .route("/_debug/reload", get(crate::websocket_handler::ws_handler))
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("_site"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Starting Axum server on {}", addr);

    axum::serve(TcpListener::bind(addr).await?, app.into_make_service()).await?;

    Ok(())
}
