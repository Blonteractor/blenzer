use super::enums::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Author {
    #[serde(rename = "node")]
    pub data: AuthorData,
    pub role: AuthorRole,
}

impl ToString for Author {
    fn to_string(&self) -> String {
        let author_role = {
            if let AuthorRole::ArtStory = self.role {
                String::from("Story & Art")
            } else {
                self.role.to_string()
            }
        };
        format!(
            "{}: {} {}",
            author_role,
            self.data.first_name.as_ref().unwrap_or(&String::default()),
            self.data.last_name.as_ref().unwrap_or(&String::default())
        )
    }
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
