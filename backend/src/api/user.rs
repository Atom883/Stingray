use crate::{
    StingrayState,
    auth::get_user_id_from_request,
    domain::{
        repositories::{
            session_repository::SessionRepository, user_data_repository::UserDataRepository,
            user_repository::UserRepository,
        },
        transaction_manager::TransactionManager,
    },
    response::AppResult,
};
use anyhow::Context;
use axum::{Json, extract::State, http::HeaderMap, response::IntoResponse};

pub async fn get_user<
    Conn: Send,
    Txm: TransactionManager<Conn>,
    SR: SessionRepository<Conn>,
    UR: UserRepository<Conn>,
    UDR: UserDataRepository<Conn>,
>(
    State(StingrayState {
        txm,
        session_repository,
        user_data_repository,
        ..
    }): State<StingrayState<Conn, Txm, SR, UR, UDR>>,
    headers: HeaderMap,
) -> AppResult<impl IntoResponse> {
    let user_id = get_user_id_from_request(txm.clone(), session_repository, headers).await?;

    let user_data = txm
        .run(|ctx| user_data_repository.get_by_user_id(&user_id, ctx))
        .await?
        .context("user_data not found")?;

    Ok(Json(user_data))
}
