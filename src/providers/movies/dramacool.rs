use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct DramaCool;

#[derive(Debug)]
pub enum DramaCoolError {}

const BASE_URL: &str = "https://dramacool.hr";

impl<'a> MovieParser<'a> for DramaCool {
    type MovieError = DramaCoolError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <DramaCool as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <DramaCool as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <DramaCool as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <DramaCool as MovieParser<'a>>::MovieError> {
        todo!()
    }
}
