use crate::{BuildSummary, build_website};
use anyhow::{Context, Result};
use axum::{Router, routing::get};
use notify_debouncer_full::{
    DebounceEventResult, Debouncer, RecommendedCache, new_debouncer,
    notify::{RecommendedWatcher, RecursiveMode},
};
use serde::Serialize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use tokio::{
    net::TcpListener,
    sync::{Mutex, broadcast},
};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::build_website::BuildMode;

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
fn setup_hot_refresh() -> Result<(
    Debouncer<RecommendedWatcher, RecommendedCache>,
    broadcast::Receiver<FileChangeEvent>,
)> {
    // Create a broadcast channel for SSE
    let (tx, rx) = broadcast::channel(100);

    // Create a debounced file watcher
    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        None,
        move |result: DebounceEventResult| {
            match result {
                Ok(events) => {
                    if !events.is_empty() {
                        let paths = events
                            .into_iter()
                            .map(|e| e.event.paths)
                            .flatten()
                            .collect();
                        let message = FileChangeEvent { paths };
                        // If there are no receivers, the message is dropped: that's fine.
                        let _ = tx.send(message);
                    }
                }
                Err(e) => tracing::warn!("Failed to watch directory: {:?}", e),
            }
        },
    )
    .context("Failed to create debouncer")?;

    // Add the _site directory to the watcher
    debouncer
        .watch(&PathBuf::from("_site"), RecursiveMode::Recursive)
        .context("Failed to watch _site directory")?;

    Ok((debouncer, rx))
}

fn setup_hot_rebuild() -> Result<(
    Debouncer<RecommendedWatcher, RecommendedCache>,
    broadcast::Receiver<BuildSummary>,
)> {
    let (tx, rx) = broadcast::channel(100);

    let mut debouncer = new_debouncer(
        Duration::from_millis(250),
        None,
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                if !events.is_empty() {
                    match build_website(BuildMode::Dev) {
                        Ok(build_summary) => {
                            let _ = tx.send(build_summary);
                        }
                        Err(e) => tracing::error!("Failed to build: {}", e),
                    }
                }
            }
            Err(e) => tracing::warn!("Failed to watch directory: {:?}", e),
        },
    )
    .context("Failed to create debouncer")?;

    // Watch multiple directories
    const WEBSITE_SRC_DIRS: [&str; 6] = [
        "_layouts",
        "assets",
        "pages",
        "posts",
        "scss",
        "root-assets",
    ];

    for dir in WEBSITE_SRC_DIRS {
        let path = PathBuf::from(dir);
        if path.exists() {
            debouncer
                .watch(&path, RecursiveMode::Recursive)
                .with_context(|| format!("Failed to watch {} directory", dir))?;
            tracing::info!("Watching directory: {}", dir);
        } else {
            tracing::warn!("Directory {} does not exist, skipping", dir);
        }
    }

    Ok((debouncer, rx))
}

// Start the server
pub async fn start_server(port: u16) -> Result<()> {
    // Watch for changes in various website source directories and rebuild the website.
    let (_rebuild_watcher, mut hot_rebuild_rx) = setup_hot_rebuild()?;

    // Watch for changes in the website output directory and push that change to the client.
    let (_refresh_watcher, hot_refresh_rx) = setup_hot_refresh()?;

    // Initial website build.
    let build_summary = build_website(BuildMode::Dev)?;

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
        .fallback_service(ServeDir::new("_site"))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Starting Axum server on {}", addr);

    axum::serve(TcpListener::bind(addr).await?, app.into_make_service()).await?;

    Ok(())
}
