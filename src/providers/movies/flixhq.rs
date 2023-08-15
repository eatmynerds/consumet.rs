use super::MovieParser;
use crate::models::{
    IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult, ISearch, ISource, TvType,
};
use visdom::{types::Elements, Vis};

pub struct FlixHQ;

#[derive(Debug)]
pub enum FlixHQError {}

const BASE_URL: &'static str = "https://flixhq.to";

impl MovieParser for FlixHQ {
    type MovieError = FlixHQError;

    async fn search(&self, query: &str) -> Result<ISearch<IMovieResult>, Self::MovieError> {
        let page: u8 = 0;

        let parsed_query = query.replace(' ', "-");
        let page_html = reqwest::Client::new()
            .get(format!(
                "{}/search/{}?page={}",
                BASE_URL, parsed_query, page
            ))
            .send()
            .await?
            .text()
            .await?;

        todo!()
    }

    async fn fetch_media_info(&self, _media_id: &str) -> Result<IMovieInfo, Self::MovieError> {
        todo!()
    }

    async fn fetch_episode_sources(&self, _episode_id: &str) -> Result<ISource, Self::MovieError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _episode_id: &str,
    ) -> Result<Vec<IEpisodeServer>, Self::MovieError> {
        todo!()
    }
}

impl FlixHQ {}
