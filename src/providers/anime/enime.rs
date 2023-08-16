use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};
pub struct Enime;

#[derive(Debug)]
pub enum EnimeError {}

const BASE_URL: &str = "https://enime.moe";
const API_URL: &str = "https://api.enime.moe";

impl<'a> AnimeParser<'a> for Enime {
    type AnimeError = EnimeError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query, page = 1, per_page = 15
    ) -> Result<ISearch<IAnimeResult>, <Enime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id
    ) -> Result<IAnimeInfo, <Enime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <Enime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <Enime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl Enime {
    async fn fetch_anime_info_id_raw(&self, _id: &str) -> String {
        todo!()
    }

    async fn fetch_anime_info_by_anilist_id(&self, _id: &str, r#_type: &str) -> IAnimeInfo {
        todo!()
    }

    async fn fetch_anime_info_by_mal_id(&self, _id: &str, r#_type: &str) -> IAnimeInfo {
        todo!()
    }

    async fn fetach_sources_from_episode_id(&self, _episode_id: &str) -> ISource {
        todo!()
    }

    async fn fetch_sources_from_source_id(&self, _source_id: &str) -> ISource {
        todo!()
    }
}
