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
    #[serde(rename = "Slice of Life")]
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
    #[serde(rename = "Boys Love")]
    BoysLove,
    Erotica,
    Harem,
    Josei,
    #[serde(rename = "Martial Arts")]
    MartialArts,
    #[serde(rename = "Avant Garde")]
    AvantGarde,
    Kids,
    Historical,
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
    Suspense,
    #[serde(rename = "Gender Bender")]
    GenderBender,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Status {
    FinishedAiring,
    CurrentlyAiring,
    CurrentlyPublishing,
    Finished,
    OnHiatus,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum NSFWLevel {
    White,
    Black,
    Gray,
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
    Manhwa,

    #[serde(rename = "light_novel")]
    LightNovel,
}
