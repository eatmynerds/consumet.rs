use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaPill;

#[derive(Debug)]
pub enum MangaPillError {}

const BASE_URL: &str = "https://mangapill.com";

impl<'a> MangaParser<'a> for MangaPill {
    type MangaError = MangaPillError;

    async fn search(
        &self,
        _args: MangaConfig<'a>,
    ) -> Result<ISearch<IMangaResult>, <MangaPill as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaPill as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaPill as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
