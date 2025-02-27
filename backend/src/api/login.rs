use super::register::RegisterRequest;
use crate::{
    StingrayState,
    domain::{
        entities::Session,
        repositories::{
            session_repository::SessionRepository, user_data_repository::UserDataRepository,
            user_repository::UserRepository,
        },
        transaction_manager::TransactionManager,
    },
    response::AppResult,
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue, header},
    response::IntoResponse,
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub session_id: String,
}

pub async fn login<
    Conn: Send,
    Txm: TransactionManager<Conn>,
    SR: SessionRepository<Conn>,
    UR: UserRepository<Conn>,
    UDR: UserDataRepository<Conn>,
>(
    State(StingrayState {
        txm,
        now_f,
        uuid_f,
        bcrypt,
        session_repository,
        user_repository,
        ..
    }): State<StingrayState<Conn, Txm, SR, UR, UDR>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    let user = txm
        .run(|ctx| user_repository.get_by_id(&req.id, ctx))
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    if bcrypt.verify(req.password, user.password_bcrypt)? {
        let session_id = uuid_f();
        txm.run(|ctx| {
            session_repository.create(
                Session {
                    id: session_id.clone(),
                    user_id: req.id.clone(),
                    created_at: now_f(),
                },
                ctx,
            )
        })
        .await?;

        Ok(Json(LoginResponse { session_id }))
    } else {
        Err(anyhow::anyhow!("password is incorrect").into())
    }
}
