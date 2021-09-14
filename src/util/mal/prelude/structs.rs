use super::enums::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Genre {
    pub id: usize,
    pub name: GenreName,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Picture {
    pub medium: String,
    pub large: String,
}

#[derive(Deserialize)]
pub struct AlternativeTitles {
    pub synonyms: Vec<String>,
    pub en: String,
    pub ja: String,
}
