use crate::{
    extractors::{
        MixDrop, MixDropSource, MixDropSubtitle, VidCloud, VidCloudSource, VidCloudSubtitle,
    },
    html::movies::flixhq_html::FlixHQHTML,
    models::{ExtractConfig, StreamingServers, TvType, VideoExtractor},
};

use serde::{Deserialize, Serialize};

/// Contains all the FlixHQ Info
pub struct FlixHQ;

#[derive(Debug, Deserialize, Serialize)]
pub enum FlixHQStreamingServers {
    VidCloud(Vec<VidCloudSource>),
    MixDrop(Vec<MixDropSource>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FlixHQSubtitles {
    VidCloud(Vec<VidCloudSubtitle>),
    MixDrop(Vec<MixDropSubtitle>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQSource {
    pub headers: String,
    pub subtitles: FlixHQSubtitles,
    pub sources: FlixHQStreamingServers,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServers {
    pub servers: Vec<FlixHQServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServer {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlixHQSeason {
    pub total_seasons: usize,
    pub episodes: Vec<Vec<FlixHQEpisode>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlixHQEpisode {
    pub id: String,
    pub title: String,
    pub url: String,
}

/// Contains Search Results
#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQSearchResults {
    pub current_page: usize,
    pub has_next_page: bool,
    pub total_pages: usize,
    pub total_results: usize,
    pub results: Vec<FlixHQMovieResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FlixHQSearchResult {
    Tv(FlixHQShowResult),
    Movie(FlixHQMovieResult),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQMovieResult {
    pub id: String,
    pub cover: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub release_date: String,
    pub media_type: TvType,
    pub genres: Vec<String>,
    pub description: String,
    pub rating: String,
    pub quality: String,
    pub duration: String,
    pub country: Vec<String>,
    pub production: Vec<String>,
    pub casts: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQShowResult {
    pub id: String,
    pub cover: String,
    pub title: String,
    pub url: String,
    pub image: String,
    pub release_date: String,
    pub media_type: TvType,
    pub genres: Vec<String>,
    pub description: String,
    pub rating: String,
    pub quality: String,
    pub duration: String,
    pub country: Vec<String>,
    pub production: Vec<String>,
    pub casts: Vec<String>,
    pub tags: Vec<String>,
    pub total_episodes: usize,
    pub seasons: FlixHQSeason,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServerInfo {
    link: String,
}

pub const BASE_URL: &'static str = "https://flixhq.to";

impl FlixHQ {
    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<FlixHQSearchResults> {
        let current_page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = reqwest::Client::new()
            .get(format!(
                "{}/search/{}?page={}",
                BASE_URL, parsed_query, current_page
            ))
            .send()
            .await?
            .text()
            .await?;

        let (ids, has_next_page, total_pages) = self.parse_search(page_html);

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let url = format!("{}/{}", BASE_URL, id);
            let media_html = reqwest::Client::new()
                .get(&url)
                .send()
                .await?
                .text()
                .await?;

            let result = self.single_page(media_html, id, url);

            results.push(result);
        }

        Ok(FlixHQSearchResults {
            current_page,
            has_next_page,
            total_pages,
            total_results: results.len(),
            results,
        })
    }

    /// Returns a future which resolves into an movie info object (including the episodes). (*[`impl Future<Output = Result<FlixHQInfo>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/providers/movies/flixhq.rs#L22-L26)*)\
    /// # Parameters
    /// * `media_id` - takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*)
    pub async fn info(&self, media_id: &str) -> anyhow::Result<FlixHQSearchResult> {
        let info_html = reqwest::Client::new()
            .get(format!("{}/{}", BASE_URL, media_id))
            .send()
            .await?
            .text()
            .await?;

        let search_result =
            self.single_page(info_html, media_id, format!("{}/{}", BASE_URL, media_id));

        let media_type = search_result.media_type;
        let is_seasons = matches!(media_type, TvType::TvSeries);

        if is_seasons {
            let id = media_id.split('-').last().unwrap_or_default().to_owned();

            let season_html = reqwest::Client::new()
                .get(format!("{}/ajax/v2/tv/seasons/{}", BASE_URL, id))
                .send()
                .await?
                .text()
                .await?;

            let season_ids = self.info_season(season_html);

            let mut seasons_and_episodes = vec![];

            for (i, episode) in season_ids.iter().enumerate() {
                let episode_html = reqwest::Client::new()
                    .get(format!("{}/ajax/v2/season/episodes/{}", BASE_URL, &episode))
                    .send()
                    .await?
                    .text()
                    .await?;

                let episodes = self.info_episode(episode_html, i);
                seasons_and_episodes.push(episodes.episodes);
            }

            Ok(FlixHQSearchResult::Tv(FlixHQShowResult {
                total_episodes: seasons_and_episodes.last().map(|x| x.len()).unwrap(),
                seasons: FlixHQSeason {
                    total_seasons: seasons_and_episodes.len(),
                    episodes: seasons_and_episodes.clone(),
                },
                id: search_result.id,
                cover: search_result.cover,
                title: search_result.title,
                url: search_result.url,
                image: search_result.image,
                release_date: search_result.release_date,
                media_type: search_result.media_type,
                genres: search_result.genres,
                description: search_result.description,
                rating: search_result.rating,
                quality: search_result.quality,
                duration: search_result.duration,
                country: search_result.country,
                production: search_result.production,
                casts: search_result.casts,
                tags: search_result.tags,
            }))
        } else {
            Ok(FlixHQSearchResult::Movie(search_result))
        }
    }

    /// Returns a future which resolves into an vector of episode servers. (*[`impl Future<Output = Result<Vec<IEpisodeServer>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L135-L146)*)\
    /// # Parameters
    /// * `episode_id` - take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*
    pub async fn servers(&self, episode_id: &str, media_id: &str) -> anyhow::Result<FlixHQServers> {
        let episode_id = format!(
            "{}/ajax/{}",
            BASE_URL,
            if !episode_id.starts_with(&format!("{}/ajax", BASE_URL)) && !media_id.contains("movie")
            {
                format!("v2/episode/servers/{}", episode_id)
            } else {
                format!("movie/episodes/{}", episode_id)
            }
        );

        let server_html = reqwest::Client::new()
            .get(episode_id)
            .send()
            .await?
            .text()
            .await?;

        let servers = self.info_server(server_html, media_id);

        Ok(FlixHQServers { servers })
    }

    /// Returns a future which resolves into an vector of episode sources and subtitles. (*[`impl Future<Output = Result<ISource>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L374-L379)*)\
    /// # Parameters
    /// * `episode_id` - takes episode id as a parameter. (*episode id can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*)
    /// * `server (optional)` - [`StreamingServers`]
    pub async fn sources(
        &self,
        episode_id: &str,
        media_id: &str,
        server: Option<StreamingServers>,
    ) -> anyhow::Result<FlixHQSource> {
        let server: StreamingServers = server.unwrap_or(StreamingServers::UpCloud);
        let servers = self.servers(episode_id, media_id).await?;

        let i = match servers
            .servers
            .iter()
            .position(|s| s.name == server.to_string())
        {
            Some(index) => index,
            None => 0,
        };

        let parts = &servers.servers[i].url;

        let server_id: &str = parts
            .split('.')
            .collect::<Vec<_>>()
            .last()
            .copied()
            .unwrap_or_default();

        let server_json = reqwest::Client::new()
            .get(format!("{}/ajax/get_link/{}", BASE_URL, server_id))
            .send()
            .await?
            .text()
            .await?;

        let server_info: FlixHQServerInfo = serde_json::from_str(&server_json)?;

        match server {
            StreamingServers::MixDrop => {
                let mut mix_drop = MixDrop {
                    sources: vec![],
                    subtitles: vec![],
                };

                mix_drop
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSource {
                    sources: FlixHQStreamingServers::MixDrop(mix_drop.sources),
                    subtitles: FlixHQSubtitles::MixDrop(mix_drop.subtitles),
                    headers: server_info.link,
                })
            }
            StreamingServers::VidCloud => {
                let mut vid_cloud = VidCloud {
                    sources: vec![],
                    subtitles: vec![],
                };

                vid_cloud
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            is_alternative: Some(true),
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSource {
                    sources: FlixHQStreamingServers::VidCloud(vid_cloud.sources),
                    subtitles: FlixHQSubtitles::VidCloud(vid_cloud.subtitles),
                    headers: server_info.link,
                })
            }
            StreamingServers::UpCloud => {
                let mut vid_cloud = VidCloud {
                    sources: vec![],
                    subtitles: vec![],
                };

                vid_cloud
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSource {
                    sources: FlixHQStreamingServers::VidCloud(vid_cloud.sources),
                    subtitles: FlixHQSubtitles::VidCloud(vid_cloud.subtitles),
                    headers: server_info.link,
                })
            }
            _ => {
                let mut vid_cloud = VidCloud {
                    sources: vec![],
                    subtitles: vec![],
                };

                vid_cloud
                    .extract(
                        server_info.link.clone(),
                        ExtractConfig {
                            is_alternative: Some(false),
                            ..Default::default()
                        },
                    )
                    .await?;

                Ok(FlixHQSource {
                    sources: FlixHQStreamingServers::VidCloud(vid_cloud.sources),
                    subtitles: FlixHQSubtitles::VidCloud(vid_cloud.subtitles),
                    headers: server_info.link,
                })
            }
        }
    }

    /// Returns a future which resolves into an vector of movie results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn recent_movies(&self) -> anyhow::Result<Vec<FlixHQMovieResult>> {
        let recent_html = reqwest::Client::new()
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_recent_movies(recent_html);

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let media_html = reqwest::Client::new()
                .get(format!("{}/{}", BASE_URL, id))
                .send()
                .await?
                .text()
                .await?;

            let result = self.single_page(media_html, id, format!("{}/{}", BASE_URL, id));

            results.push(result);
        }

        Ok(results)
    }

    /// Returns a future which resolves into an vector of tv shows results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn recent_shows(&self) -> anyhow::Result<Vec<FlixHQMovieResult>> {
        let recent_html = reqwest::Client::new()
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_recent_shows(recent_html);

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let media_html = reqwest::Client::new()
                .get(format!("{}/{}", BASE_URL, id))
                .send()
                .await?
                .text()
                .await?;

            let result = self.single_page(media_html, id, format!("{}/{}", BASE_URL, id));

            results.push(result);
        }

        Ok(results)
    }

    /// Returns a future which resolves into an vector of movie results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn trending_movies(&self) -> anyhow::Result<Vec<FlixHQMovieResult>> {
        let trending_html = reqwest::Client::new()
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_trending_movies(trending_html);

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let media_html = reqwest::Client::new()
                .get(format!("{}/{}", BASE_URL, id))
                .send()
                .await?
                .text()
                .await?;

            let result = self.single_page(media_html, id, format!("{}/{}", BASE_URL, id));

            results.push(result);
        }

        Ok(results)
    }

    /// Returns a future which resolves into an vector of tv shows results  (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)
    /// # Parameters
    /// * `None`
    pub async fn trending_shows(&self) -> anyhow::Result<Vec<FlixHQMovieResult>> {
        let trending_html = reqwest::Client::new()
            .get(format!("{}/home", BASE_URL))
            .send()
            .await?
            .text()
            .await?;

        let ids = self.parse_trending_shows(trending_html);

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let media_html = reqwest::Client::new()
                .get(format!("{}/{}", BASE_URL, id))
                .send()
                .await?
                .text()
                .await?;

            let result = self.single_page(media_html, id, format!("{}/{}", BASE_URL, id));

            results.push(result);
        }

        Ok(results)
    }
}
