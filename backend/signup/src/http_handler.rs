use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::Client;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use common::{
    models::{Session, User},
    repositories::{
        generic_repository::DynamoDBRepository as _, session_repository::SessionRepository,
        userdata_repository::UserRepository,
    },
};
use lambda_http::{Body, Error, Request, Response};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Req {
    id: String,
    password: String,
}

pub(crate) async fn signup(event: Request) -> Result<Response<Body>, Error> {
    return Ok(Response::builder()
        .status(200)
        .body(Body::from("Hello, World!"))
        .unwrap());
    let conf = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let client = if env::var("AWS_SAM_LOCAL").is_ok() {
        Client::from_conf(
            aws_sdk_dynamodb::Config::builder()
                .endpoint_url("http://localhost:8000")
                .region(Region::from_static("ap-northeast-1"))
                .credentials_provider(conf.credentials_provider().unwrap())
                .build(),
        )
    } else {
        Client::new(&conf)
    };

    let user_repository = UserRepository::new(&client);
    let session_repository = SessionRepository::new(&client);

    let req: Req = serde_json::from_slice(event.body().as_ref())?;

    let password_hash = hash(&req.password, DEFAULT_COST)?;
    let now = Utc::now().to_rfc3339();

    user_repository
        .create(
            &req.id,
            &User {
                password_hash_with_salt: password_hash,
                created_at: now.clone(),
            },
        )
        .await?;

    let session_id = uuid::Uuid::new_v4().to_string();
    let session = Session {
        user_id: req.id,
        created_at: now.clone(),
        expires_at: now,
    };
    session_repository.create(&session_id, &session).await?;

    let response = Response::builder()
        .status(200)
        .header("Set-Cookie", format!("session_id={session_id}; HttpOnly"))
        .body(Body::from("Signup successful"))?;

    Ok(response)
}
