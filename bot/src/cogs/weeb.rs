use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use std::time::Duration;

use mal::anime::Anime;

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
                let mut anime = unsafe { search_results.get_unchecked_mut(choice_int - 1) };
                anime.reload().await;
                choice_msg.delete(ctx).await?;
                sent_choices_message
                    .edit(ctx, |m| {
                        m.content("").add_embed(|e| {
                            e.title(format!(
                                "{} `{}`",
                                &anime.title,
                                &anime.alternative_titles.as_ref().unwrap().ja
                            ))
                            .url(&anime.url())
                            .description(&anime.synopsis.as_ref().unwrap())
                            .image(&anime.cover_art.large)
                            .color(Color::from_rgb(4, 105, 207))
                        })
                    })
                    .await?;
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
