use log::{debug, error};
use serenity::{
    builder::CreateEmbed,
    framework::standard::{Args, CommandResult},
    model::{channel::Message, id::UserId},
    prelude::*,
    utils::Color,
};
use songbird::{
    input::{self, Input, Restartable},
    tracks::TrackHandle,
    Songbird,
};
use std::{future::Future, sync::Arc, time::Duration};
use tokio::runtime::Handle as RuntimeHandle;
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput};

pub type VoiceHandler = std::sync::Arc<tokio::sync::Mutex<songbird::Call>>;

pub async fn get_song(url: impl ToString) -> Result<Input, input::error::Error> {
    let source = match Restartable::ytdl(url.to_string(), true).await {
        Ok(source) => source,
        Err(why) => {
            error!("Couldn't start source: {:?}", why);
            return Err(why);
        }
    };

    Ok(source.into())
}

pub async fn search_songs(
    query: impl AsRef<str> + Sync + Send + 'static,
    limit: usize,
) -> Vec<Result<Input, input::error::Error>> {
    let ytdl = RuntimeHandle::current()
        .spawn_blocking(move || {
            YoutubeDl::search_for(&SearchOptions::youtube(query.as_ref()).with_count(limit))
                .flat_playlist(true)
                .extract_audio(false)
                .run()
                .unwrap()
        })
        .await
        .unwrap();

    if let YoutubeDlOutput::Playlist(search_result) = ytdl {
        let search_results_vec = search_result.entries.unwrap_or_default();
        let mut videos = Vec::with_capacity(search_results_vec.len());

        for vid in search_results_vec.into_iter() {
            videos.push(Ok(Restartable::ytdl(
                vid.webpage_url.unwrap_or_default(),
                true,
            )
            .await
            .unwrap()
            .into()));
        }

        return videos;
    } else {
        unreachable!()
    }
}

pub async fn get_playlist(url: impl AsRef<str>) -> Vec<Result<Input, input::error::Error>> {
    debug!("Making ytdl do shit....");
    let ytdl = YoutubeDl::new(url.as_ref())
        .extract_audio(false)
        .flat_playlist(true)
        .run()
        .unwrap();
    debug!("Ytdl did shit.");

    if let YoutubeDlOutput::Playlist(search_result) = ytdl {
        let search_results_vec = search_result.entries.unwrap_or_default();
        let mut videos = Vec::with_capacity(search_results_vec.len());

        debug!("Making serenity do shit");
        for vid in search_results_vec.into_iter() {
            videos.push(Ok(Restartable::ytdl(
                format!("https://www.youtube.com/watch?v={}", vid.id),
                true,
            )
            .await
            .unwrap()
            .into()));
        }
        debug!("serenity did shit.");

        return videos;
    } else {
        unreachable!()
    }
}
pub async fn search_song(query: impl ToString) -> Result<Input, input::error::Error> {
    let source = match Restartable::ytdl_search(query.to_string(), true).await {
        Ok(source) => source,
        Err(why) => {
            error!("Couldn't start source: {:?}", why);
            return Err(why);
        }
    };

    Ok(source.into())
}

pub fn song_embed(track_handle: &TrackHandle, position: usize) -> CreateEmbed {
    let metadata = track_handle.metadata();
    CreateEmbed::default()
        .image(metadata.thumbnail.as_ref().unwrap_or(&String::default()))
        .title("Song added to queue")
        .description(format!(
            "**#{}** \n *{}*",
            position,
            metadata.title.as_ref().unwrap_or(&String::default())
        ))
        .url(metadata.source_url.as_ref().unwrap_or(&String::default()))
        .color(Color::from_rgb(4, 105, 207))
        .to_owned()
}

pub fn playlist_embed(tracks: &[TrackHandle]) -> CreateEmbed {
    let metadata = tracks.iter().next().unwrap().metadata();
    let mut description = String::new();

    for (i, track) in tracks.iter().enumerate() {
        let metadata = track.metadata();
        description += &format!(
            "**{}.** *{}* \n",
            i + 1,
            metadata.title.as_ref().unwrap_or(&String::default())
        );
    }

    CreateEmbed::default()
        .image(metadata.thumbnail.as_ref().unwrap_or(&String::default()))
        .title("Playlist added to queue")
        .description(description)
        .url(metadata.source_url.as_ref().unwrap_or(&String::default()))
        .color(Color::from_rgb(4, 105, 207))
        .to_owned()
}

pub async fn play_song_now(
    url: String,
    handler_lock: Arc<VoiceHandler>,
) -> Result<TrackHandle, songbird::input::error::Error> {
    let mut handler = handler_lock.lock().await;

    let source = match Restartable::ytdl(url, true).await {
        Ok(source) => source,
        Err(why) => {
            error!("Couldn't start source: {:?}", why);
            return Err(why);
        }
    };

    Ok(handler.play_source(source.into()))
}

