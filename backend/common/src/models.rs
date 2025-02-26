use anyhow::ensure;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub password_hash_with_salt: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub user_id: String,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    a_state: AState,
    feeds: HashMap<Feed, i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AState {
    name: String,
    hp: i64,
    max_hp: i64,
    color: Color,
}

#[derive(Serialize, Deserialize)]
pub enum Color {
    Yellow,
    Red,
    Blue,
    Green,
    Black,
    White,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Feed(char);
impl TryInto<Feed> for char {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Feed, Self::Error> {
        ensure!(
            self.is_ascii_alphabetic(),
            "Not an ASCII alphabetic character"
        );
        Ok(Feed(self.to_ascii_uppercase()))
    }
}
