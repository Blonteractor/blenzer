use mal::anime::Anime;
use mal::manga::Manga;
use mal::prelude::Reloadable;
use rand::seq::SliceRandom;
use serenity::builder::{CreateActionRow, CreateEmbed};
use serenity::constants::EMBED_MAX_LENGTH;
use serenity::futures::StreamExt;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use std::time::Duration;

pub const INVALID_INPUT_MSG: &'static str =
    "Invalid input, give an integer greater than 0 and less than the number of results pls.";

pub trait ToDoublePagedEmbed {
    fn double_paged_embed(&self) -> (CreateEmbed, CreateEmbed);
}

impl ToDoublePagedEmbed for Anime {
    fn double_paged_embed(&self) -> (CreateEmbed, CreateEmbed) {
        let synopsis_unshortened = format!(
            "{} \n\n {}",
            self.synopsis.as_ref().unwrap(),
            self.background.as_ref().unwrap()
        );

        let synopsis: String = if synopsis_unshortened.len() > EMBED_MAX_LENGTH {
            String::from("...") + &synopsis_unshortened[0..(EMBED_MAX_LENGTH - 5)]
        } else {
            synopsis_unshortened
        };

        let title = format!(
            "{} `{}`",
            &self.title,
            &self.alternative_titles.as_ref().unwrap().ja
        );

        let page1 = CreateEmbed::default()
            .title(&title)
            .url(&self.url())
            .description(synopsis)
            .image(&self.cover_art.large)
            .color(Color::from_rgb(4, 105, 207))
            .to_owned();

        let page2 = CreateEmbed::default()
            .title(&title)
            .url(&self.url())
            .image(
                &self
                    .pictures
                    .as_ref()
                    .unwrap()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .large,
            )
            .color(Color::from_rgb(4, 105, 207))
            .field("Score", format!("`{}`", self.score.unwrap_or(0.0)), true)
            .field("Rank", format!("`{}`", self.rank.unwrap_or(0)), true)
            .field(
                "Popularity",
                format!("`{}`", self.popularity.unwrap_or(0)),
                true,
            )
            .field(
                "Number of Episodes",
                format!("`{}`", self.episodes.unwrap_or(0)),
                true,
            )
            .field(
                "Season",
                {
                    if let Some(ref season) = self.start_season {
                        format!("`{} {}`", season.season, season.year)
                    } else {
                        String::from("NA")
                    }
                },
                true,
            )
            .field(
                "Broadcast",
                {
                    if let Some(ref broadcast) = self.broadcast {
                        format!("`{} {}`", broadcast.day_of_the_week, broadcast.start_time)
                    } else {
                        String::from("`NA`")
                    }
                },
                true,
            )
            .field(
                "Studio(s)",
                {
                    if let Some(genres) = &self.studios {
                        cleanly_join_vec(genres)
                    } else {
                        String::from("NA")
                    }
                },
                true,
            )
            .field(
                "Age Rating",
                format!(
                    "`{}`",
                    self.rating
                        .as_ref()
                        .unwrap_or(&mal::prelude::enums::Rating::NA)
                ),
                true,
            )
            .field(
                "Release",
                format!("`{}`", self.start.as_ref().unwrap_or(&String::from("NA"))),
                true,
            )
            .field(
                "End",
                format!("`{}`", self.end.as_ref().unwrap_or(&String::from("NA"))),
                true,
            )
            .field(
                "Status",
                format!(
                    "`{}`",
                    self.status
                        .as_ref()
                        .unwrap_or(&mal::prelude::enums::Status::NA)
                ),
                true,
            )
            .field(
                "Genres",
                {
                    if let Some(genres) = &self.genres {
                        cleanly_join_vec(genres)
                    } else {
                        String::from("NA")
                    }
                },
                true,
            )
            .to_owned();

        (page1, page2)
    }
}

