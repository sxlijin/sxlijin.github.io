use anyhow::Result;
use async_stream::stream;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebouncedEvent};
use serde::Serialize;
use serde_json;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tower_http::services::ServeDir;

#[derive(Serialize, Clone)]
struct SseEvent {
    paths: Vec<String>,
    kind: String,
}
type NotifyChannel = tokio::sync::broadcast::Sender<SseEvent>;

async fn sse_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let event_stream = stream! {
        let mut ticker = interval(Duration::from_secs(1));
        loop {
            ticker.tick().await;
            yield Ok::<_, Infallible>(format!("data: tick\n\n"));
        }
    };

    let body = Body::wrap_stream(event_stream);
    let mut response = Response::new(body);
    response
        .headers_mut()
        .insert("Content-Type", "text/event-stream".parse().unwrap());
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a broadcast channel for file events
    let (tx, _) = tokio::sync::broadcast::channel::<SseEvent>(100);
    let notify_channel: NotifyChannel = tx;

    // Set up the file watcher
    let notify_channel_for_watcher = notify_channel.clone();
    let mut debouncer = new_debouncer(
        Duration::from_secs(1),
        None,
        move |events: Vec<DebouncedEvent>| {
            for event in events {
                let _ = notify_channel_for_watcher.send(event);
            }
        },
    )?;

    // Start watching the _site directory
    debouncer.watch(&PathBuf::from("_site"), RecursiveMode::Recursive)?;

    // Create the static file service
    let static_service = ServeDir::new("_site");

    // Create the service function
    let make_svc = make_service_fn(move |_conn| {
        let notify_channel = notify_channel.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let notify_channel = notify_channel.clone();
                async move {
                    // Handle SSE endpoint
                    if req.uri().path() == "/_debug/notify" {
                        let mut response = Response::new(Body::empty());
                        response
                            .headers_mut()
                            .insert(CONTENT_TYPE, HeaderValue::from_static("text/event-stream"));
                        response
                            .headers_mut()
                            .insert("Cache-Control", HeaderValue::from_static("no-cache"));
                        response
                            .headers_mut()
                            .insert("Connection", HeaderValue::from_static("keep-alive"));

                        // Create a receiver for this client
                        let mut rx = notify_channel.subscribe();

                        // Create a stream that sends SSE events when files change
                        let stream = async_stream::stream! {
                            while let Ok(event) = rx.recv().await {
                                let event_json = serde_json::to_string(&event).unwrap();
                                yield Ok::<_, Infallible>(Body::from(format!("data: {}\n\n", event_json)));
                            }
                        };

                        *response.body_mut() = Body::wrap_stream(stream);
                        return Ok::<_, Infallible>(response);
                    }

                    static_service.clone().oneshot(req).await
                }
            }))
        }
    });

    // Create the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running at http://{}", addr);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
