const DEFAULT_RECONNECT_ATTEMPTS = 100;
const DEFAULT_RECONNECT_INTERVAL_MS = 1000;

const maybeReloadWebsocket = (reconnectAttempts) => {
	if (reconnectAttempts < DEFAULT_RECONNECT_ATTEMPTS) {
		console.log(
			`WebSocket error occurred, attempting to reconnect (attempt ${reconnectAttempts + 1}/${DEFAULT_RECONNECT_ATTEMPTS})...`,
		);
		setTimeout(() => {
			setupWebSocket(reconnectAttempts + 1);
		}, DEFAULT_RECONNECT_INTERVAL_MS);
	} else {
		console.log(
			"Failed to set up hot refresh: reached max reconnects for websocket",
		);
	}
};

const setupWebSocket = (reconnectAttempts) => {
	console.log("Setting up WebSocket connection");

	// Only create a new connection if there isn't an active one
	if (
		window.hotReloadWs &&
		(window.hotReloadWs.readyState === WebSocket.CONNECTING ||
			window.hotReloadWs.readyState === WebSocket.OPEN)
	) {
		console.log("Active WebSocket connection exists, reusing it");
		return;
	}

	const ws = new WebSocket(`ws://${window.location.host}/_debug/reload`);
	window.hotReloadWs = ws;

	ws.onopen = () => {
		// reset the connection counter when we successfully re-establish a connection
		reconnectAttempts = 0;
		const build_summary = document.querySelector(
			'meta[name="build-summary"]',
		).content;
		ws.send(
			JSON.stringify({
				request: "get_build_staleness",
				build_timestamp,
			}),
		);
	};

	ws.onmessage = (event) => {
		console.log("WebSocket message received, reloading page", event);
		window.location.reload();
	};

	ws.onclose = () => {
		console.log("Websocket was closed");
		maybeReloadWebsocket(reconnectAttempts);
	};

	ws.onerror = (err) => {
		console.error("WebSocket error:", err);
		maybeReloadWebsocket(reconnectAttempts);
	};
};

// Start WebSocket connection
setupWebSocket(0);
