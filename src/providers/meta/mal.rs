use crate::{
    models::{
        IAnimeEpisode, IAnimeInfo, IAnimeResult, IEpisodeServer, ISearch, ISource, MediaStatus,
    },
    providers::{AniListError, AnimeConfig, AnimeParser},
};

pub struct MyAnimeList;

#[derive(Debug)]
pub enum MyAnimeListError {}

const KITSU_GRAPHQL_URL: &str = "https://kitsu.io/api/graphl";
const MAL_SYNC_URL: &str = "https://api.malsync.moe";

impl<'a> AnimeParser<'a> for MyAnimeList {
    type AnimeError = AniListError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query, page = 1
    ) -> Result<ISearch<IAnimeResult>, <MyAnimeList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>, // anime_id, dub = false, fetch_filler = false
    ) -> Result<IAnimeInfo, <MyAnimeList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <MyAnimeList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>,
    ) -> Result<ISource, <MyAnimeList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl MyAnimeList {
    async fn mal_status_to_media_status(&self, _status: String) -> MediaStatus {
        todo!()
    }

    async fn populate_episode_list(
        &self,
        _episode: Vec<IAnimeEpisode>,
        _url: String,
        _count: Option<usize>,
    ) {
        todo!()
    }

    async fn find_anime_raw(&self, _slug: String, _external_links: String) {
        todo!()
    }

    async fn find_anime_slug(
        &self,
        _title: String,
        _season: String,
        _start_date: usize,
        _mal_id: String,
        _dub: bool,
        _external_links: String,
    ) -> Vec<IAnimeEpisode> {
        todo!()
    }

    async fn find_kitsu_anime(
        &self,
        _possible_provider_episodes: Vec<IAnimeEpisode>,
        _season: String,
        _start_date: usize,
    ) {
        todo!()
    }

    async fn fetch_mal_info_by_id(&self, _id: String) -> IAnimeInfo {
        todo!()
    }
}
