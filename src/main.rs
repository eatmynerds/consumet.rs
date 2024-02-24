use consumet::{
    models::StreamingServers,
    providers::movies,
    providers::movies::flixhq::{FlixHQInfo, FlixHQStreamingServers},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new instance of the FlixHQ provider
    let flixhq = movies::FlixHQ;

    // Search for a movie. In this case, "Vincenzo"
    let data = flixhq.search("Vincenzo", None).await?;

    let movie_id = &data.results[0].id;

    let movie_info = flixhq.info(&movie_id).await?;

    match movie_info {
        FlixHQInfo::Tv(show) => {
            let media_id = show.id;
            // Get the first episodes id
            let episode_id = &show.seasons.episodes[0][0].id;

            let servers = flixhq.servers(&episode_id, &media_id).await?;

            let chosen_server = match servers.servers[0].name.as_str() {
                "UpCloud" => StreamingServers::UpCloud,
                "VidCloud" => StreamingServers::VidCloud,
                _ => todo!()
            };

            // Get the video sources from UpCloud
            let sources = flixhq
                .sources(&episode_id, &media_id, Some(chosen_server))
                .await?;

            match sources.sources {
                FlixHQStreamingServers::VidCloud(embed_links) => {
                    println!("{:#?}", embed_links);
                }
                FlixHQStreamingServers::MixDrop(_) => {}
            }
        }
        FlixHQInfo::Movie(movie) => {
            let media_id = movie.id;

            let episode_id = media_id.rsplit("-").collect::<Vec<&str>>()[0];

            let servers = flixhq.servers(episode_id, &media_id).await?;

            let chosen_server = match servers.servers[0].name.as_str() {
                "UpCloud" => StreamingServers::UpCloud,
                "VidCloud" => StreamingServers::VidCloud,
                _ => todo!()
            };

            let sources = flixhq
                .sources(episode_id, &media_id, Some(chosen_server))
                .await?;

            match sources.sources {
                FlixHQStreamingServers::VidCloud(embed_links) => {
                    println!("{:#?}", embed_links);
                }
                FlixHQStreamingServers::MixDrop(_) => {}
            }
        }
    }

    Ok(())
}
