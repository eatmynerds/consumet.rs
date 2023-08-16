use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct FlameScans;

#[derive(Debug)]
pub enum FlameScansError {}

const BASE_URL: &str = "https://flamescans.org";

impl<'a> MangaParser<'a> for FlameScans {
    type MangaError = FlameScansError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query
    ) -> Result<ISearch<IMangaResult>, <FlameScans as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <FlameScans as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <FlameScans as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
