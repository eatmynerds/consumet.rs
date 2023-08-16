use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct AnimeFox;

#[derive(Debug)]
pub enum AnimeFoxError {}

const BASE_URL: &str = "https://animefox.tv";

impl<'a> AnimeParser<'a> for AnimeFox {
    type AnimeError = AnimeFoxError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IAnimeResult>, <AnimeFox as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id | anime_url
    ) -> Result<IAnimeInfo, <AnimeFox as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <AnimeFox as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <AnimeFox as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl AnimeFox {
    async fn fetch_recent_episodes(
        &self,
        _page: Option<usize>, /* page = 1 */
    ) -> ISearch<IAnimeResult> {
        todo!()
    }
}
