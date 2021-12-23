mod utils;

use mal::manga::Manga;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use super::super::util::parsers;
use mal::anime::Anime;

use utils::*;

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

    let awaited_choice = parsers::await_choice_int(
        ctx,
        msg,
        search_results.len() as isize,
        0,
        30,
        |_| INVALID_INPUT_MSG,
        || INVALID_INPUT_MSG,
    )
    .await?;

    if let Some(choice_int) = awaited_choice {
        //SAFETY: Already checked bounds manually kekw ratio
        let anime = unsafe { search_results.get_unchecked_mut((choice_int as usize) - 1) };
        anime.reload().await;

        let (anime_embed_1, _) = anime.double_paged_embed();

        //let mut cached = vec![choice_int];

        sent_choices_message
            .edit(ctx, |m| {
                m.content("")
                    .set_embed(anime_embed_1.clone())
                    .components(|c| {
                        c.set_action_rows(vec![create_actionrow_for_pagination(
                            "Synopsis", "Details",
                        )])
                    })
            })
            .await?;

        handle_component_interactions_for_pagination(
            &mut sent_choices_message,
            ctx,
            choice_int as usize,
            &mut search_results,
            |a| Ok(a.double_paged_embed().0),
            |a| Ok(a.double_paged_embed().1),
        )
        .await?;
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

    let awaited_choice = parsers::await_choice_int(
        ctx,
        msg,
        search_results.len() as isize,
        0,
        30,
        |_| INVALID_INPUT_MSG,
        || INVALID_INPUT_MSG,
    )
    .await?;

    if let Some(choice_int) = awaited_choice {
        //SAFETY: Already checked bounds manually kekw ratio
        let manga = unsafe { search_results.get_unchecked_mut((choice_int as usize) - 1) };
        manga.reload().await;

        let (manga_embed_1, _) = manga.double_paged_embed();

        //let cached = vec![choice_int];

        sent_choices_message
            .edit(ctx, |m| {
                m.content("")
                    .set_embed(manga_embed_1.clone())
                    .components(|c| {
                        c.set_action_rows(vec![create_actionrow_for_pagination(
                            "Details", "Synopsis",
                        )])
                    })
            })
            .await?;

        handle_component_interactions_for_pagination(
            &mut sent_choices_message,
            ctx,
            choice_int as usize,
            &mut search_results,
            |a| Ok(a.double_paged_embed().0),
            |a| Ok(a.double_paged_embed().1),
        )
        .await?;
    }

    Ok(())
}

#[group]
#[commands(anime, manga)]
pub struct Weeb;
