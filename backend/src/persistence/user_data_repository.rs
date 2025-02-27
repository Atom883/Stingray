use fxhash::FxHashMap;
use sqlx::FromRow;

use crate::{
    domain::{
        entities::{AState, Feed, UserData},
        repositories::user_data_repository::UserDataRepository,
        transaction_manager::{SqlxConn, TransactionContext},
    },
    get_conn,
};

#[derive(Clone, Copy)]
pub struct SqlxUserDataRepository;

impl UserDataRepository<SqlxConn> for SqlxUserDataRepository {
    async fn create(
        &self,
        user_data: UserData,
        ctx: TransactionContext<SqlxConn>,
    ) -> anyhow::Result<()> {
        let user_data_: UserData_ = user_data.into();
        sqlx::query(
            r#"
            INSERT INTO user_data (user_id, a_state, feeds)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&user_data_.user_id)
        .bind(&user_data_.a_state)
        .bind(&user_data_.feeds)
        .execute(get_conn!(ctx))
        .await?;

        Ok(())
    }

    async fn get_by_id(
        &self,
        id: &str,
        ctx: TransactionContext<SqlxConn>,
    ) -> anyhow::Result<Option<UserData>> {
        let row = sqlx::query_as::<_, UserData_>(
            r#"
            SELECT user_id, a_state, feeds
            FROM user_data
            WHERE user_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(get_conn!(ctx))
        .await?;

        Ok(row.map(|user_data_| user_data_.into()))
    }

    async fn get_by_user_id(
        &self,
        user_id: &str,
        ctx: TransactionContext<SqlxConn>,
    ) -> anyhow::Result<Option<UserData>> {
        let row = sqlx::query_as::<_, UserData_>(
            r#"
                SELECT user_id, a_state, feeds
                FROM user_data
                WHERE user_id = $1
                "#,
        )
        .bind(user_id)
        .fetch_optional(get_conn!(ctx))
        .await?;

        Ok(row.map(|user_data_| user_data_.into()))
    }

    async fn update(
        &self,
        user_data: UserData,
        ctx: TransactionContext<SqlxConn>,
    ) -> anyhow::Result<()> {
        let user_data_: UserData_ = user_data.into();
        sqlx::query(
            r#"
                UPDATE user_data
                SET a_state = $2, feeds = $3
                WHERE user_id = $1
                "#,
        )
        .bind(&user_data_.user_id)
        .bind(&user_data_.a_state)
        .bind(&user_data_.feeds)
        .execute(get_conn!(ctx))
        .await?;

        Ok(())
    }
}

#[derive(FromRow)]
struct UserData_ {
    user_id: String,
    a_state: sqlx::types::Json<AState>,
    feeds: sqlx::types::Json<FxHashMap<Feed, u64>>,
}

impl From<UserData_> for UserData {
    fn from(user_data_: UserData_) -> UserData {
        UserData {
            user_id: user_data_.user_id,
            a_state: user_data_.a_state.0,
            feeds: user_data_.feeds.0,
        }
    }
}

impl From<UserData> for UserData_ {
    fn from(user_data: UserData) -> UserData_ {
        UserData_ {
            user_id: user_data.user_id,
            a_state: sqlx::types::Json(user_data.a_state),
            feeds: sqlx::types::Json(user_data.feeds),
        }
    }
}
