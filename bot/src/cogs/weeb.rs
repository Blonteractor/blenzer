use mal::manga::Manga;
use rand::seq::SliceRandom;
use serenity::builder::CreateEmbed;
use serenity::constants::EMBED_MAX_LENGTH;
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
        if let Ok(mut choice_int) = choice_msg.content.parse::<usize>() {
            if choice_int > 0 && choice_int <= search_results.len() {
                //SAFETY: Already checked bounds manually kekw ratio
                let mut anime = unsafe { search_results.get_unchecked_mut(choice_int - 1) };
                anime.reload().await;
                choice_msg.delete(ctx).await?;

                let (mut anime_embed_1, mut anime_embed_2) = anime_embed(anime);

                let mut cached = Vec::new();
                cached.push(choice_int);

                sent_choices_message
                    .edit(ctx, |m| {
                        m.content("")
                            .set_embed(anime_embed_1.clone())
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
                            .edit(ctx, |m| m.set_embed(anime_embed_2.clone()))
                            .await?;
                    } else if interaction.data.custom_id == "anime_synopsis" {
                        sent_choices_message
                            .edit(ctx, |m| m.set_embed(anime_embed_1.clone()))
                            .await?;
                    } else {
                        if interaction.data.custom_id == "anime_next" {
                            choice_int += 1;
                            if choice_int > search_results.len() {
                                choice_int = 1;
                            }
                        } else if interaction.data.custom_id == "anime_prev" {
                            choice_int -= 1;
                            if choice_int > 1 {
                                choice_int = search_results.len();
                            }
                        }

                        //SAFETY: Already checked bounds manually kekw ratio
                        anime = unsafe { search_results.get_unchecked_mut(choice_int - 1) };

                        if !cached.contains(&choice_int) {
                            anime.reload().await;
                        }
                        let anime_embeds = anime_embed(anime);
                        anime_embed_1 = anime_embeds.0;
                        anime_embed_2 = anime_embeds.1;
                        sent_choices_message
                            .edit(ctx, |m| m.set_embed(anime_embed_1.clone()))
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

#[command]
async fn manga(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    let mut search_results = Manga::search_basic(query, 10, true).await?;

    let mut sent_choices_message = msg
        .reply(
            ctx,
            search_results
                .iter()
                .enumerate()
                .map(|manga| format!("{}. **{}**", manga.0 + 1, &manga.1.title))
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
        if let Ok(mut choice_int) = choice_msg.content.parse::<usize>() {
            if choice_int > 0 && choice_int <= search_results.len() {
                //SAFETY: Already checked bounds manually kekw ratio
                let mut manga = unsafe { search_results.get_unchecked_mut(choice_int - 1) };
                manga.reload().await;
                choice_msg.delete(ctx).await?;

                let (mut manga_embed_1, mut manga_embed_2) = manga_embed(manga);

                let mut cached = Vec::new();
                cached.push(choice_int);

                sent_choices_message
                    .edit(ctx, |m| {
                        m.content("")
                            .set_embed(manga_embed_1.clone())
                            .components(|c| {
                                c.create_action_row(|a| {
                                    a.create_button(|b| {
                                        b.style(ButtonStyle::Primary)
                                            .label("Previous")
                                            .custom_id("manga_prev")
                                    })
                                    .create_button(|b| {
                                        b.style(ButtonStyle::Secondary)
                                            .label("Synopsis")
                                            .custom_id("manga_synopsis")
                                    })
                                    .create_button(|b| {
                                        b.style(ButtonStyle::Secondary)
                                            .label("Details")
                                            .custom_id("manga_details")
                                    })
                                    .create_button(|b| {
                                        b.style(ButtonStyle::Primary)
                                            .label("Next")
                                            .custom_id("manga_next")
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
                    if interaction.data.custom_id == "manga_details" {
                        sent_choices_message
                            .edit(ctx, |m| m.set_embed(manga_embed_2.clone()))
                            .await?;
                    } else if interaction.data.custom_id == "manga_synopsis" {
                        sent_choices_message
                            .edit(ctx, |m| m.set_embed(manga_embed_1.clone()))
                            .await?;
                    } else {
                        if interaction.data.custom_id == "manga_next" {
                            choice_int += 1;
                            if choice_int > search_results.len() {
                                choice_int = 1;
                            }
                        } else if interaction.data.custom_id == "manga_prev" {
                            choice_int -= 1;
                            if choice_int > 1 {
                                choice_int = search_results.len();
                            }
                        }

                        //SAFETY: Already checked bounds manually kekw ratio
                        manga = unsafe { search_results.get_unchecked_mut(choice_int - 1) };

                        if !cached.contains(&choice_int) {
                            manga.reload().await;
                        }
                        let manga_embeds = manga_embed(manga);
                        manga_embed_1 = manga_embeds.0;
                        manga_embed_2 = manga_embeds.1;
                        sent_choices_message
                            .edit(ctx, |m| m.set_embed(manga_embed_1.clone()))
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
        msg.channel_id.send_message(ctx, |c| c.content("I got no response sadly, try refining your search term if you didn't find your manga.")).await?;
    }
    Ok(())
}

fn manga_embed(manga: &Manga) -> (CreateEmbed, CreateEmbed) {
    let synopsis_unshortened = format!(
        "{} \n\n {}",
        manga.synopsis.as_ref().unwrap(),
        manga.background.as_ref().unwrap()
    );

    let synopsis: String = if synopsis_unshortened.len() > EMBED_MAX_LENGTH {
        String::from("...") + &synopsis_unshortened[0..(EMBED_MAX_LENGTH - 5)]
    } else {
        synopsis_unshortened
    };

    let title = format!(
        "{} `{}`",
        &manga.title,
        &manga.alternative_titles.as_ref().unwrap().ja
    );

    let page1 = CreateEmbed::default()
        .title(&title)
        .url(&manga.url())
        .description(synopsis)
        .image(&manga.cover_art.large)
        .color(Color::from_rgb(4, 105, 207))
        .to_owned();

    let page2 = CreateEmbed::default()
        .title(&title)
        .url(&manga.url())
        .image(
            &manga
                .pictures
                .as_ref()
                .unwrap()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .large,
        )
        .field("Score", format!("`{}`", manga.score.unwrap_or(0.0)), true)
        .field("Rank", format!("`{}`", manga.rank.unwrap_or(0)), true)
        .field(
            "Popularity",
            format!("`{}`", manga.popularity.unwrap_or(0)),
            true,
        )
        .field(
            "Number of Chapters",
            format!("`{}`", manga.chapters.unwrap_or(0)),
            true,
        )
        .field(
            "Number of Volumes",
            format!("`{}`", manga.volumes.unwrap_or(0)),
            true,
        )
        .field(
            "Author(s)",
            {
                if let Some(authors) = &manga.authors {
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
                manga
                    .rating
                    .as_ref()
                    .unwrap_or(&mal::prelude::enums::Rating::NA)
            ),
            true,
        )
        .field(
            "Release",
            format!("`{}`", manga.start.as_ref().unwrap_or(&String::from("NA"))),
            true,
        )
        .field(
            "End",
            format!("`{}`", manga.end.as_ref().unwrap_or(&String::from("NA"))),
            true,
        )
        .field(
            "Status",
            format!(
                "`{}`",
                manga
                    .status
                    .as_ref()
                    .unwrap_or(&mal::prelude::enums::Status::NA)
            ),
            true,
        )
        .field(
            "Genres",
            {
                if let Some(genres) = &manga.genres {
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

fn cleanly_join_vec(to_join: &Vec<impl ToString>) -> String {
    to_join
        .iter()
        .map(|g| format!("`{}`", &g.to_string()))
        .collect::<Vec<String>>()
        .join(" | ")
}

fn anime_embed(anime: &Anime) -> (CreateEmbed, CreateEmbed) {
    let synopsis_unshortened = format!(
        "{} \n\n {}",
        anime.synopsis.as_ref().unwrap(),
        anime.background.as_ref().unwrap()
    );

    let synopsis: String = if synopsis_unshortened.len() > EMBED_MAX_LENGTH {
        String::from("...") + &synopsis_unshortened[0..(EMBED_MAX_LENGTH - 5)]
    } else {
        synopsis_unshortened
    };

    let title = format!(
        "{} `{}`",
        &anime.title,
        &anime.alternative_titles.as_ref().unwrap().ja
    );

    let page1 = CreateEmbed::default()
        .title(&title)
        .url(&anime.url())
        .description(synopsis)
        .image(&anime.cover_art.large)
        .color(Color::from_rgb(4, 105, 207))
        .to_owned();

    let page2 = CreateEmbed::default()
        .title(&title)
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
        .field("Score", format!("`{}`", anime.score.unwrap_or(0.0)), true)
        .field("Rank", format!("`{}`", anime.rank.unwrap_or(0)), true)
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
                if let Some(genres) = &anime.studios {
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
                anime
                    .rating
                    .as_ref()
                    .unwrap_or(&mal::prelude::enums::Rating::NA)
            ),
            true,
        )
        .field(
            "Release",
            format!("`{}`", anime.start.as_ref().unwrap_or(&String::from("NA"))),
            true,
        )
        .field(
            "End",
            format!("`{}`", anime.end.as_ref().unwrap_or(&String::from("NA"))),
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
#[group]
#[commands(anime, manga)]
pub struct Weeb;
