use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct CrunchyRoll;

#[derive(Debug)]
pub enum CrunchyRollError {}

const BASE_URL: &str = "https://cronchy.consumet.stream";

impl<'a> AnimeParser<'a> for CrunchyRoll {
    type AnimeError = CrunchyRollError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query
    ) -> Result<ISearch<IAnimeResult>, <CrunchyRoll as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id | media_type | fetch_all_seasons = false
    ) -> Result<IAnimeInfo, <CrunchyRoll as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <CrunchyRoll as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <CrunchyRoll as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}
