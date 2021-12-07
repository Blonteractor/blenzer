pub mod utils;

use log::{error, trace};
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::*,
    prelude::*,
};
use songbird::tracks::LoopState;
use std::time::Duration;
use utils::*;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    let song = search_song(query).await?;

    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let mut handler_lock = voice_manager.get(guild.id);

        if handler_lock.is_none() {
            // User not in vc, join
            join(ctx, msg, args).await?;

            // Try and get handler lock again
            handler_lock = voice_manager.get(guild.id);
        }

        let handler_lock = handler_lock.unwrap();
        let mut handler = handler_lock.lock().await;

        handler.enqueue_source(song);

        let queue = handler.queue().current_queue();
        let latest_track = queue.last().unwrap();

        let embed = song_embed(&latest_track, queue.len())?
            .timestamp(&msg.timestamp)
            .footer(|f| {
                f.text(format!("Requested by {}", &msg.author.name))
                    .icon_url(&msg.author.avatar_url().as_ref().unwrap())
            })
            .to_owned();

        msg.channel_id
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        handler.queue().pause()?;

        msg.reply(ctx, "Music paused").await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let mut handler = handler_lock.lock().await;

        handler.stop();

        //Clears the queue
        handler.queue().modify_queue(|queue| {
            queue.clear();
        });

        msg.reply(ctx, "Music paused").await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("loop")]
async fn loopcurrent(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        match handler.queue().current() {
            Some(track) => match track.get_info().await?.loops {
                LoopState::Finite(0) => {
                    track.enable_loop()?;
                    msg.reply(ctx, "Loop on").await?;
                }
                _ => {
                    track.disable_loop()?;
                    msg.reply(ctx, "Loop off").await?;
                }
            },
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn seek(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let time = match args.single::<u64>() {
        Ok(time) => time,
        Err(_) => {
            msg.reply(ctx, "Invalid time format, enter time in seconds.")
                .await?;
            return Ok(());
        }
    };

    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        match handler.queue().current() {
            Some(track) => {
                if track.metadata().duration.unwrap().as_secs() >= time {
                    track.seek_time(Duration::from_secs(time))?;
                } else {
                    msg.reply(
                        ctx,
                        "Your seek time is more than the duration of this track.",
                    )
                    .await?;
                }
            }
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }

        msg.reply(ctx, format!("Seeked to {}s", time)).await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("fwd")]
async fn forward(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut time = match args.single::<u64>() {
        Ok(time) => time,
        Err(_) => {
            msg.reply(ctx, "Invalid time format, enter time in seconds.")
                .await?;
            return Ok(());
        }
    };

    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        match handler.queue().current() {
            Some(track) => {
                time += track.get_info().await?.position.as_secs();
                if track.metadata().duration.unwrap().as_secs() >= time {
                    track.seek_time(Duration::from_secs(time))?;
                } else {
                    msg.reply(
                        ctx,
                        "Your seek time is more than the duration of this track.",
                    )
                    .await?;
                }
            }
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }

        msg.reply(ctx, format!(":fast_forward: Seeked to **{}s**", time))
            .await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("back")]
async fn backward(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut time = match args.single::<i64>() {
        Ok(time) => time,
        Err(_) => {
            msg.reply(ctx, "Invalid time format, enter time in seconds.")
                .await?;
            return Ok(());
        }
    };

    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        match handler.queue().current() {
            Some(track) => {
                time = (track.get_info().await?.position.as_secs() as i64) - time;
                if time >= 0 {
                    track.seek_time(Duration::from_secs(time as u64))?;
                } else {
                    msg.reply(ctx, "Cant't go back that much.").await?;
                }
            }
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }

        msg.reply(ctx, format!(":arrow_backward: Seeked to **{}s**", time))
            .await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn restart(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        match handler.queue().current() {
            Some(track) => {
                track.seek_time(Duration::from_secs(0))?;
            }
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }

        msg.reply(ctx, "Music restarted").await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        handler.queue().resume()?;

        msg.reply(ctx, "Music resumed").await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        if let Err(_) = handler.queue().skip() {
            msg.reply(ctx, "Already at last song in queue").await?;
        } else {
            msg.reply(ctx, "*Track skipped*").await?;
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

#[command]
#[only_in(guilds)]
#[aliases("q")]
async fn queue(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(mut voice_manager) = songbird::get(ctx).await {
        voice_manager = voice_manager.clone();

        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        let handler = handler_lock.lock().await;

        msg.reply(
            ctx,
            handler
                .queue()
                .current_queue()
                .iter()
                .enumerate()
                .map(|(position, track)| {
                    format!(
                        "**{}.** {}",
                        position + 1,
                        track.metadata().title.as_ref().unwrap_or(&String::new())
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[group]
#[commands(
    play,
    join,
    leave,
    pause,
    resume,
    restart,
    seek,
    skip,
    loopcurrent,
    stop,
    forward,
    backward,
    queue
)]
pub struct Music;
