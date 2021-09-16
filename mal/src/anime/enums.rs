use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Source {
    Original,
    Manga,
    Game,
    LightNovel,
    VisualNovel,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}
