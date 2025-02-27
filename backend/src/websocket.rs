use crate::domain::entities::{User, UserData};
use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::{HeaderMap, Response},
};
use futures_util::{SinkExt, StreamExt as _};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum WsRequest {
    Match,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum WsResponse {
    Matched { opponent: UserData },
}

pub async fn ws_handler(ws: WebSocketUpgrade, headers: HeaderMap) -> Response<axum::body::Body> {
    ws.on_upgrade(websocket)
}

async fn websocket(ws: WebSocket) {
    let (mut sender, mut receiver) = ws.split();

    while let Some(msg) = receiver.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("websocket error: {e}");
                return;
            }
        };

        let msg: axum::extract::ws::Utf8Bytes = match msg {
            Message::Text(msg) => msg,
            Message::Close(_) => {
                tracing::info!("websocket closed");
                return;
            }
            _ => continue,
        };

        let msg: WsRequest = match serde_json::from_str(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("failed to parse message: {e}");
                return;
            }
        };

        let response = match msg {
            WsRequest::Match => {
                let opponent = UserData::new("dummy".to_string());
                serde_json::to_string(&WsResponse::Matched { opponent }).unwrap()
            }
        };

        // await 10 seconds
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        if let Err(e) = sender.send(Message::Text(response.into())).await {
            tracing::error!("websocket error: {e}");
            return;
        }
    }
}
