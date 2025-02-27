use sqlx::{Sqlite, SqlitePool, Transaction, pool::PoolConnection};
use std::{pin::Pin, sync::Arc};
use tokio::sync::{Mutex, MutexGuard};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait TransactionManager<Conn>: Send + Sync {
    fn run<'a, R: Send + 'a, F: Future<Output = anyhow::Result<R>> + Send + 'a>(
        &'a self,
        f: impl FnOnce(TransactionContext<Conn>) -> F + Send + 'a,
    ) -> BoxFuture<'a, anyhow::Result<R>>;
    fn with_tx<'a, R: Send + 'a, F: Future<Output = anyhow::Result<R>> + Send + 'a>(
        &'a self,
        f: impl FnOnce(TransactionContext<Conn>) -> F + Send + 'a,
    ) -> BoxFuture<'a, anyhow::Result<R>>;
}

pub struct TransactionContext<Conn>(Arc<Mutex<Conn>>);

impl<Conn> Clone for TransactionContext<Conn> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl TransactionContext<SqlxConn> {
    /// Don't call this method directly. Use the `get_conn!` or `bind_conn!` macros instead.
    pub async fn conn(&self) -> MutexGuard<'_, SqlxConn> {
        self.0.lock().await
    }
}

#[macro_export]
macro_rules! get_conn {
    ($tx_ctx:expr) => {
        match *$tx_ctx.conn().await {
            SqlxConn::Tx(ref mut tx) => &mut **tx,
            SqlxConn::Pool(ref mut pool) => &mut **pool,
        }
    };
}

#[macro_export]
macro_rules! bind_conn {
    ($tx_ctx:expr, $conn_name:ident) => {
        let mut __guard = $tx_ctx.conn().await;
        let $conn_name = match *__guard {
            SqlxConn::Tx(ref mut tx) => &mut **tx,
            SqlxConn::Pool(ref mut pool) => &mut **pool,
        };
    };
}

#[derive(Debug)]
pub enum SqlxConn {
    Tx(Transaction<'static, Sqlite>),
    Pool(PoolConnection<Sqlite>),
}

pub struct SqlxTransactionManager(SqlitePool);

impl SqlxTransactionManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self(pool)
    }
}

impl TransactionManager<SqlxConn> for SqlxTransactionManager {
    fn run<'a, R: Send + 'a, F: Future<Output = anyhow::Result<R>> + Send + 'a>(
        &'a self,
        f: impl FnOnce(TransactionContext<SqlxConn>) -> F + Send + 'a,
    ) -> BoxFuture<'a, anyhow::Result<R>> {
        Box::pin(async move {
            let conn = self.0.acquire().await?;
            let tx_ctx = TransactionContext(Arc::new(Mutex::new(SqlxConn::Pool(conn))));
            f(tx_ctx).await
        })
    }

    fn with_tx<'a, R: Send + 'a, F: Future<Output = anyhow::Result<R>> + Send + 'a>(
        &'a self,
        f: impl FnOnce(TransactionContext<SqlxConn>) -> F + Send + 'a,
    ) -> BoxFuture<'a, anyhow::Result<R>> {
        Box::pin(async move {
            let tx = Arc::new(Mutex::new(SqlxConn::Tx(self.0.begin().await?)));
            let result = f(TransactionContext(tx.clone())).await?;
            // ensure that the arc ref count is 1
            let SqlxConn::Tx(tx) = Arc::try_unwrap(tx).unwrap().into_inner() else {
                panic!(
                    "The TransactionContext value was incorrectly manipulated in the with_tx closure."
                );
            };
            tx.commit().await?;
            Ok(result)
        })
    }
}
