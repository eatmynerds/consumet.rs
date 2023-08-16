use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct NineAnime;

#[derive(Debug)]
pub enum NineAnimeError {}

const BASE_URL: &str = "https://9anime.pl";

impl<'a> AnimeParser<'a> for NineAnime {
    type AnimeError = NineAnimeError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, /* query, page = 1 */
    ) -> Result<ISearch<IAnimeResult>, <NineAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id | anime_url
    ) -> Result<IAnimeInfo, <NineAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <NineAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id, server = StreamingServers::VizCloud
    ) -> Result<ISource, <NineAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl NineAnime {
    async fn ev(&self, _query: &str, _raw: Option<bool>) -> String {
        todo!()
    }

    async fn search_vrf(&self, _query: &str, _raw: Option<bool>) -> String {
        todo!()
    }

    async fn decrypt(&self, _query: &str, _raw: Option<bool>) -> String {
        todo!()
    }

    async fn vizcloud(&self, _query: &str) -> String {
        todo!()
    }

    async fn custom_request(&self, _query: &str, _action: &str) -> String {
        todo!()
    }
}