impl ToDoublePagedEmbed for Manga {
    fn double_paged_embed(&self) -> (CreateEmbed, CreateEmbed) {
        let synopsis_unshortened = format!(
            "{} \n\n {}",
            self.synopsis.as_ref().unwrap(),
            self.background.as_ref().unwrap()
        );

        let synopsis: String = if synopsis_unshortened.len() > EMBED_MAX_LENGTH {
            String::from("...") + &synopsis_unshortened[0..(EMBED_MAX_LENGTH - 5)]
        } else {
            synopsis_unshortened
        };

        let title = format!(
            "{} `{}`",
            &self.title,
            &self.alternative_titles.as_ref().unwrap().ja
        );

        let page1 = CreateEmbed::default()
            .title(&title)
            .url(&self.url())
            .description(synopsis)
            .image(&self.cover_art.large)
            .color(Color::from_rgb(4, 105, 207))
            .to_owned();

        let page2 = CreateEmbed::default()
            .title(&title)
            .url(&self.url())
            .image(
                &self
                    .pictures
                    .as_ref()
                    .unwrap()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .large,
            )
            .field("Score", format!("`{}`", self.score.unwrap_or(0.0)), true)
            .field("Rank", format!("`{}`", self.rank.unwrap_or(0)), true)
            .field(
                "Popularity",
                format!("`{}`", self.popularity.unwrap_or(0)),
                true,
            )
            .field(
                "Number of Chapters",
                format!("`{}`", self.chapters.unwrap_or(0)),
                true,
            )
            .field(
                "Number of Volumes",
                format!("`{}`", self.volumes.unwrap_or(0)),
                true,
            )
            .field(
                "Author(s)",
                {
                    if let Some(authors) = &self.authors {
                        cleanly_join_vec(authors)
                    } else {
                        String::from("NA")
                    }
                },
                true,
            )
            .field(
                "Age Rating",
                format!(
                    "`{}`",
                    self.rating
                        .as_ref()
                        .unwrap_or(&mal::prelude::enums::Rating::NA)
                ),
                true,
            )
            .field(
                "Release",
                format!("`{}`", self.start.as_ref().unwrap_or(&String::from("NA"))),
                true,
            )
            .field(
                "End",
                format!("`{}`", self.end.as_ref().unwrap_or(&String::from("NA"))),
                true,
            )
            .field(
                "Status",
                format!(
                    "`{}`",
                    self.status
                        .as_ref()
                        .unwrap_or(&mal::prelude::enums::Status::NA)
                ),
                true,
            )
            .field(
                "Genres",
                {
                    if let Some(genres) = &self.genres {
                        cleanly_join_vec(genres)
                    } else {
                        String::from("NA")
                    }
                },
                true,
            )
            .to_owned();

        (page1, page2)
    }
}

pub fn create_actionrow_for_pagination<S>(action_1_name: S, action_2_name: S) -> CreateActionRow
where
    S: ToString,
{
    CreateActionRow::default()
        .create_button(|b| {
            b.style(ButtonStyle::Primary)
                .label("Previous")
                .custom_id("prev")
        })
        .create_button(|b| {
            b.style(ButtonStyle::Secondary)
                .label(action_1_name)
                .custom_id("action_1")
        })
        .create_button(|b| {
            b.style(ButtonStyle::Secondary)
                .label(action_2_name)
                .custom_id("action_2")
        })
        .create_button(|b| {
            b.style(ButtonStyle::Primary)
                .label("Next")
                .custom_id("next")
        })
        .to_owned()
}

pub fn cleanly_join_vec(to_join: &Vec<impl ToString>) -> String {
    to_join
        .iter()
        .map(|g| format!("`{}`", &g.to_string()))
        .collect::<Vec<String>>()
        .join(" | ")
}

trait Action {}

pub async fn handle_component_interactions_for_pagination<F, A, E>(
    msg: &mut Message,
    ctx: &Context,
    mut current: usize,
    to_paginate_on: &mut Vec<E>,
    on_action_1: F,
    on_action_2: A,
) -> Result<(), SerenityError>
where
    F: FnOnce(&E) -> Result<CreateEmbed, SerenityError> + Copy,
    A: FnOnce(&E) -> Result<CreateEmbed, SerenityError> + Copy,
    E: ToDoublePagedEmbed + Reloadable,
{
    let mut current_object = unsafe { to_paginate_on.get_unchecked_mut(current - 1) };

    let mut cached = Vec::new();
    cached.push(current);

    let mut interaction_stream = msg
        .await_component_interactions(ctx)
        .timeout(Duration::new(30, 0))
        .await;

    while let Some(interaction) = interaction_stream.next().await {
        if interaction.data.custom_id == "action_1" {
            msg.edit(ctx, |m| {
                m.set_embed(on_action_1(&current_object).unwrap_or_default().clone())
            })
            .await?;
        } else if interaction.data.custom_id == "action_2" {
            msg.edit(ctx, |m| {
                m.set_embed(on_action_2(&current_object).unwrap_or_default().clone())
            })
            .await?;
        } else {
            if interaction.data.custom_id == "next" {
                current += 1;
                if current > to_paginate_on.len() {
                    current = 1;
                }
            } else if interaction.data.custom_id == "prev" {
                current -= 1;
                if current < 1 {
                    current = to_paginate_on.len();
                }
            }

            //SAFETY: Already checked bounds manually kekw ratio
            current_object = unsafe { to_paginate_on.get_unchecked_mut(current - 1) };

            if !cached.contains(&current) {
                current_object.reload().await;
                cached.push(current);
            }

            let embeds = current_object.double_paged_embed();

            msg.edit(ctx, |m| m.set_embed(embeds.0.clone())).await?;
        }

        interaction.defer(ctx).await?;
    }

    Ok(())
}
