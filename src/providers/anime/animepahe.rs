use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeEpisode, IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct AnimePahe;

#[derive(Debug)]
pub enum AnimePaheError {}

const BASE_URL: &str = "https://animepahe.com";

impl<'a> AnimeParser<'a> for AnimePahe {
    type AnimeError = AnimePaheError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>,
    ) -> Result<ISearch<IAnimeResult>, <AnimePahe as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id, page = -1
    ) -> Result<IAnimeInfo, <AnimePahe as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id | episode_link
    ) -> Result<Vec<IEpisodeServer>, <AnimePahe as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <AnimePahe as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl AnimePahe {
    async fn fetch_episodes(&self, _session: &str, _page: usize) -> Vec<IAnimeEpisode> {
        todo!()
    }
}