pub async fn add_song_to_queue(
    url: String,
    handler_lock: &Arc<VoiceHandler>,
) -> Result<(), songbird::input::error::Error> {
    let mut handler = handler_lock.lock().await;

    let source = match Restartable::ytdl(url, true).await {
        Ok(source) => source,
        Err(why) => {
            error!("Couldn't start source: {:?}", why);
            return Err(why);
        }
    };

    handler.enqueue_source(source.into());

    Ok(())
}

pub async fn with_handler<'a, Fut, Cmd>(
    ctx: &'a Context,
    msg: &'a Message,
    args: Args,
    execute: Cmd,
) -> CommandResult
where
    Fut: Future<Output = CommandResult>,
    Cmd: FnOnce(&'a Context, &'a Message, Args, VoiceHandler, Arc<Songbird>) -> Fut,
{
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(voice_manager) = songbird::get(ctx).await {
        let handler_lock = if let Some(hl) = voice_manager.get(guild.id) {
            hl
        } else {
            // User not in vc
            msg.reply(ctx, "I aint even in a vc").await?;

            return Ok(());
        };

        execute(ctx, msg, args, handler_lock, voice_manager).await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}

pub async fn try_with_handler<'a, Fut, Cmd>(
    ctx: &'a Context,
    msg: &'a Message,
    args: Args,
    execute: Cmd,
) -> CommandResult
where
    Fut: Future<Output = CommandResult>,
    Cmd: FnOnce(&'a Context, &'a Message, Args, Option<VoiceHandler>, Arc<Songbird>) -> Fut,
{
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Some(voice_manager) = songbird::get(ctx).await {
        let handler_lock = voice_manager.get(guild.id);

        execute(ctx, msg, args, handler_lock, voice_manager).await?;
    } else {
        error!("Couldn't retreive the songbird voice manager");
        return Ok(());
    }
    Ok(())
}
pub struct SongRequestedBy;

impl TypeMapKey for SongRequestedBy {
    type Value = UserId;
}

pub trait HumanReadable {
    fn to_human_readable(&self) -> String;

    fn from_human_readable(query: String) -> Option<Self>
    where
        Self: Sized;
}

impl HumanReadable for Duration {
    fn to_human_readable(&self) -> String {
        let total_secs = self.as_secs();
        let hours = total_secs / 3600;
        let minutes = total_secs / 60;
        let secs = total_secs % 60;

        let minutes_str = if minutes < 10 {
            format!("0{}:", minutes)
        } else {
            format!("{}:", minutes)
        };

        let seconds_str = if secs < 10 {
            format!("0{}", secs)
        } else {
            format!("{}", secs)
        };

        let hours_str = if hours == 0 {
            String::new()
        } else if hours < 10 {
            format!("0{}:", hours)
        } else {
            format!("{}:", hours)
        };

        format!("{}{}{}", hours_str, minutes_str, seconds_str)
    }

    fn from_human_readable(query: String) -> Option<Self> {
        let query_vec = query
            .split(":")
            .map(|s| match s.parse::<i64>() {
                Ok(i) => i,
                Err(_) => -1,
            })
            .collect::<Vec<i64>>();

        let count = query_vec.len();

        for i in &query_vec {
            if *i == -1 {
                return None;
            }
        }

        if count == 1 {
            Some(Duration::from_secs(
                (query_vec.into_iter().next().unwrap()) as u64,
            ))
        } else if count == 2 {
            let mut iter = query_vec.into_iter();
            Some(Duration::from_secs(
                (iter.next().unwrap() * 60 + iter.next().unwrap()) as u64,
            ))
        } else if count == 3 {
            let mut iter = query_vec.into_iter();
            Some(Duration::from_secs(
                (iter.next().unwrap() * 3600 + iter.next().unwrap() * 60 + iter.next().unwrap())
                    as u64,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::utils::*;
    use std::time::Duration;
    #[test]
    fn duration_parser() {
        let parsed = Duration::from_human_readable(String::from("23:17")).unwrap();
        assert_eq!(parsed.as_secs(), 23 * 60 + 17);

        let parsed = Duration::from_human_readable(String::from("06:23:17")).unwrap();
        assert_eq!(parsed.as_secs(), 06 * 3600 + 23 * 60 + 17);

        let parsed = Duration::from_human_readable(String::from("17")).unwrap();
        assert_eq!(parsed.as_secs(), 17);

        let parsed = Duration::from_human_readable(String::from("07")).unwrap();
        assert_eq!(parsed.as_secs(), 7);

        let parsed = Duration::from_human_readable(String::from("02:57")).unwrap();
        assert_eq!(parsed.as_secs(), 2 * 60 + 57);
    }

    #[test]
    fn yt_pl() {
        let o = YoutubeDl::new(
            "https://www.youtube.com/watch?v=GPGdXrQID7c&list=PLDEF999E346D88D70&index=5",
        )
        .run()
        .unwrap();

        match o {
            YoutubeDlOutput::SingleVideo(_) => {
                println!("v");
            }
            YoutubeDlOutput::Playlist(p) => {
                println!("p");
                let _ = p
                    .entries
                    .unwrap()
                    .iter()
                    .map(|v| println!("{}", v.title))
                    .collect::<Vec<()>>();
            }
        }
    }
}
