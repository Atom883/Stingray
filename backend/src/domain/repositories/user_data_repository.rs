use crate::domain::{entities::UserData, transaction_manager::TransactionContext};
use anyhow::Result;

pub trait UserDataRepository<Conn>: Send + Sync + Copy {
    fn create(
        &self,
        user_data: UserData,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<()>> + Send;
    fn get_by_id(
        &self,
        id: &str,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<Option<UserData>>> + Send;
    fn get_by_user_id(
        &self,
        user_id: &str,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<Option<UserData>>> + Send;
    fn update(
        &self,
        id: &str,
        user_data: UserData,
        ctx: TransactionContext<Conn>,
    ) -> impl Future<Output = Result<()>> + Send;
    // fn delete(
    //     &self,
    //     id: &str,
    //     ctx: TransactionContext<Conn>,
    // ) -> impl Future<Output = Result<Option<UserData>>> + Send;
}
