use super::generic_repository::DynamoDBRepository;
use crate::models::Session;
use aws_sdk_dynamodb::Client;

pub struct SessionRepository<'a> {
    client: &'a Client,
}

impl<'a> SessionRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }
}

impl<'a> DynamoDBRepository for SessionRepository<'a> {
    const TABLE_NAME: &'static str = "SessionId";
    type Key = String;
    type Value = Session;
    fn get_client(&self) -> &Client {
        self.client
    }
}
