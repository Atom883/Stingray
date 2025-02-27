use axum::{extract::{ws::WebSocket, WebSocketUpgrade}, http::{HeaderMap, Response}};

pub async fn ws_handler(ws: WebSocketUpgrade, headers: HeaderMap) -> Response<axum::body::Body> {
    ws.on_upgrade(websocket)
}

async fn websocket(ws: WebSocket) {
    todo!()
}
