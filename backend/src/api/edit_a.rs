use crate::{
    StingrayState,
    auth::get_user_id_from_request,
    domain::{
        entities::{AState, Color, UserData},
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
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AEditRequest {
    pub name: String,
    pub color: Color,
    pub font: Option<String>,
    pub is_bold: bool,
    pub is_outlined: bool,
}

pub async fn edit_a<
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
    Json(new_a_state): Json<AEditRequest>,
) -> AppResult<impl IntoResponse> {
    let user_id = get_user_id_from_request(txm.clone(), session_repository, headers).await?;

    let UserData {
        user_id,
        a_state,
        feeds,
    } = txm
        .run(|ctx| user_data_repository.get_by_user_id(&user_id, ctx))
        .await?
        .context("user_data not found")?;

    let a_state = AState {
        name: new_a_state.name,
        color: new_a_state.color,
        font: new_a_state.font,
        is_bold: new_a_state.is_bold,
        is_outlined: new_a_state.is_outlined,
        ..a_state
    };

    txm.run(|ctx| {
        user_data_repository.update(
            UserData {
                user_id,
                a_state,
                feeds,
            },
            ctx,
        )
    })
    .await?;

    Ok(())
}
