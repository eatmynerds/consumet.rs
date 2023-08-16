use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct SmashyStream;

#[derive(Debug)]
pub enum SmashyStreamError {}

const BASE_URL: &str = "https://embed.smashystream.com";

impl<'a> MovieParser<'a> for SmashyStream {
    type MovieError = SmashyStreamError;

    async fn search(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISearch<IMovieResult>, <SmashyStream as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <SmashyStream as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <SmashyStream as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <SmashyStream as MovieParser<'a>>::MovieError> {
        todo!()
    }
}
