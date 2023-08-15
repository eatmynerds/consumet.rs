use super::AnimeParser;
use crate::models::{IAnimeInfo, IEpisodeServer, ISource};

pub struct GogoAnime;

#[derive(Debug)]
pub enum GogoAnimeError {}

impl AnimeParser for GogoAnime {
    type AnimeError = GogoAnimeError;

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

impl GogoAnime {}
