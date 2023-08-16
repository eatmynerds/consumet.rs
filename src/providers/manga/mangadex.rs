use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaDex;

#[derive(Debug)]
pub enum MangaDexError {}

const BASE_URL: &str = "https://mangadex.org";
const API_URL: &str = "https://api.mangadex.org";

impl<'a> MangaParser<'a> for MangaDex {
    type MangaError = MangaDexError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query, page = 1, limit = 20
    ) -> Result<ISearch<IMangaResult>, <MangaDex as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaDex as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaDex as MangaParser<'a>>::MangaError> {
        todo!()
    }
}

impl MangaDex {
    async fn fetch_all_chapters(&self, _manga_id: &str, _offset: usize) -> Vec<String> {
        todo!()
    }

    async fn fetch_cover_image(&self, _cover_id: &str) -> String {
        todo!()
    }
}
