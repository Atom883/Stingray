use anyhow::{Result, anyhow};
use aws_sdk_dynamodb::{
    Client,
    types::{AttributeAction, AttributeValue, AttributeValueUpdate},
};
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Display;

const KEY: &str = "key";
const VALUE: &str = "value";

#[async_trait::async_trait]
pub trait DynamoDBRepository {
    const TABLE_NAME: &'static str;
    type Key: ToString + Sync + Send + Display;
    type Value: Serialize + DeserializeOwned + Sync + Send;
    fn get_client(&self) -> &Client;

    async fn create(&self, key: &Self::Key, value: &Self::Value) -> Result<()> {
        let client = self.get_client();
        let key = key.to_string();
        let value = serde_json::to_string(value)?;
        client
            .put_item()
            .table_name(Self::TABLE_NAME)
            .item(KEY, AttributeValue::S(key))
            .item(VALUE, AttributeValue::S(value))
            .send()
            .await?;
        Ok(())
    }

    async fn read(&self, key: &Self::Key) -> Result<Self::Value> {
        let client = self.get_client();
        let dynamo_key = key.to_string();
        let res = client
            .get_item()
            .table_name(Self::TABLE_NAME)
            .key(KEY, AttributeValue::S(dynamo_key))
            .send()
            .await?;

        let item = res
            .item
            .ok_or_else(|| anyhow!("There is no such key: {key}"))?;
        let raw_string = item
            .get(VALUE)
            .ok_or_else(|| anyhow!("No such key in this table"))?
            .as_s()
            .map_err(|_| anyhow!("Could not parse"))?;
        Ok(serde_json::from_str::<Self::Value>(raw_string)?)
    }

    async fn update(&self, key: &Self::Key, value: &Self::Value) -> Result<()> {
        let dynamo_value = serde_json::to_string(value)?;
        let client = self.get_client();
        let dynamo_key = key.to_string();
        let attr_val_up = AttributeValueUpdate::builder()
            .action(AttributeAction::Put)
            .value(AttributeValue::S(dynamo_value))
            .build();

        client
            .update_item()
            .table_name(Self::TABLE_NAME)
            .key(KEY, AttributeValue::S(dynamo_key))
            .attribute_updates(VALUE, attr_val_up)
            .send()
            .await?;
        Ok(())
    }

    async fn delete(&self, key: &Self::Key) -> Result<()> {
        let client = self.get_client();
        let dynamo_key = key.to_string();
        client
            .delete_item()
            .table_name(Self::TABLE_NAME)
            .key(KEY, AttributeValue::S(dynamo_key))
            .send()
            .await?;
        Ok(())
    }
}
