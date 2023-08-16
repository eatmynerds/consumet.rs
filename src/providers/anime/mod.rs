use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource, StreamingServers};

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

#[derive(Default, Clone, Debug)]
pub struct AnimeConfig<'a> {
    pub query: Option<&'a str>,
    pub page: Option<i8>,
    pub anime_id: Option<&'a str>,
    pub episode_id: Option<&'a str>,
    pub server: Option<StreamingServers>,
    pub dub: Option<bool>,
    pub fetch_filler: Option<bool>,
}

pub trait AnimeParser<'a> {
    type AnimeError;

    async fn search(
        &self,
        args: AnimeConfig<'a>,
    ) -> Result<ISearch<IAnimeResult>, Self::AnimeError>;

    async fn fetch_anime_info(&self, args: AnimeConfig<'a>)
        -> Result<IAnimeInfo, Self::AnimeError>;

    async fn fetch_episode_servers(
        &self,
        args: AnimeConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, Self::AnimeError>;

    async fn fetch_episode_sources(
        &self,
        args: AnimeConfig<'a>,
    ) -> Result<ISource, Self::AnimeError>;
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
