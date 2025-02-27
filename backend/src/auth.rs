use crate::domain::{
    repositories::session_repository::SessionRepository, transaction_manager::TransactionManager,
};
use anyhow::Context as _;
use axum::http::HeaderMap;
use std::sync::Arc;

pub async fn get_user_id_from_request<
    Conn: Send,
    Txm: TransactionManager<Conn>,
    SR: SessionRepository<Conn>,
>(
    txm: Arc<Txm>,
    session_repository: SR,
    headers: HeaderMap,
) -> anyhow::Result<String> {
    tracing::info!("headers: {:?}", headers);
    let session_id = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|cookie| {
            cookie
                .split(';')
                .find(|cookie| cookie.starts_with("session_id="))
                .map(|cookie| cookie.trim_start_matches("session_id="))
        })
        .context("Missing Authorization")?;

    let session = txm
        .run(|ctx| session_repository.get_by_id(session_id, ctx))
        .await?
        .context("Missing Authorization")?;
    Ok(session.user_id)
}
