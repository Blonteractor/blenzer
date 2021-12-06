use log::{error, warn};
use serenity::builder::CreateEmbed;
use serenity::prelude::*;
use serenity::utils::Color;
use songbird::{
    input::{Input, Restartable},
    Call,
};
use std::sync::Arc;
use youtube_dl::SearchOptions;
use youtube_dl::SingleVideo;
use youtube_dl::YoutubeDl;
use youtube_dl::YoutubeDlOutput;

pub fn get_song(url: impl ToString) -> Result<SingleVideo, youtube_dl::Error> {
    let video = if let YoutubeDlOutput::SingleVideo(video) = YoutubeDl::new(&url.to_string())
        .extract_audio(false)
        .run()?
    {
        *video
    } else {
        warn!("Song embed requested for playlist url");
        unreachable!();
    };

    Ok(video)
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

pub fn song_embed(video: SingleVideo) -> Result<CreateEmbed, youtube_dl::Error> {
    Ok(CreateEmbed::default()
        .image(video.thumbnail.unwrap_or_default())
        .title("Song added to queue")
        .description(format!("**#1** \n *{}*", video.title))
        .url(video.webpage_url.unwrap_or_default())
        .color(Color::from_rgb(4, 105, 207))
        .to_owned())
}

pub async fn play_song(
    url: String,
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
