use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct Marin;

#[derive(Debug)]
pub enum MarinError {}

impl<'a> AnimeParser<'a> for Marin {
    type AnimeError = MarinError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IAnimeResult>, <Marin as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id
    ) -> Result<IAnimeInfo, <Marin as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <Marin as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <Marin as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl Marin {
    async fn recent_episodes(&self, _page: usize) -> ISearch<IAnimeInfo> {
        todo!()
    }
}
