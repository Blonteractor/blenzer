use log::error;
use serenity::{builder::CreateEmbed, prelude::*, utils::Color};
use songbird::{
    input::{self, Input, Restartable},
    tracks::TrackHandle,
    Call,
};
use std::sync::Arc;
use youtube_dl::{SearchOptions, SingleVideo, YoutubeDl, YoutubeDlOutput};

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

pub fn search_songs(
    query: impl ToString,
    limit: usize,
) -> Result<Vec<SingleVideo>, youtube_dl::Error> {
    let ytdl = YoutubeDl::search_for(&SearchOptions::youtube(&query.to_string()).with_count(limit))
        .extract_audio(false)
        .run()?;

    if let YoutubeDlOutput::Playlist(search_result) = ytdl {
        let videos = search_result.entries.unwrap_or_default();

        return Ok(videos);
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

pub fn song_embed(
    track_handle: &TrackHandle,
    position: usize,
) -> Result<CreateEmbed, youtube_dl::Error> {
    let metadata = track_handle.metadata();
    Ok(CreateEmbed::default()
        .image(metadata.thumbnail.as_ref().unwrap_or(&String::default()))
        .title("Song added to queue")
        .description(format!(
            "**#{}** \n *{}*",
            position,
            metadata.title.as_ref().unwrap_or(&String::default())
        ))
        .url(metadata.source_url.as_ref().unwrap_or(&String::default()))
        .color(Color::from_rgb(4, 105, 207))
        .to_owned())
}

pub async fn play_song_now(
    url: String,
    handler_lock: Arc<Mutex<Call>>,
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
    handler_lock: &Arc<Mutex<Call>>,
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

pub struct SongRequestedBy;

impl TypeMapKey for SongRequestedBy {
    type Value = String;
}
