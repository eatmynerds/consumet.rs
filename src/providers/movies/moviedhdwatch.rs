use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct MovieHDWatch;

#[derive(Debug)]
pub enum MovieHDWatchError {}

const BASE_URL: &str = "https://movieshd.watch";

impl<'a> MovieParser<'a> for MovieHDWatch {
    type MovieError = MovieHDWatchError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <MovieHDWatch as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <MovieHDWatch as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <MovieHDWatch as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <MovieHDWatch as MovieParser<'a>>::MovieError> {
        todo!()
    }
}

impl MovieHDWatch {
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
