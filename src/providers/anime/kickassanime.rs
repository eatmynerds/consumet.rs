use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct KickAssAnime;

#[derive(Debug)]
pub enum KickAssAnimeError {}

const BASE_URL: &str = "https://kickassanime.am";

impl<'a> AnimeParser<'a> for KickAssAnime {
    type AnimeError = KickAssAnimeError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query
    ) -> Result<ISearch<IAnimeResult>, <KickAssAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id
    ) -> Result<IAnimeInfo, <KickAssAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <KickAssAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <KickAssAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}
