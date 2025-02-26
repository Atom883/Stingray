use super::generic_repository::DynamoDBRepository;
use crate::models::UserData;
use aws_sdk_dynamodb::Client;

pub struct UserDataRepository<'a> {
    client: &'a Client,
}

impl<'a> UserDataRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }
}

impl<'a> DynamoDBRepository for UserDataRepository<'a> {
    const TABLE_NAME: &'static str = "UserData";
    type Key = String;
    type Value = UserData;
    fn get_client(&self) -> &Client {
        self.client
    }
}
