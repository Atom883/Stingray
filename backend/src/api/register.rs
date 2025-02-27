use crate::{
    StingrayState,
    domain::{
        entities::{Session, User, UserData},
        repositories::{
            session_repository::SessionRepository, user_data_repository::UserDataRepository,
            user_repository::UserRepository,
        },
        transaction_manager::TransactionManager,
    },
    response::AppResult,
};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::{Json, extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub id: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub session_id: String,
    pub user: UserData,
}

pub async fn register<
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
    let password_bcrypt = bcrypt.hash(req.password, bcrypt::DEFAULT_COST)?;
    let now = now_f();
    let user_id = req.id.clone();

    let session_id = txm
        .with_tx(async move |ctx| {
            user_repository
                .create(
                    User {
                        id: req.id.clone(),
                        password_bcrypt,
                        created_at: now.clone(),
                    },
                    ctx.clone(),
                )
                .await?;

            let session_id = uuid_f();
            session_repository
                .create(
                    Session {
                        id: session_id.clone(),
                        user_id: req.id.clone(),
                        created_at: now.clone(),
                    },
                    ctx,
                )
                .await?;

            Ok(session_id)
        })
        .await?;

    let userdata = UserData::new(user_id);

    Ok(Json({
        RegisterResponse {
            session_id,
            user: userdata,
        }
    }))
}
