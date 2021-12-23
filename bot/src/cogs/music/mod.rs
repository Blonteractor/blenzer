pub mod utils;

use log::{trace, warn};
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
use songbird::Songbird;
use std::sync::Arc;
use std::time::Duration;
use utils::*;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn play_track(
        ctx: &Context,
        msg: &Message,
        args: Args,
        mut handler_lock: Option<VoiceHandler>,
        voice_manager: Arc<Songbird>,
    ) -> CommandResult {
        let query = args.rest();

        let song_search_result = if query.starts_with("https://www.")
            || query.starts_with("http://www.")
        {
            if query.contains("list=") {
                msg.reply(ctx, "WARNING: Playing playlists is kinda slow, it takes almost 3 sec / song to queue it so be patient kthxbye.").await?;
                get_playlist(query).await
            } else {
                vec![get_song(query).await]
            }
        } else {
            vec![search_song(query).await]
        };

        let mut songs = Vec::new();

        for song in song_search_result {
            songs.push(match song {
                Ok(inp) => inp,
                Err(_) => {
                    msg.reply(ctx, "Couldn't find anything with that query.")
                        .await?;
                    continue;
                }
            })
        }

        if handler_lock.is_none() {
            // User not in vc, join
            join(ctx, msg, args).await?;

            // Try and get handler lock again
            handler_lock = voice_manager.get(msg.guild_id.unwrap());
        }

        let handler_lock = handler_lock.unwrap();
        let mut handler = handler_lock.lock().await;

        let queue_len = handler.queue().len();
        let songs_len = songs.len();

        for song in songs {
            handler.enqueue_source(song);
        }

        let queue = handler.queue().current_queue();
        drop(handler);

        let embed = if songs_len == 1 {
            let latest_track = queue.last().unwrap();

            {
                let mut writer = latest_track.typemap().write().await;
                writer.insert::<SongRequestedBy>(msg.author.id);
            }

            song_embed(latest_track, queue.len())
                .timestamp(&msg.timestamp)
                .footer(|f| {
                    f.text(format!("Requested by {}", &msg.author.name))
                        .icon_url(&msg.author.avatar_url().as_ref().unwrap())
                })
                .to_owned()
        } else {
            let latest_tracks = &queue[queue_len..queue_len + songs_len];

            for track in latest_tracks {
                let mut writer = track.typemap().write().await;
                writer.insert::<SongRequestedBy>(msg.author.id);
            }
            playlist_embed(latest_tracks)
        };

        msg.channel_id
            .send_message(ctx, |m| m.set_embed(embed))
            .await?;
        Ok(())
    }

    try_with_handler(ctx, msg, args, play_track).await
}

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn pause_track(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let handler = handler.lock().await;

        let pause_result = handler.queue().pause();
        drop(handler);

        if pause_result.is_err() {
            msg.reply(ctx, "Track not playing").await?;
        } else {
            msg.reply(ctx, "*Track paused*").await?;
        }

        Ok(())
    }

    with_handler(ctx, msg, args, pause_track).await
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn stop_queue(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let mut handler = handler.lock().await;
        handler.stop();

        //Clears the queue
        handler.queue().modify_queue(|queue| {
            queue.clear();
        });

        drop(handler);

        msg.reply(ctx, "Music Stopped").await?;

        Ok(())
    }

    with_handler(ctx, msg, args, stop_queue).await
}

