use std::collections::HashMap;

use crate::{
    models::{
        Genres, IAnimeEpisode, IAnimeInfo, IAnimeResult, IEpisodeServer, IMangaChapter, ISearch,
        ISource,
    },
    providers::{AnimeConfig, AnimeParser},
};

pub struct AniList;

#[derive(Debug)]
pub enum AniListError {}

const ANILIST_GRAPHQL_URL: &str = "https://graphql.anilist.co";
const KITSU_GRAPHQL_URL: &str = "https://kitsu.io/api/graphl";
const MAL_SYNC_URL: &str = "https://api.malsync.moe";
const ENIME_URL: &str = "https://api.enime.moe";

impl<'a> AnimeParser<'a> for AniList {
    type AnimeError = AniListError;

    async fn search(
        &self,
        _args: AnimeConfig<'a>, // query, page = 1, per_page = 1
    ) -> Result<ISearch<IAnimeResult>, <AniList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_anime_info(
        &self,
        _args: AnimeConfig<'a>,
    ) -> Result<IAnimeInfo, <AniList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_servers(
        &self,
        _args: AnimeConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, <AniList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }

    async fn fetch_episode_sources(
        &self,
        _args: AnimeConfig<'a>,
    ) -> Result<ISource, <AniList as AnimeParser<'a>>::AnimeError> {
        todo!()
    }
}

impl AniList {
    async fn advanced_search(
        &self,
        _query: Option<String>,
        _type: String,
        _page: usize,
        _per_page: usize,
        _format: Option<String>,
        _sort: Option<Vec<String>>,
        _genres: Option<Vec<Genres>>,
        _id: Option<String>,
        _year: Option<usize>,
        _status: Option<String>,
        _season: Option<String>,
    ) -> ISearch<IAnimeResult> {
        todo!()
    }

    async fn fetch_anime(
        &self,
        _title: HashMap<String, String>,
        _season: String,
        _start_date: usize,
        _mal_id: usize,
        _dub: bool,
        _anilist_id: String,
        _external_links: Option<String>,
    ) -> Vec<IAnimeEpisode> {
        todo!()
    }

    async fn find_anime_slug(
        &self,
        _title: String,
        _season: String,
        _start_date: usize,
        _mal_id: usize,
        _dub: bool,
        _anilist_id: String,
        _external_links: Option<String>,
    ) -> Vec<IAnimeEpisode> {
        todo!()
    }

    async fn find_kitsu_anime(
        &self,
        _possible_provider_episodes: Vec<IAnimeEpisode>,
        _season: Option<String>,
        _start_date: usize,
    ) {
        todo!()
    }

    async fn fetch_trending_anime(
        &self,
        _page: Option<usize>,
        _per_page: Option<usize>,
    ) -> ISearch<IAnimeResult> {
        todo!()
    }

    async fn fetch_popular_anime(
        &self,
        _page: Option<usize>,
        _per_page: Option<usize>,
    ) -> ISearch<IAnimeResult> {
        todo!()
    }

    async fn fetch_airing_schedule(
        &self,
        _page: Option<usize>,
        _per_page: Option<usize>,
        _week_start: Option<usize>,
        _week_end: Option<usize>,
        _not_yet_aired: Option<bool>,
    ) -> ISearch<IAnimeResult> {
        todo!()
    }

    async fn fetch_anime_genres(
        &self,
        _genres: Vec<Genres>,
        _page: Option<usize>,
        _per_page: Option<usize>,
    ) {
        todo!()
    }

    async fn find_anime_raw(&self, _slug: String, _external_links: String) {
        todo!()
    }

    async fn fetch_random_anime(&self) -> IAnimeInfo {
        todo!()
    }

    async fn fetch_recent_episodes(
        &self,
        _provider: String,
        _page: Option<usize>,
        _per_page: Option<usize>,
    ) -> ISearch<IAnimeResult> {
        todo!()
    }

    // TODO: Change media to a struct instead
    async fn fetch_default_episode_list(
        &self,
        _media: HashMap<String, String>,
        _dub: bool,
        _id: String,
    ) {
        todo!()
    }

    async fn fetch_episodes_list_by_id(
        &self,
        _id: String,
        _dub: bool,
        _fetch_filler: Option<bool>,
    ) {
        todo!()
    }

    async fn fetch_anilist_info_by_id(&self, _id: String) {
        todo!()
    }

    async fn fetch_staff_by_id(&self, _id: usize) {
        todo!()
    }

    // TODO: static manga or some shit like that
    // static Manga = class Manga { provider: MangaParser; }

    async fn fetch_character_info_by_id(&self, _id: String) {
        todo!()
    }

    // TODO: Rename all occurences of `provider: String` with the MangaParser trait

    async fn find_manga_slug(
        &self,
        _provider: String,
        _title: String,
        _mal_id: usize,
    ) -> Vec<IMangaChapter> {
        todo!()
    }

    async fn find_manga_raw(&self, _provider: String, _slug: String, _title: String) {
        todo!()
    }

    async fn find_manga(
        &self,
        _provider: String,
        _title: HashMap<String, String>,
        _mal_id: usize,
    ) -> Vec<IMangaChapter> {
        todo!()
    }
}
