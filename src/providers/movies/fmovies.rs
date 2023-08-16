use super::{MovieConfig, MovieParser};
use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub struct Fmovies;

#[derive(Debug)]
pub enum FmoviesError {}

const BASE_URL: &str = "https://fmovies.to";

impl<'a> MovieParser<'a> for Fmovies {
    type MovieError = FmoviesError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <Fmovies as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<IMovieInfo, <Fmovies as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <Fmovies as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <Fmovies as MovieParser<'a>>::MovieError> {
        todo!()
    }
}

impl Fmovies {
    async fn ev(&self, query: &str) -> String {
        todo!()
    }

    async fn decrypt(&self, query: &str) -> String {
        todo!()
    }

    async fn ajax_req_url(&self, id: &str) -> String {
        todo!()
    }
}
