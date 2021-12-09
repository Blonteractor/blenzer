pub mod utils;

use log::{error, trace, warn};
use num_traits::ToPrimitive;
use serenity::{
    builder::CreateEmbed,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::*,
    prelude::*,
    utils::Color,
};
use songbird::tracks::{LoopState, PlayMode};
use std::time::Duration;
use utils::*;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    let song_search_result =
        if query.starts_with("https://www.") || query.starts_with("http://www.") {
            get_song(query).await
        } else {
            search_song(query).await
        };

    let song = match song_search_result {
        Ok(inp) => inp,
        Err(_) => {
            msg.reply(ctx, "Couldn't find anything with that query.")
                .await?;
            return Ok(());
        }
    };

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

        {
            let mut writer = latest_track.typemap().write().await;
            writer.insert::<SongRequestedBy>(msg.author.id);
        }

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
#[aliases("vol")]
async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let volume = match args.single::<u64>() {
        Ok(vol) => {
            if vol <= 100 {
                vol
            } else {
                msg.reply(ctx, "Invalid volume, enter a number between 0 and 100.")
                    .await?;
                return Ok(());
            }
        }
        Err(_) => {
            msg.reply(ctx, "Invalid volume, enter a number between 0 and 100.")
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
                track.set_volume(volume as f32 / 100.0).unwrap();
            }
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }

        msg.reply(ctx, format!("Track volume set to {}", volume))
            .await?;
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

        let queue = handler.queue();

        let queue_string = if !queue.is_empty() {
            let mut queue_string = String::new();

            for (position, track) in queue.current_queue().iter().enumerate() {
                let track_str = format!(
                    "**{}.** {} | *Requested By: {}*",
                    position + 1,
                    track.metadata().title.as_ref().unwrap_or(&String::new()),
                    track
                        .typemap()
                        .read()
                        .await
                        .get::<SongRequestedBy>()
                        .unwrap()
                        .to_user_cached(ctx)
                        .await
                        .unwrap_or_default()
                        .name
                );

                queue_string += &track_str;
            }

            queue_string
        } else {
            String::from(
                "The queue is empty! Use the *play command* to add something to the queue.",
            )
        };

        msg.reply(ctx, queue_string).await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("np")]
async fn nowplaying(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
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

        let np_track = match handler.queue().current() {
            Some(track) => track,
            None => {
                msg.reply(
                    ctx,
                    "There is nothing playing, use the *play command* to play something.",
                )
                .await?;
                return Ok(());
            }
        };

        let np_info = np_track.get_info().await?;

        let track_progress = match np_track.get_info().await {
            Ok(state) => state.position,
            Err(why) => {
                warn!("Couldnt get info of track: {}", why);
                Duration::from_secs(0)
            }
        };

        let np_metadata = np_track.metadata();
        let np_duration = np_metadata.duration.unwrap_or(Duration::from_secs(1));

        let footer_text_loop = match np_info.loops {
            LoopState::Finite(0) => "",
            LoopState::Infinite | LoopState::Finite(_) => "🔁",
        };

        let footer_text_pause = match np_info.playing {
            PlayMode::Pause => "▶️",
            PlayMode::Play => "⏸️",
            _ => unreachable!(),
        };

        let mut footer_text_volume = if np_info.volume >= 0.66 {
            "🔊 "
        } else if np_info.volume >= 0.33 {
            "🔉 "
        } else if np_info.volume >= 0.01 {
            "🔈 "
        } else {
            "🔇 "
        }
        .to_owned();

        let requested_by = np_track
            .typemap()
            .read()
            .await
            .get::<SongRequestedBy>()
            .unwrap()
            .to_user(ctx)
            .await;

        footer_text_volume += &(np_info.volume * 100.0).to_string();

        let np_embed = CreateEmbed::default()
            .title("Now Playing")
            .url(np_metadata.source_url.as_ref().unwrap_or(&String::new()))
            .color(Color::from_rgb(4, 105, 207))
            .thumbnail(np_metadata.thumbnail.as_ref().unwrap_or(&String::from(
                "https://bitsofco.de/content/images/2018/12/broken-1.png",
            )))
            .description(format!(
                "*{}* \n\n **{} / {}**  {}",
                np_metadata.title.as_ref().unwrap_or(&String::new()),
                track_progress.to_human_readable(),
                np_duration.to_human_readable(),
                {
                    let seeker = "▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬";
                    let seeker_button = ":radio_button:";

                    let progress_ratio: f64 =
                        track_progress.as_secs_f64() / np_duration.as_secs() as f64;

                    let seeker_pos = (((seeker.chars().count()) as f64) * progress_ratio)
                        .round()
                        .to_usize()
                        .unwrap();

                    seeker
                        .chars()
                        .into_iter()
                        .enumerate()
                        .map(|(i, c)| {
                            if i == seeker_pos {
                                seeker_button.to_string()
                            } else {
                                c.to_string()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("")
                }
            ))
            .footer(|f| {
                f.text(format!(
                    "Requested by: {} || {} {} {}",
                    {
                        match requested_by {
                            Ok(ref user) => user.name.clone(),
                            Err(_) => String::from("Unknown"),
                        }
                    },
                    footer_text_loop,
                    footer_text_pause,
                    footer_text_volume,
                ))
                .icon_url({
                    match requested_by {
                        Ok(ref user) => user.avatar_url().unwrap_or(String::from(
                            "https://bitsofco.de/content/images/2018/12/broken-1.png",
                        )),
                        Err(_) => {
                            String::from("https://bitsofco.de/content/images/2018/12/broken-1.png")
                        }
                    }
                })
            })
            .to_owned();

        msg.channel_id
            .send_message(ctx, |m| m.set_embed(np_embed))
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
    queue,
    nowplaying,
    volume
)]
pub struct Music;
