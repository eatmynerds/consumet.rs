use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct Goku;

#[derive(Debug)]
pub enum GokuError {}

const BASE_URL: &str = "https://goku.sx";

impl<'a> MovieParser<'a> for Goku {
    type MovieError = GokuError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <Goku as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <Goku as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <Goku as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <Goku as MovieParser<'a>>::MovieError> {
        todo!()
    }
}

impl Goku {
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
