use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct BiliBili;

#[derive(Debug)]
pub enum BiliBiliError {}

const BASE_URL: &str = "https://bilibili.tv";
const API_URL: &str = "https://api.bilibili.tv/intl/gateway/web";

impl<'a> AnimeParser<'a> for BiliBili {
    type AnimeError = BiliBiliError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query,
    ) -> Result<ISearch<IAnimeResult>, <BiliBili as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id
    ) -> Result<IAnimeInfo, <BiliBili as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <BiliBili as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<ISource, <BiliBili as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}
