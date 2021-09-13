use super::enums::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Author {
    #[serde(rename = "node")]
    pub data: AuthorData,
    pub role: AuthorRole,
}

impl std::ops::Deref for Author {
    type Target = AuthorData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Deserialize)]
pub struct AuthorData {
    pub id: usize,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
