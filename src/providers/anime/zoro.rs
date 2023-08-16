use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct Zoro;

#[derive(Debug)]
pub enum ZoroError {}

const BASE_URL: &str = "https://aniwatch.to";

impl<'a> AnimeParser<'a> for Zoro {
    type AnimeError = ZoroError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IAnimeResult>, <Zoro as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id
    ) -> Result<IAnimeInfo, <Zoro as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <Zoro as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id, server = StreamingServers::VidCloud
    ) -> Result<ISource, <Zoro as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl Zoro {
    async fn fetch_recent_episodes(&self, _page: usize) -> ISearch<IAnimeResult> {
        todo!()
    }
}
