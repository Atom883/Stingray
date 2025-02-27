use anyhow::Context;
use axum::{
    Router,
    extract::{ConnectInfo, Request},
    http::{HeaderMap, Method, StatusCode, header::CONTENT_TYPE},
    middleware::{Next, from_fn},
    response::Response,
    routing::{get, post},
};
use bcrypt::BcryptError;
use domain::{
    repositories::{
        session_repository::SessionRepository, user_data_repository::UserDataRepository,
        user_repository::UserRepository,
    },
    transaction_manager::{SqlxTransactionManager, TransactionManager},
};
use persistence::{
    db::create_sqlite_pool, session_repository::SqlxSessionRepository,
    user_data_repository::SqlxUserDataRepository, user_repository::SqlxUserRepository,
};
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tower_http::cors::CorsLayer;
use websocket::ws_handler;

mod api;
mod auth;
mod domain;
mod persistence;
mod response;
mod websocket;

pub trait BCrypt: Send + Sync {
    fn hash(&self, password: String, cost: u32) -> Result<String, BcryptError>;
    fn verify(&self, password: String, hash: String) -> Result<bool, BcryptError>;
}
#[derive(Clone, Copy)]
struct BCryptImpl;
impl BCrypt for BCryptImpl {
    fn hash(&self, password: String, cost: u32) -> Result<String, BcryptError> {
        bcrypt::hash(password, cost)
    }
    fn verify(&self, password: String, hash: String) -> Result<bool, BcryptError> {
        bcrypt::verify(password, &hash)
    }
}

pub struct StingrayState<
    Conn: Send,
    Txm: TransactionManager<Conn>,
    SR: SessionRepository<Conn>,
    UR: UserRepository<Conn>,
    UDR: UserDataRepository<Conn>,
> {
    pub txm: Arc<Txm>,
    pub now_f: Arc<dyn Fn() -> String + Send + Sync>,
    pub uuid_f: Arc<dyn Fn() -> String + Send + Sync>,
    pub bcrypt: Arc<dyn BCrypt>,
    pub session_repository: SR,
    pub user_repository: UR,
    pub user_data_repository: UDR,
    _conn: std::marker::PhantomData<Conn>,
}

impl<
    Conn: Send,
    Txm: TransactionManager<Conn>,
    SR: SessionRepository<Conn>,
    UR: UserRepository<Conn>,
    UDR: UserDataRepository<Conn>,
> Clone for StingrayState<Conn, Txm, SR, UR, UDR>
{
    fn clone(&self) -> Self {
        Self {
            txm: Arc::clone(&self.txm),
            uuid_f: Arc::clone(&self.uuid_f),
            now_f: Arc::clone(&self.now_f),
            bcrypt: Arc::clone(&self.bcrypt),
            session_repository: self.session_repository,
            user_repository: self.user_repository,
            user_data_repository: self.user_data_repository,
            _conn: std::marker::PhantomData,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dir = {
        let mut path = std::env::current_dir().expect("Failed to get current directory.");
        path.push("Stingray");
        if !path.exists() {
            std::fs::create_dir(&path).context("Failed to create data directory.")?;
        }
        path
    };

    let sqlite_pool = create_sqlite_pool(&dir.to_string_lossy()).await?;

    let state = StingrayState {
        txm: Arc::new(SqlxTransactionManager::new(sqlite_pool)),
        uuid_f: Arc::new(|| uuid::Uuid::new_v4().to_string()),
        now_f: Arc::new(|| chrono::Utc::now().to_rfc3339()),
        bcrypt: Arc::new(BCryptImpl),
        session_repository: SqlxSessionRepository,
        user_repository: SqlxUserRepository,
        user_data_repository: SqlxUserDataRepository,
        _conn: std::marker::PhantomData,
    };

    let api_routes = Router::new()
        .route("/register", post(api::register))
        .route("/login", post(api::login))
        .route("/user", get(api::get_user))
        .route("/add_fish", post(api::add_fish))
        .route("/eat_fish", post(api::eat_fish))
        .route("/edit_a", post(api::edit_a))
        .with_state(state.clone());
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest("/api", api_routes)
        .layer(from_fn(service))
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([CONTENT_TYPE]),
        )
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3000))
            .await
            .context("Failed to bind to port.")?;

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("Failed to get local address.")?
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn service(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();

    if let Some(response) = static_file_service(path) {
        return Ok(response);
    }

    Ok(next.run(request).await)
}

fn static_file_service(path: &str) -> Option<Response> {
    static ASSETS_DIR: include_dir::Dir =
        include_dir::include_dir!("$CARGO_MANIFEST_DIR/../frontend/build/client");

    let file_path = if path == "/" {
        "index.html"
    } else {
        &path[1..]
    };

    let file = ASSETS_DIR.get_file(file_path)?;

    let mime = mime_guess::from_path(file.path()).first_or_text_plain();

    Response::builder()
        .header("Content-Type", mime.as_ref())
        .body(axum::body::Body::from(file.contents().to_vec()))
        .inspect_err(|e| tracing::error!("{e}"))
        .ok()
}
