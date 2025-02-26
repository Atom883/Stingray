use anyhow::ensure;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub password_bcrypt: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub user_id: String,
    pub a_state: AState,
    pub feeds: FxHashMap<Feed, i64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AState {
    pub name: String,
    pub hp: i64,
    pub max_hp: i64,
    pub color: Color,
    pub font_family: Option<String>,
    pub is_bold: bool,
    pub is_outlined: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub enum Color {
    Yellow,
    Red,
    Blue,
    Green,
    Black,
    #[default]
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
