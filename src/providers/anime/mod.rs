use crate::models::{IAnimeInfo, IEpisodeServer, ISource};

pub mod animefox;
pub mod animepahe;
pub mod animesaturn;
pub mod bilibili;
pub mod crunchyroll;
pub mod enime;
pub mod gogoanime;
pub mod kickassanime;
pub mod marin;
pub mod nineanime;
pub mod zoro;

pub trait AnimeParser {
    type AnimeError;

    async fn fetch_anime_info(&self, anime_id: &str) -> Result<IAnimeInfo, Self::AnimeError>;
    async fn fetch_episode_servers(
        &self,
        episode_id: &str,
    ) -> Result<Vec<IEpisodeServer>, Self::AnimeError>;
    async fn fetch_episode_sources(&self, episode_id: &str) -> Result<ISource, Self::AnimeError>;
}

pub use animefox::*;
pub use animepahe::*;
pub use animesaturn::*;
pub use bilibili::*;
pub use crunchyroll::*;
pub use enime::*;
pub use gogoanime::*;
pub use kickassanime::*;
pub use marin::*;
pub use nineanime::*;
pub use zoro::*;
