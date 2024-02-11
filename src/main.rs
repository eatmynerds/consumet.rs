use consumet::{
    extractors::{MixDrop, VidCloud},
    models::StreamingServers,
    providers::movies,
    providers::movies::flixhq::{FlixHQSearchResult, FlixHQStreamingServers},
};

use std::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new instance of the FlixHQ provider
    let flixhq = movies::FlixHQ;
    // Search for a movie. In this case, "Vincenzo"
    // let data = flixhq.search("Vincenzo", None).await?;
    // let movie_id = &data.results[0].id;

    let movie_id = "tv/watch-rick-and-morty-39480";

    let movie_info = flixhq.info(&movie_id).await?;

    match movie_info {
        FlixHQSearchResult::Tv(show) => {
            let media_id = show.id;
            // Get the first episodes id
            let episode_id = &show.seasons.episodes[0][0].id;

            let servers = flixhq.servers(&episode_id, &media_id).await?;

            // Get the video sources from vidcloud
            let sources = flixhq
                .sources(&episode_id, &media_id, Some(StreamingServers::VidCloud))
                .await?;

            match sources.sources {
                FlixHQStreamingServers::VidCloud(embed_links) => {
                    let _ = Command::new("mpv")
                        .arg(&embed_links[0].url)
                        .spawn()
                        .unwrap();
                }
                FlixHQStreamingServers::MixDrop(_) => {}
            }
        }
        FlixHQSearchResult::Movie(movie) => {}
    }

    Ok(())
}
