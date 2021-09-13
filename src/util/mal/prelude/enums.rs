use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Rating {
    #[serde(rename = "pg_13")]
    PG13,
    #[serde(rename = "r")]
    R,
    #[serde(rename = "r+")]
    RPLUS,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "pg")]
    PG,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum DayOfTheWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Deserialize)]
pub enum GenreName {
    Action,
    Adventure,
    Comedy,
    Drama,
    #[serde(rename = "Slice Of Life")]
    SliceOfLife,
    Fantasy,
    Magic,
    Supernatural,
    Horror,
    Mystery,
    Psychological,
    Romance,
    #[serde(rename = "Sci-Fi")]
    SciFi,
    Cypberpunk,
    Game,
    Ecchi,
    Demons,
    Military,
    Harem,
    Josei,
    #[serde(rename = "Martial Arts")]
    MartialArts,
    Kids,
    Hisorical,
    Hentai,
    Isekai,
    Millitary,
    Mecha,
    Music,
    Parody,
    Police,
    #[serde(rename = "Post Apocalyptic")]
    PostApocalyptic,
    #[serde(rename = "Reverse Harem")]
    ReverseHarem,
    School,
    Seinen,
    Shoujou,
    Samurai,
    #[serde(rename = "Shoujou Ai")]
    ShoujoAi,
    Shounen,
    #[serde(rename = "Shounen Ai")]
    ShounenAi,
    Space,
    Sports,
    #[serde(rename = "Super Power")]
    SuperPower,
    Tragedy,
    Vampire,
    Yuri,
    Yaoi,
    Thriller,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Status {
    FinishedAiring,
    Airing,
    CurrentlyPublishing,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum NSFWLevel {
    White,
    Black,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum MediaType {
    TV,
    ONA,
    OVA,
    Movie,
    Special,
    Manga,
}