use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct KissAsian;

#[derive(Debug)]
pub enum KissAsianError {}

const BASE_URL: &str = "https://kissasian.mx";

impl<'a> MovieParser<'a> for KissAsian {
    type MovieError = KissAsianError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <KissAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <KissAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <KissAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <KissAsian as MovieParser<'a>>::MovieError> {
        todo!()
    }
}
