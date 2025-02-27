use crate::{
    domain::{
        entities::User,
        repositories::user_repository::UserRepository,
        transaction_manager::{SqlxConn, TransactionContext},
    },
    get_conn,
};

#[derive(Clone, Copy)]
pub struct SqlxUserRepository;

impl UserRepository<SqlxConn> for SqlxUserRepository {
    async fn create(&self, user: User, ctx: TransactionContext<SqlxConn>) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, password_bcrypt, created_at)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&user.id)
        .bind(&user.password_bcrypt)
        .bind(&user.created_at)
        .execute(get_conn!(ctx))
        .await?;

        Ok(())
    }

    async fn get_by_id(
        &self,
        id: &str,
        ctx: TransactionContext<SqlxConn>,
    ) -> anyhow::Result<Option<User>> {
        let row = sqlx::query_as::<_, User>(
            r#"
            SELECT id, password_bcrypt, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(get_conn!(ctx))
        .await?;

        Ok(row)
    }
}
