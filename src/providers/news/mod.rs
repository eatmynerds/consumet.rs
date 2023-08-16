pub mod animenewsnetwork;

pub trait NewsParser {
    type NewsError;

    async fn fetch_news_feeds(
        &self,
        topic: Option<Topics>,
    ) -> Result<Vec<INewsFeed>, Self::NewsError>;

    async fn fetch_news_info(&self, id: String) -> Result<INewsInfo, Self::NewsError>;
}

pub use animenewsnetwork::*;

use crate::models::{INewsFeed, INewsInfo, Topics};