#[command]
#[only_in(guilds)]
#[aliases("loop")]
async fn loopcurrent(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn loop_track(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let handler = handler.lock().await;
        let current = handler.queue().current();
        drop(handler);

        match current {
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

        Ok(())
    }

    with_handler(ctx, msg, args, loop_track).await
}

#[command]
#[only_in(guilds)]
async fn seek(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn seek_track(
        ctx: &Context,
        msg: &Message,
        mut args: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let duration = match args.single::<String>() {
            Ok(time) => match Duration::from_human_readable(time) {
                Some(duration) => duration,
                None => {
                    msg.reply(ctx, "Invalid time format").await?;
                    return Ok(());
                }
            },
            Err(_) => {
                msg.reply(ctx, "Invalid time format").await?;
                return Ok(());
            }
        };

        let time = duration.as_secs();

        let handler = handler.lock().await;
        let current = handler.queue().current();
        drop(handler);

        match current {
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
        msg.reply(
            ctx,
            format!("Seeked to `**{}**`", &duration.to_human_readable()),
        )
        .await?;

        Ok(())
    }

    with_handler(ctx, msg, args, seek_track).await
}

#[command]
#[only_in(guilds)]
#[aliases("vol")]
async fn volume(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn vol_track(
        ctx: &Context,
        msg: &Message,
        mut args: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
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

        let handler = handler.lock().await;
        let current = handler.queue().current();
        drop(handler);

        match current {
            Some(track) => {
                track.set_volume(volume as f32 / 100.0).unwrap();
            }
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }

        msg.reply(ctx, format!("Track volume set to {}", volume))
            .await?;

        Ok(())
    }

    with_handler(ctx, msg, args, vol_track).await
}

#[command]
#[only_in(guilds)]
#[aliases("fwd")]
async fn forward(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn fwd_track(
        ctx: &Context,
        msg: &Message,
        mut args: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let mut time = match args.single::<u64>() {
            Ok(time) => time,
            Err(_) => {
                msg.reply(ctx, "Invalid time format, enter time in seconds.")
                    .await?;
                return Ok(());
            }
        };

        let handler = handler.lock().await;
        let current = handler.queue().current();
        drop(handler);

        match current {
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

        msg.reply(
            ctx,
            format!(
                ":fast_forward: Seeked to **{}**",
                Duration::from_secs(time).to_human_readable()
            ),
        )
        .await?;

        Ok(())
    }

    with_handler(ctx, msg, args, fwd_track).await
}

#[command]
#[only_in(guilds)]
#[aliases("back")]
async fn backward(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn back_track(
        ctx: &Context,
        msg: &Message,
        mut args: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let mut time = match args.single::<i64>() {
            Ok(time) => time,
            Err(_) => {
                msg.reply(ctx, "Invalid time format, enter time in seconds.")
                    .await?;
                return Ok(());
            }
        };

        let handler = handler.lock().await;
        let current = handler.queue().current();
        drop(handler);

        match current {
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

        msg.reply(
            ctx,
            format!(
                ":rewind: Seeked to **{}**",
                Duration::from_secs(time as u64).to_human_readable()
            ),
        )
        .await?;

        Ok(())
    }

    with_handler(ctx, msg, args, back_track).await
}

#[command]
#[only_in(guilds)]
async fn restart(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn res_track(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let handler = handler.lock().await;

        let current = handler.queue().current();
        drop(handler);

        match current {
            Some(track) => {
                track.seek_time(Duration::from_secs(0))?;
            }
            None => {
                msg.reply(ctx, "Nothing's playing").await?;
            }
        }

        msg.reply(ctx, "Music restarted").await?;

        Ok(())
    }

    with_handler(ctx, msg, args, res_track).await
}

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn res_track(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let handler = handler.lock().await;

        let res_result = handler.queue().resume();
        drop(handler);

        if res_result.is_err() {
            msg.reply(ctx, "Track not paused").await?;
        } else {
            msg.reply(ctx, "*Track resumed*").await?;
        }

        Ok(())
    }

    with_handler(ctx, msg, args, res_track).await
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn skip_track(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let handler = handler.lock().await;

        let skip_result = handler.queue().skip();
        drop(handler);

        if skip_result.is_err() {
            msg.reply(ctx, "Already at last song in queue").await?;
        } else {
            msg.reply(ctx, "*Track skipped*").await?;
        }

        Ok(())
    }

    with_handler(ctx, msg, args, skip_track).await
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn join_channel(
        ctx: &Context,
        msg: &Message,
        _: Args,
        _: Option<VoiceHandler>,
        voice_manager: Arc<Songbird>,
    ) -> CommandResult {
        let guild = msg.guild(&ctx).await.unwrap();

        if let Some(id) = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|state| state.channel_id)
        {
            voice_manager.join(guild.id, id).await.1?;

            trace!(
                "Joined voice channel {} in server {} ({}) as commander by user {}",
                id.mention(),
                guild.name,
                guild.id,
                msg.author.mention()
            );

            msg.reply(ctx, format!("joined {}", id.mention())).await?;
        } else {
            msg.reply(ctx, "User not in a voice channel").await?;

            return Ok(());
        }

        Ok(())
    }

    try_with_handler(ctx, msg, args, join_channel).await
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn leave_channel(
        ctx: &Context,
        msg: &Message,
        _: Args,
        _: Option<VoiceHandler>,
        voice_manager: Arc<Songbird>,
    ) -> CommandResult {
        let guild = msg.guild(&ctx).await.unwrap();

        if let Some(id) = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|state| state.channel_id)
        {
            voice_manager.leave(guild.id).await?;

            trace!(
                "Left voice channel {} in server {} ({}) as commander by user {}",
                id.mention(),
                guild.name,
                guild.id,
                msg.author.mention()
            );

            msg.reply(ctx, format!("Left {}", id.mention())).await?;
        } else {
            msg.reply(ctx, "User not in a voice channel").await?;

            return Ok(());
        }

        Ok(())
    }

    try_with_handler(ctx, msg, args, leave_channel).await
}

#[command]
#[only_in(guilds)]
#[aliases("q")]
async fn queue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn q(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let handler = handler.lock().await;

        let queue = handler.queue();

        let current_queue = queue.current_queue();
        let is_queue_empty = queue.is_empty();

        drop(handler);

        let queue_string = if !is_queue_empty {
            let mut queue_string = String::new();

            for (position, track) in current_queue.iter().enumerate() {
                let track_str = format!(
                    "**{}.** {} | *Requested By: {}* \n",
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

        Ok(())
    }

    with_handler(ctx, msg, args, q).await
}

#[command]
#[only_in(guilds)]
#[aliases("np")]
async fn nowplaying(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    async fn np(
        ctx: &Context,
        msg: &Message,
        _: Args,
        handler: VoiceHandler,
        _: Arc<Songbird>,
    ) -> CommandResult {
        let handler = handler.lock().await;

        let current = handler.queue().current();
        drop(handler);

        let np_track = match current {
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
        let np_duration = np_metadata
            .duration
            .unwrap_or_else(|| Duration::from_secs(1));

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
                        Ok(ref user) => user.avatar_url().unwrap_or_else(|| {
                            String::from("https://bitsofco.de/content/images/2018/12/broken-1.png")
                        }),
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
        Ok(())
    }

    with_handler(ctx, msg, args, np).await
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
