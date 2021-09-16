use super::super::prelude::enums::*;
use super::enums::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Studio {
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Broadcast {
    pub day_of_the_week: DayOfTheWeek,
    pub start_time: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct StartSeason {
    pub year: usize,
    pub season: Season,
}
