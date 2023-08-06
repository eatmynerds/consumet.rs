use super::flixhq_html::{
    create_html_fragment, Episodes, Info, Page, Recent, Search, Seasons, Server, Trending,
};

use crate::models::{
    BaseProvider, ExtractConfig, IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult,
    IMovieSeason, ISearch, ISource, StreamingServers, TvType, VideoExtractor,
};

use crate::extractors::{MixDrop, VidCloud};

use serde::{Deserialize, Serialize};

/// Contains all the FlixHQ Info
pub struct FlixHQ;

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQServerInfo {
    link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlixHQInfo {
    pub base: IMovieResult,
    pub info: IMovieInfo,
}

impl BaseProvider for FlixHQ {
    #[inline]
    fn name(&self) -> &str {
        "FlixHQ"
    }

    #[inline]
    fn base_url(&self) -> &str {
        "https://flixhq.to"
    }

    #[inline]
    fn logo(&self) -> &str {
        "https://upload.wikimedia.org/wikipedia/commons/7/7a/MyAnimeList_Logo.png"
    }

    #[inline]
    fn class_path(&self) -> &str {
        "MOVIES.FlixHQ"
    }
}

impl FlixHQ {
    pub(crate) fn parse_recent_shows(&self, recent_html: String) -> Vec<Option<String>> {
        let fragment = create_html_fragment(&trending_html);

        let trending_parser = Recent { elements: fragment };

        trending_parser.recent_shows()
    }

    pub(crate) fn parse_recent_movies(&self, recent_html: String) -> Vec<Option<String>> {
        let fragment = create_html_fragment(&trending_html);

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

    pub(crate) fn single_page(&self, media_html: String, id: &str, url: String) -> IMovieResult {
        let fragment = create_html_fragment(&media_html);

        let search_parser = Search {
            elements: &fragment,
            id,
        };

        let info_parser = Info {
            elements: &fragment,
        };

        IMovieResult {
            cover: search_parser.search_cover(),
            title: search_parser.search_title(),
            other_names: None,
            url: Some(url),
            image: search_parser.search_image(),
            release_date: info_parser.info_label(3, "Released:").join(""),
            media_type: search_parser.search_media_type(),
            id: Some(id.to_string()),
        }
    }

    pub(crate) fn info_page(&self, info_html: String, search_result: IMovieResult) -> FlixHQInfo {
        let fragment = create_html_fragment(&info_html);

        let info_parser = Info {
            elements: &fragment,
        };

        FlixHQInfo {
            base: search_result,
            info: IMovieInfo {
                genres: Some(info_parser.info_label(2, "Genre:")),
                description: info_parser.info_description(),
                quality: info_parser.info_quality(),
                rating: info_parser.info_rating(),
                status: None,
                duration: info_parser.info_duration(),
                country: Some(info_parser.info_label(1, "Country:")),
                production: Some(info_parser.info_label(4, "Production:")),
                casts: Some(info_parser.info_label(5, "Casts:")),
                tags: Some(info_parser.info_label(6, "Tags:")),
                total_episodes: None,
                seasons: None,
                episodes: None,
            },
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

        Episodes::episode_results(fragment, self.base_url(), index)
    }

    pub(crate) fn info_server(&self, server_html: String, media_id: &str) -> Vec<IEpisodeServer> {
        let fragment = create_html_fragment(&server_html);

        let server_parser = Server { element: fragment };

        server_parser
            .parse_server_html(self.base_url(), media_id)
            .into_iter()
            .flatten()
            .collect()
    }

    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<IMovieResult>> {
        let page = page.unwrap_or(1);

        let parsed_query = query.replace(' ', "-");
        let page_html = reqwest::Client::new()
            .get(format!(
                "{}/search/{}?page={}",
                self.base_url(),
                parsed_query,
                page
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
    pub async fn fetch_search_result(&self, id: &str) -> anyhow::Result<IMovieResult> {
        let url = format!("{}/{}", self.base_url(), id);

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
    pub async fn info(&self, media_id: &str) -> anyhow::Result<FlixHQInfo> {
        let search_result = self.fetch_search_result(media_id).await?;

        let media_type = search_result.media_type.unwrap();
        let is_seasons = matches!(media_type, TvType::TvSeries);

        let info_html = reqwest::Client::new()
            .get(format!("{}/{}", self.base_url(), media_id))
            .send()
            .await?
            .text()
            .await?;

        let info = self.info_page(info_html, search_result);

        if is_seasons {
            let id = media_id.split('-').last().unwrap_or_default().to_owned();

            let season_html = reqwest::Client::new()
                .get(format!("{}/ajax/v2/tv/seasons/{}", self.base_url(), id))
                .send()
                .await?
                .text()
                .await?;

            let season_ids = self.info_season(season_html);

            let mut seasons_and_episodes: Vec<Vec<IMovieEpisode>> = vec![];

            for (i, episode) in season_ids.iter().enumerate() {
                let episode_html = reqwest::Client::new()
                    .get(format!(
                        "{}/ajax/v2/season/episodes/{}",
                        self.base_url(),
                        &episode
                    ))
                    .send()
                    .await?
                    .text()
                    .await?;

                let episodes = self.info_episode(episode_html, i);
                seasons_and_episodes.push(episodes.episodes);
            }

            Ok(FlixHQInfo {
                base: info.base,
                info: IMovieInfo {
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
                    ..info.info
                },
            })
        } else {
            Ok(FlixHQInfo {
                base: info.base,
                info: IMovieInfo {
                    total_episodes: None,
                    seasons: None,
                    episodes: None,
                    ..info.info
                },
            })
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
            self.base_url(),
            if !episode_id.starts_with(&format!("{}/ajax", self.base_url()))
                && !media_id.contains("movie")
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
            .get(format!("{}/ajax/get_link/{}", self.base_url(), server_id))
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
    pub async fn recent_movies(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let recent_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
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
    pub async fn recent_shows(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let recent_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
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
    pub async fn trending_movies(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let trending_movies_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
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
    pub async fn trending_shows(&self) -> anyhow::Result<Vec<IMovieResult>> {
        let trending_shows_html = reqwest::Client::new()
            .get(format!("{}/home", self.base_url()))
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
