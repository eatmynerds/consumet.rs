use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct BrMangas;

#[derive(Debug)]
pub enum BrMangasError {}

const BASE_URL: &str = "https://brmangas.com";

impl<'a> MangaParser<'a> for BrMangas {
    type MangaError = BrMangasError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query
    ) -> Result<ISearch<IMangaResult>, <BrMangas as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <BrMangas as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <BrMangas as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
