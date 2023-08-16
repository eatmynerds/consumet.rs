use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct AsuraScans;

#[derive(Debug)]
pub enum AsuraScansError {}

const BASE_URL: &str = "https://asurascans.com";

impl<'a> MangaParser<'a> for AsuraScans {
    type MangaError = AsuraScansError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query
    ) -> Result<ISearch<IMangaResult>, <AsuraScans as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <AsuraScans as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <AsuraScans as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
