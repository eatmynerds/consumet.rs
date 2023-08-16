use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct ViewAsian;

#[derive(Debug)]
pub enum ViewAsianError {}

const BASE_URL: &str = "https://viewasian.co";

impl<'a> MovieParser<'a> for ViewAsian {
    type MovieError = ViewAsianError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <ViewAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <ViewAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <ViewAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <ViewAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }
}
