use crate::models::{INewsFeed, INewsInfo, Topics};

use super::NewsParser;

pub struct AnimeNewsNetwork;

#[derive(Debug)]
pub enum AnimeNewsNetworkError {}

const BASE_URL: &str = "https://www.animenewsnetwork.com";

impl NewsParser for AnimeNewsNetwork {
    type NewsError = AnimeNewsNetworkError;

    async fn fetch_news_feeds(
        &self,
        topic: Option<Topics>,
    ) -> Result<Vec<INewsFeed>, Self::NewsError> {
        todo!()
    }

    async fn fetch_news_info(&self, id: String) -> Result<INewsInfo, Self::NewsError> {
        todo!()
    }
}

impl AnimeNewsNetwork {
    async fn scrape_news_info(&self, url: String) -> INewsInfo {
        todo!()
    }
}
