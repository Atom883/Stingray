use crate::domain::{entities::User, transaction_manager::TransactionContext};
use anyhow::Result;

pub trait UserRepository<Conn>: Send + Sync + Copy {
    fn create(
        &self,
        user: User,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn get_by_id(
        &self,
        id: &str,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<Option<User>>> + Send;
    // fn update(
    //     &self,
    //     id: &str,
    //     user: User,
    //     ctx: TransactionContext<Conn>,
    // ) -> impl Future<Output = Result<()>> + Send;
    // fn delete(
    //     &self,
    //     id: &str,
    //     ctx: TransactionContext<Conn>,
    // ) -> impl Future<Output = Result<Option<User>>> + Send;
}
