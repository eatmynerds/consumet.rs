use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct UmmaGurau;

#[derive(Debug)]
pub enum UmmaGurauError {}

const BASE_URL: &str = "https://www1.ummagurau.com";

impl<'a> MovieParser<'a> for UmmaGurau {
    type MovieError = UmmaGurauError;

    async fn search(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISearch<IMovieResult>, <UmmaGurau as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <UmmaGurau as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <UmmaGurau as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <UmmaGurau as MovieParser<'a>>::MovieError> {
        todo!()
    }
}
