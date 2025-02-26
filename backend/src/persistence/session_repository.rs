use crate::{
    domain::{
        entities::Session,
        repositories::session_repository::SessionRepository,
        transaction_manager::{SqlxConn, TransactionContext},
    },
    get_conn,
};

#[derive(Clone, Copy)]
pub struct SqlxSessionRepository;

impl SessionRepository<SqlxConn> for SqlxSessionRepository {
    async fn create(
        &self,
        session: Session,
        ctx: TransactionContext<SqlxConn>,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sessions (id, user_id, created_at)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&session.id)
        .bind(&session.user_id)
        .bind(&session.created_at)
        .execute(get_conn!(ctx))
        .await?;

        Ok(())
    }

    async fn get_by_id(
        &self,
        id: &str,
        ctx: TransactionContext<SqlxConn>,
    ) -> anyhow::Result<Option<crate::domain::entities::Session>> {
        let row = sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, created_at
            FROM sessions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(get_conn!(ctx))
        .await?;

        Ok(row)
    }
}
