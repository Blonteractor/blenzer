use super::super::util::parsers;
use log::{debug, error, warn};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use serenity::{
    builder::CreateEmbed,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
};
use songbird::{
    input::{Input, Restartable},
    Call,
};
use std::sync::Arc;
use youtube_dl::YoutubeDl;
use youtube_dl::YoutubeDlOutput;

#[command]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let urls = parsers::url(&mut args);

    if urls.is_empty() {
        msg.reply(ctx, "Invalid URL.").await?;
    }

    // Already checked urls was not empty, leaking the string looks hacky idk, might cause memory leaks help
    let url = string_to_static_str(urls.into_iter().next().unwrap());

    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        if let Some(handler_lock) = voice_manager.get(guild.id) {
            match play_song(url, handler_lock).await {
                Ok(()) => {
                    let embed = song_embed(url.to_string())?
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
                    let embed = song_embed(url.to_string())?
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

fn song_embed(url: impl ToString) -> Result<CreateEmbed, youtube_dl::Error> {
    let video = if let YoutubeDlOutput::SingleVideo(video) = YoutubeDl::new(&url.to_string())
        .extract_audio(false)
        .run()?
    {
        video
    } else {
        warn!("Song embed requested for playlist url");
        unreachable!();
    };

    Ok(CreateEmbed::default()
        .image(video.thumbnail.unwrap_or_default())
        .title("Song added to queue")
        .description(format!("**#1** \n *{}*", video.title))
        .url(&url.to_string())
        .color(Color::from_rgb(4, 105, 207))
        .to_owned())
}

fn search_song(query: impl ToString) -> Result<String, youtube_dl::Error> {
    let ytldl = YoutubeDl::new(query.to_string());
    Ok(String::new())
}
async fn play_song(
    url: &'static str,
    handler_lock: Arc<Mutex<Call>>,
) -> Result<(), songbird::input::error::Error> {
    let mut handler = handler_lock.lock().await;

    let source = match Restartable::ytdl(url, true).await {
        Ok(source) => source,
        Err(why) => {
            error!("Couldn't start source: {:?}", why);
            return Err(why);
        }
    };

    handler.play_source(Input::from(source));

    Ok(())
}
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
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

            debug!(
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

            debug!(
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
