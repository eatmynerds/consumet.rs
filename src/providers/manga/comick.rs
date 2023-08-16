use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct Comick;

#[derive(Debug)]
pub enum ComickError {}

const BASE_URL: &str = "https://comick.app";
const API_URL: &str = "https://api.comick.app";

impl<'a> MangaParser<'a> for Comick {
    type MangaError = ComickError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query, page = 1, limit = 20
    ) -> Result<ISearch<IMangaResult>, <Comick as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <Comick as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <Comick as MangaParser<'a>>::MangaError> {
        todo!()
    }
}

impl Comick {
    async fn fetch_all_chapters(&self, _manga_id: &str, _page: usize) -> Vec<String> {
        todo!()
    }

    async fn get_comic_id(&self, _id: &str) -> String {
        todo!()
    }
}
