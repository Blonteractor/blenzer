use super::enums::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Genre {
    pub id: usize,
    pub name: GenreName,
}

impl ToString for Genre {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
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
