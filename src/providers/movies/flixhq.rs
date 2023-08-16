use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct FlixHQ;

#[derive(Debug)]
pub enum FlixHQError {}

const BASE_URL: &str = "https://flixhq.to";

impl<'a> MovieParser<'a> for FlixHQ {
    type MovieError = FlixHQError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <FlixHQ as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <FlixHQ as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <FlixHQ as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <FlixHQ as MovieParser<'a>>::MovieError> {
        todo!()
    }
}

impl FlixHQ {
    async fn fetch_recent_movies(&self) -> Vec<IMovieResult> {
        todo!()
    }

    async fn fetch_recent_shows(&self) -> Vec<IMovieResult> {
        todo!()
    }

    async fn fetch_trending_movies(&self) -> Vec<IMovieResult> {
        todo!()
    }

    async fn fetch_trending_shows(&self) -> Vec<IMovieResult> {
        todo!()
    }
}
