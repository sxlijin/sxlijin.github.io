const DEFAULT_RECONNECT_ATTEMPTS = 3;
const DEFAULT_RECONNECT_INTERVAL_MS = 1000;

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
		const buildTimestamp = document.querySelector(
			'meta[name="build-timestamp"]',
		).content;
		console.log("Build timestamp:", buildTimestamp);

		console.log("WebSocket connection opened");
		ws.send(
			JSON.stringify({
				hello: "world",
			}),
		);
	};

	ws.onmessage = (event) => {
		console.log("WebSocket message received, reloading page");
		// window.location.reload();
	};

	// ws.onclose = () => { };

	ws.onerror = (err) => {
		console.error("WebSocket error:", err);
		if (reconnectAttempts < DEFAULT_RECONNECT_ATTEMPTS) {
			console.log(
				`WebSocket error occurred, attempting to reconnect (attempt ${reconnectAttempts + 1}/${maxReconnectAttempts})...`,
			);
			setTimeout(() => {
				setupWebSocket(reconnectAttempts + 1);
			}, DEFAULT_RECONNECT_INTERVAL_MS);
		} else {
			console.log(
				"Failed to set up hot reload: reached max reconnects for websocket",
			);
		}
		ws.close();
	};
};

// Start WebSocket connection
setupWebSocket(0);
