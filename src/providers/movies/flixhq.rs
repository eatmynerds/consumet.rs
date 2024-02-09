use super::flixhq_html::{
    create_html_fragment, Episodes, Info, Page, Recent, Search, Seasons, Server, Trending,
};

use crate::models::{
    ExtractConfig, IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult, IMovieSeason, ISearch,
    ISource, StreamingServers, TvType, VideoExtractor,
};

use crate::extractors::{MixDrop, VidCloud};

use serde::{Deserialize, Serialize};

/// Contains all the FlixHQ Info
pub struct FlixHQ;

#[derive(Debug, Deserialize, Serialize)]
pub enum FlixhqSearchResult {
    Tv(FlixHQShowResult),
    Movie(FlixHQMovieResult),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQMovieResult {
    pub id: String,
    pub cover: Option<String>,
    pub title: String,
    pub url: String,
    pub image: Option<String>,
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
    pub cover: Option<String>,
    pub title: String,
    pub url: String,
    pub image: Option<String>,
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
    pub total_episodes: Option<usize>,
    pub seasons: Option<IMovieSeason>,
    pub episodes: Option<Vec<Vec<IMovieEpisode>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServerInfo {
    link: String,
}

const BASE_URL: &'static str = "https://flixhq.to";

impl FlixHQ {
    pub(crate) fn parse_recent_shows(&self, recent_html: String) -> Vec<Option<String>> {
        let fragment = create_html_fragment(&recent_html);

        let trending_parser = Recent { elements: fragment };

        trending_parser.recent_shows()
    }

    pub(crate) fn parse_recent_movies(&self, recent_html: String) -> Vec<Option<String>> {
        let fragment = create_html_fragment(&recent_html);

        let trending_parser = Recent { elements: fragment };

        trending_parser.recent_movies()
    }

    pub(crate) fn parse_trending_movies(&self, trending_html: String) -> Vec<Option<String>> {
        let fragment = create_html_fragment(&trending_html);

        let trending_parser = Trending { elements: fragment };

        trending_parser.trending_movies()
    }

    pub(crate) fn parse_trending_shows(&self, trending_html: String) -> Vec<Option<String>> {
        let fragment = create_html_fragment(&trending_html);

        let trending_parser = Trending { elements: fragment };

        trending_parser.trending_shows()
    }

    pub(crate) fn parse_search(
        &self,
        page_html: String,
    ) -> (Vec<Option<String>>, bool, Option<usize>) {
        let fragment = create_html_fragment(&page_html);

        let page_parser = Page { elements: fragment };

        let ids = page_parser.page_ids();
        (ids, page_parser.has_next_page(), page_parser.total_pages())
    }

    pub(crate) fn single_page(
        &self,
        media_html: String,
        id: &str,
        url: String,
    ) -> FlixHQMovieResult {
        let fragment = create_html_fragment(&media_html);

        let search_parser = Search {
            elements: &fragment,
            id,
        };

        let info_parser = Info {
            elements: &fragment,
        };

        FlixHQMovieResult {
            cover: search_parser.search_cover(),
            title: search_parser.search_title(),
            url,
            image: search_parser.search_image(),
            release_date: info_parser.info_label(3, "Released:").join(""),
            media_type: search_parser.search_media_type(),
            id: id.to_string(),
            genres: info_parser.info_label(2, "Genre:"),
            description: info_parser.info_description(),
            quality: info_parser.info_quality(),
            rating: info_parser.info_rating(),
            duration: info_parser.info_duration(),
            country: info_parser.info_label(1, "Country:"),
            production: info_parser.info_label(4, "Production:"),
            casts: info_parser.info_label(5, "Casts:"),
            tags: info_parser.info_label(6, "Tags:"),
        }
    }

    pub(crate) fn info_season(&self, season_html: String) -> Vec<String> {
        let fragment = create_html_fragment(&season_html);

        let season_parser = Seasons { elements: fragment };

        season_parser
            .season_results()
            .into_iter()
            .flatten()
            .collect()
    }

    pub(crate) fn info_episode(&self, episode_html: String, index: usize) -> Episodes {
        let fragment = create_html_fragment(&episode_html);

        Episodes::episode_results(fragment, BASE_URL, index)
    }

    pub(crate) fn info_server(&self, server_html: String, media_id: &str) -> Vec<IEpisodeServer> {
        let fragment = create_html_fragment(&server_html);

        let server_parser = Server { element: fragment };

        server_parser
            .parse_server_html(BASE_URL, media_id)
            .into_iter()
            .flatten()
            .collect()
    }

    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<FlixHQMovieResult>> {
        let page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = reqwest::Client::new()
            .get(format!(
                "{}/search/{}?page={}",
                BASE_URL, parsed_query, page
            ))
            .send()
            .await?
            .text()
            .await?;

        let (ids, has_next_page, total_pages) = self.parse_search(page_html);

        let mut results = vec![];

        for id in ids.iter().flatten() {
            let result = self.fetch_search_result(id).await?;

            results.push(result);
        }

        Ok(ISearch {
            current_page: Some(page),
            has_next_page,
            total_pages,
            total_results: results.len(),
            results,
        })
    }

    /// Returns a future which resolves into an movie result object (*[`impl Future<Output = Result<IMovieResult>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
    /// # Parameters
    /// * `id` - the id of a movie or show
    pub async fn fetch_search_result(&self, id: &str) -> anyhow::Result<FlixHQMovieResult> {
        let url = format!("{}/{}", BASE_URL, id);

        let media_html = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .text()
            .await?;

        Ok(self.single_page(media_html, id, url))
    }

    /// Returns a future which resolves into an movie info object (including the episodes). (*[`impl Future<Output = Result<FlixHQInfo>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/providers/movies/flixhq.rs#L22-L26)*)\
    /// # Parameters
    /// * `media_id` - takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*)
    pub async fn info(&self, media_id: &str) -> anyhow::Result<FlixhqSearchResult> {
        let search_result = self.fetch_search_result(media_id).await?;

        let media_type = search_result.media_type;
        let is_seasons = matches!(media_type, TvType::TvSeries);

        let info_html = reqwest::Client::new()
            .get(format!("{}/{}", BASE_URL, media_id))
            .send()
            .await?
            .text()
            .await?;

        if is_seasons {
            let id = media_id.split('-').last().unwrap_or_default().to_owned();

            let season_html = reqwest::Client::new()
                .get(format!("{}/ajax/v2/tv/seasons/{}", BASE_URL, id))
                .send()
                .await?
                .text()
                .await?;

            let season_ids = self.info_season(season_html);

            let mut seasons_and_episodes: Vec<Vec<IMovieEpisode>> = vec![];

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

            Ok(FlixhqSearchResult::Tv(FlixHQShowResult {
                total_episodes: seasons_and_episodes.last().map(|x| x.len()),
                seasons: Some(IMovieSeason {
                    season: Some(
                        seasons_and_episodes
                            .last()
                            .and_then(|x| x.last())
                            .and_then(|y| y.season)
                            .unwrap_or(0),
                    ),
                    image: None,
                    episodes: Some(seasons_and_episodes.clone()),
                }),
                episodes: Some(seasons_and_episodes),
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
            Ok(FlixhqSearchResult::Movie(search_result))
        }
    }

    /// Returns a future which resolves into an vector of episode servers. (*[`impl Future<Output = Result<Vec<IEpisodeServer>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L135-L146)*)\
    /// # Parameters
    /// * `episode_id` - take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*)
    /// * `media_id` - takes media id as a parameter. (*media id can be found in the media info object*
    pub async fn servers(
        &self,
        episode_id: &str,
        media_id: &str,
    ) -> anyhow::Result<Vec<IEpisodeServer>> {
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

        Ok(servers)
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
    ) -> anyhow::Result<ISource> {
        let server: StreamingServers = server.unwrap_or(StreamingServers::UpCloud);
        let servers = self.servers(episode_id, media_id).await?;

        let i = match servers
            .iter()
            .position(|s| s.name == Some(server.to_string()))
        {
            Some(index) => index,
            None => 0,
        };

        let parts = servers[i].url.clone().unwrap_or_default();

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

                Ok(ISource {
                    sources: Some(mix_drop.sources),
                    subtitles: Some(mix_drop.subtitles),
                    headers: Some(server_info.link),
                    intro: None,
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

                Ok(ISource {
                    sources: Some(vid_cloud.sources),
                    subtitles: Some(vid_cloud.subtitles),
                    headers: Some(server_info.link),
                    intro: None,
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

                Ok(ISource {
                    sources: Some(vid_cloud.sources),
                    subtitles: Some(vid_cloud.subtitles),
                    headers: Some(server_info.link),
                    intro: None,
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

                Ok(ISource {
                    sources: Some(vid_cloud.sources),
                    subtitles: Some(vid_cloud.subtitles),
                    headers: Some(server_info.link),
                    intro: None,
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
            let result = self.fetch_search_result(id).await?;

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
            let result = self.fetch_search_result(id).await?;

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
            let result = self.fetch_search_result(id).await?;

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
            let result = self.fetch_search_result(id).await?;

            results.push(result);
        }

        Ok(results)
    }
}
