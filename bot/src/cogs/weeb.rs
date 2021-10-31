use rand::seq::SliceRandom;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    futures::StreamExt,
};
use std::time::Duration;

use mal::anime::Anime;
use serenity::model::interactions::message_component::ButtonStyle;

#[command]
async fn anime(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    let mut search_results = Anime::search_basic(query, 10, true).await?;

    let mut sent_choices_message = msg
        .reply(
            ctx,
            search_results
                .iter()
                .enumerate()
                .map(|anime| format!("{}. **{}**", anime.0 + 1, &anime.1.title))
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .await?;

    if let Some(choice_msg) = msg
        .author
        .await_reply(ctx)
        .timeout(Duration::new(30, 0))
        .await
    {
        if let Ok(choice_int) = choice_msg.content.parse::<usize>() {
            if choice_int > 0 && choice_int <= search_results.len() {
                //SAFETY: Already checked bounds manually kekw ratio
                let anime = unsafe { search_results.get_unchecked_mut(choice_int - 1) };
                anime.reload().await;
                choice_msg.delete(ctx).await?;

                let synopsis_unshortened = format!(
                    "{} \n\n {}",
                    anime.synopsis.as_ref().unwrap(),
                    anime.background.as_ref().unwrap()
                );

                let synopsis: String = if synopsis_unshortened.len() > 5500 {
                    String::from("...") + &synopsis_unshortened[0..5497]
                } else {
                    synopsis_unshortened
                };

                let title = format!(
                    "{} `{}`",
                    &anime.title,
                    &anime.alternative_titles.as_ref().unwrap().ja
                );

                sent_choices_message
                    .edit(ctx, |m| {
                        m.content("")
                            .add_embed(|e| {
                                e.title(&title)
                                    .url(&anime.url())
                                    .description(synopsis)
                                    .image(&anime.cover_art.large)
                                    .color(Color::from_rgb(4, 105, 207))
                            })
                            .components(|c| {
                                c.create_action_row(|a| {
                                    a.create_button(|b| {
                                        b.style(ButtonStyle::Primary)
                                            .label("Previous")
                                            .custom_id("anime_prev")
                                    })
                                    .create_button(|b| {
                                        b.style(ButtonStyle::Secondary)
                                            .label("Synopsis")
                                            .custom_id("anime_synopsis")
                                    })
                                    .create_button(|b| {
                                        b.style(ButtonStyle::Secondary)
                                            .label("Details")
                                            .custom_id("anime_details")
                                    })
                                    .create_button(|b| {
                                        b.style(ButtonStyle::Primary)
                                            .label("Next")
                                            .custom_id("anime_next")
                                    })
                                })
                            })
                    })
                    .await?;

                let mut interaction_stream = sent_choices_message
                    .await_component_interactions(ctx)
                    .timeout(Duration::new(30, 0))
                    .await;

                while let Some(interaction) = interaction_stream.next().await {
                    if interaction.data.custom_id == "anime_details" {
                        sent_choices_message
                            .edit(ctx, |m| {
                                m.embed(|e| {
                                    e.title(&title)
                                        .url(&anime.url())
                                        .image(
                                            &anime
                                                .pictures
                                                .as_ref()
                                                .unwrap()
                                                .choose(&mut rand::thread_rng())
                                                .unwrap()
                                                .large,
                                        )
                                        .color(Color::from_rgb(4, 105, 207))
                                        .field(
                                            "Score",
                                            format!("`{}`", anime.score.unwrap_or(0.0)),
                                            true,
                                        )
                                        .field(
                                            "Rank",
                                            format!("`{}`", anime.rank.unwrap_or(0)),
                                            true,
                                        )
                                        .field(
                                            "Popularity",
                                            format!("`{}`", anime.popularity.unwrap_or(0)),
                                            true,
                                        )
                                        .field(
                                            "Number of Episodes",
                                            format!("`{}`", anime.episodes.unwrap_or(0)),
                                            true,
                                        )
                                        .field(
                                            "Season",
                                            {
                                                if let Some(ref season) = anime.start_season {
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
                                                if let Some(ref broadcast) = anime.broadcast {
                                                    format!(
                                                        "`{} {}`",
                                                        broadcast.day_of_the_week,
                                                        broadcast.start_time
                                                    )
                                                } else {
                                                    String::from("`NA`")
                                                }
                                            },
                                            true,
                                        )
                                        .field("Studio(s)", "`TODO`", true)
                                        .field(
                                            "Age Rating",
                                            format!(
                                                "`{}`",
                                                anime
                                                    .rating
                                                    .as_ref()
                                                    .unwrap_or(&mal::prelude::enums::Rating::NA)
                                            ),
                                            true,
                                        )
                                        .field(
                                            "Release",
                                            format!(
                                                "`{}`",
                                                anime.start.as_ref().unwrap_or(&String::from("NA"))
                                            ),
                                            true,
                                        )
                                        .field(
                                            "End",
                                            format!(
                                                "`{}`",
                                                anime.end.as_ref().unwrap_or(&String::from("NA"))
                                            ),
                                            true,
                                        )
                                        .field(
                                            "Status",
                                            format!(
                                                "`{}`",
                                                anime
                                                    .status
                                                    .as_ref()
                                                    .unwrap_or(&mal::prelude::enums::Status::NA)
                                            ),
                                            true,
                                        )
                                        .field(
                                            "Genres",
                                            {
                                                if let Some(genres) = &anime.genres {
                                                    genres
                                                        .iter()
                                                        .map(|g| format!("`{}`", &g.name))
                                                        .collect::<Vec<String>>()
                                                        .join(" | ")
                                                } else {
                                                    String::from("NA")
                                                }
                                            },
                                            true,
                                        )
                                })
                            })
                            .await?;
                    }
                    interaction.defer(ctx).await?;
                }
            } else {
                choice_msg
                    .reply(ctx, "Invalid input, give an integer greater than 0 and less than the number of results pls.")
                    .await?;
            }
        } else {
            choice_msg
                .reply(ctx, "Invalid input, give an integer greater than 0 and less than the number of results pls.")
                .await?;
        }
    } else {
        msg.channel_id.send_message(ctx, |c| c.content("I got no response sadly, try refining your search term if you didn't find your anime.")).await?;
    }
    Ok(())
}
#[group]
#[commands(anime)]
pub struct Weeb;
