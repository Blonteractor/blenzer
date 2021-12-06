pub mod utils;

use log::{error, trace};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use utils::*;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    let search_result = search_songs(query, 1)?.into_iter().next();

    let song = match search_result {
        Some(vid) => vid,
        None => {
            msg.reply(ctx, "No results were found with the query")
                .await?;
            return Ok(());
        }
    };

    let url = song.webpage_url.as_ref().unwrap().clone();

    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        if let Some(handler_lock) = voice_manager.get(guild.id) {
            match play_song(url, handler_lock).await {
                Ok(()) => {
                    let embed = song_embed(song)?
                        .timestamp(&msg.timestamp)
                        .footer(|f| {
                            f.text(format!("Requested by {}", &msg.author.name))
                                .icon_url(&msg.author.avatar_url().as_ref().unwrap())
                        })
                        .to_owned();

                    msg.channel_id
                        .send_message(ctx, |m| m.set_embed(embed))
                        .await?;
                }
                Err(_) => {
                    msg.reply(ctx, "Error playing your song.").await?;
                }
            }
        } else {
            // User not in vc, join
            join(ctx, msg, args).await?;

            // Try and get lock again after joining vc
            let handler_lock = voice_manager.get(guild.id).unwrap();

            let _ = match play_song(url, handler_lock).await {
                Ok(()) => {
                    let embed = song_embed(song)?
                        .timestamp(&msg.timestamp)
                        .footer(|f| {
                            f.text(format!("Requested by {}", &msg.author.name))
                                .icon_url(&msg.author.avatar_url().as_ref().unwrap())
                        })
                        .to_owned();

                    msg.channel_id
                        .send_message(ctx, |m| m.set_embed(embed))
                        .await?;
                }
                Err(_) => {
                    msg.reply(ctx, "Error playing your song.").await?;
                }
            };
        }
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx).await.unwrap();

    if let Some(id) = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|state| state.channel_id)
    {
        if let Some(mut voice_manager) = songbird::get(ctx).await {
            voice_manager = voice_manager.clone();
            voice_manager.join(guild.id, id).await.1?;

            trace!(
                "Joined voice channel {} in server {} ({}) as commander by user {}",
                id.mention(),
                guild.name,
                guild.id,
                msg.author.mention()
            );

            msg.reply(ctx, format!("Joined {}", id.mention())).await?;
        } else {
            error!("Couldn't retreive the songbird voice manager");
            return Ok(());
        }
    } else {
        msg.reply(ctx, "User not in a voice channel").await?;

        return Ok(());
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx).await.unwrap();

    if let Some(id) = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|state| state.channel_id)
    {
        if let Some(mut voice_manager) = songbird::get(ctx).await {
            voice_manager = voice_manager.clone();
            voice_manager.leave(guild.id).await?;

            trace!(
                "Left voice channel {} in server {} ({}) as commander by user {}",
                id.mention(),
                guild.name,
                guild.id,
                msg.author.mention()
            );

            msg.reply(ctx, format!("Joined {}", id.mention())).await?;
        } else {
            error!("Couldn't retreive the songbird voice manager");
            return Ok(());
        }
    } else {
        msg.reply(ctx, "User not in a voice channel").await?;

        return Ok(());
    }

    Ok(())
}

#[group]
#[commands(play, join, leave)]
pub struct Music;
