use crate::{
    models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource},
    providers::{MovieConfig, MovieParser},
};

pub struct TMDB;

#[derive(Debug)]
pub enum TMDBError {}

const BASE_URL: &str = "https:/www.themoviedb.org";
const API_URL: &str = "https://api.themoviedb.org/3";

impl<'a> MovieParser<'a> for TMDB {
    type MovieError = TMDBError;

    async fn search(
        &self,
        args: MovieConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IMovieResult>, <TMDB as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_media_info(
        &self,
        args: MovieConfig<'a>, // media_id, type
    ) -> Result<IMovieInfo, <TMDB as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <TMDB as MovieParser<'a>>::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, <TMDB as MovieParser<'a>>::MovieError> {
        todo!()
    }
}

impl TMDB {
    // TODO: Change extra_data to a struct
    async fn find_id_from_title(&self, title: String, extra_data: String) -> String {
        todo!()
    }
}
