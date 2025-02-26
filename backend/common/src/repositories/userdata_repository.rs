use super::generic_repository::DynamoDBRepository;
use crate::models::User;
use aws_sdk_dynamodb::Client;

pub struct UserRepository<'a> {
    client: &'a Client,
}

impl<'a> UserRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }
}

impl<'a> DynamoDBRepository for UserRepository<'a> {
    const TABLE_NAME: &'static str = "User";
    type Key = String;
    type Value = User;
    fn get_client(&self) -> &Client {
        self.client
    }
}
