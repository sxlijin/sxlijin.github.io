use crate::hot_reload::HotReloadServerState;
use crate::EmbeddedBuildTimestamp;
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio::select;

#[derive(Clone, Debug, Serialize)]
struct RebuildEvent {
    build_timestamp: SystemTime,
}

pub async fn ws_handler(
    State(state): State<HotReloadServerState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|ws| async move { ws_handler_impl(state, ws).await })
}

#[derive(Debug, Deserialize)]
#[serde(tag = "request", rename_all = "snake_case")]
enum WebsocketRequest {
    GetBuildStaleness {
        build_timestamp: EmbeddedBuildTimestamp,
    },
}

async fn ws_handler_impl(mut state: HotReloadServerState, mut ws: WebSocket) {
    tracing::info!("WebSocket connection established");

    loop {
        select! {
            Some(Ok(axum::extract::ws::Message::Text(ws_req))) = ws.recv() => {
                tracing::info!("Received websocket request: {}", ws_req);
                let Ok(req) = serde_json::from_str::<WebsocketRequest>(&ws_req) else {
                    tracing::warn!("failed to deser build staleness recv");
                    continue;
                };
                match req {
                    WebsocketRequest::GetBuildStaleness {
                        build_timestamp
                    } => {
                        let latest_build_timestamp = {
                            state.latest_build_summary.lock().await.build_timestamp
                        };
                        if build_timestamp.0 < latest_build_timestamp {
                            if let Err(e) = ws
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&RebuildEvent { build_timestamp: latest_build_timestamp })
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
