use crate::domain::{entities::Session, transaction_manager::TransactionContext};
use anyhow::Result;

pub trait SessionRepository<Conn>: Send + Sync + Copy {
    fn create(
        &self,
        session: Session,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn get_by_id(
        &self,
        id: &str,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<Option<Session>>> + Send;
    // fn update(
    //     &self,
    //     id: &str,
    //     session: Session,
    //     ctx: TransactionContext<Conn>,
    // ) -> impl Future<Output = Result<()>> + Send;
    // fn delete(
    //     &self,
    //     id: &str,
    //     ctx: TransactionContext<Conn>,
    // ) -> impl Future<Output = Result<Option<Session>>> + Send;
}
