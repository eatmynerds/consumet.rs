use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct AnimeSaturn;

#[derive(Debug)]
pub enum AnimeSaturnError {}

const BASE_URL: &str = "https://animesaturn.tv";

impl<'a> AnimeParser<'a> for AnimeSaturn {
    type AnimeError = AnimeSaturnError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query
    ) -> Result<ISearch<IAnimeResult>, <AnimeSaturn as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id
    ) -> Result<IAnimeInfo, <AnimeSaturn as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <AnimeSaturn as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <AnimeSaturn as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}
