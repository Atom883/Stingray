use anyhow::ensure;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub password_bcrypt: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub user_id: String,
    pub a_state: AState,
    pub feeds: FxHashMap<Feed, u64>,
}

impl UserData {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            a_state: AState::default(),
            feeds: FxHashMap::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AState {
    pub name: String,
    pub hp: u64,
    pub max_hp: u64,
    pub color: Color,
    pub font: Option<String>,
    pub is_bold: bool,
    pub is_outlined: bool,
}

impl Default for AState {
    fn default() -> Self {
        Self {
            name: "Stingray".to_string(),
            hp: 100,
            max_hp: 100,
            color: Color::White,
            font: None,
            is_bold: false,
            is_outlined: false,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum Color {
    Yellow,
    Red,
    Blue,
    Green,
    Black,
    #[default]
    White,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
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

impl Feed {
    pub fn is_a(&self) -> bool {
        self.0 == 'A'
    }
}
