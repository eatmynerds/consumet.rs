use super::{AnimeConfig, AnimeParser};
use crate::models::{IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource};

pub struct GogoAnime;

#[derive(Debug)]
pub enum GogoAnimeError {}

const BASE_URL: &str = "https://gogoanimehd.to";
const AJAX_URL: &str = "https://ajax.godo-load.com/ajax";

impl<'a> AnimeParser<'a> for GogoAnime {
    type AnimeError = GogoAnimeError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IAnimeResult>, <GogoAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id
    ) -> Result<IAnimeInfo, <GogoAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>, // episode_id
    ) -> Result<Vec<IEpisodeServer>, <GogoAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>, // episode_id, server = StreamingServers::VidStreaming
    ) -> Result<ISource, <GogoAnime as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl GogoAnime {
    async fn fetch_anime_id_from_episode_id(&self, _episode_id: &str) -> String {
        todo!()
    }

    async fn fetch_recent_episode(&self, _page: usize, r#_type: usize) -> ISearch<IAnimeResult> {
        todo!()
    }

    async fn fetch_genre_info(&self, _genre: &str, _page: usize) -> ISearch<IAnimeResult> {
        todo!()
    }

    async fn fetch_top_airing(&self, _page: usize) -> ISearch<IAnimeResult> {
        todo!()
    }
}
