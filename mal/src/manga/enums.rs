use serde::Deserialize;

#[derive(Deserialize)]
pub enum AuthorRole {
    Art,
    Story,

    #[serde(rename = "Story & Art")]
    ArtStory,
}
