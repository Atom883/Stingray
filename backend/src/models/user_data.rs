
use sqlx::FromRow;
use sqlx::types::Json;
use std::collections::HashMap;
use crate::models::{AState, Feed};

#[derive(FromRow)]
pub struct UserData {
    pub user_id: String,
    pub a_state: Json<AState>,
    pub feeds: Json<HashMap<Feed, u64>>,
}
