use super::MovieParser;
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct KissAsian;

#[derive(Debug)]
pub enum KissAsianError {}

impl MovieParser for KissAsian {
    type MovieError = KissAsianError;

    async fn search(&self, _query: &str) -> Result<ISearch<IMovieResult>, Self::MovieError> {
        todo!()
    }

    async fn fetch_media_info(&self, _media_id: &str) -> Result<IMovieInfo, Self::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _episode_id: &str,
    ) -> Result<Vec<IEpisodeServer>, Self::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(&self, _episode_id: &str) -> Result<ISource, Self::MovieError> {
        todo!()
    }
}

impl KissAsian {}
