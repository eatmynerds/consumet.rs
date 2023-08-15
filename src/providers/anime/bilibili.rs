use super::AnimeParser;
use crate::models::{IAnimeInfo, IEpisodeServer, ISource};

pub struct BiliBili;

#[derive(Debug)]
pub enum BiliBiliError {}

impl AnimeParser for BiliBili {
    type AnimeError = BiliBiliError;

    async fn fetch_anime_info(&self, _anime_id: &str) -> Result<IAnimeInfo, Self::AnimeError> {
        todo!()
    }
    async fn fetch_episode_servers(
        &self,
        _episode_id: &str,
    ) -> Result<Vec<IEpisodeServer>, Self::AnimeError> {
        todo!()
    }
    async fn fetch_episode_sources(&self, _episode_id: &str) -> Result<ISource, Self::AnimeError> {
        todo!()
    }
}

impl BiliBili {}