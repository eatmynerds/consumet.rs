use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaPark;

#[derive(Debug)]
pub enum MangaParkError {}

const BASE_URL: &str = "https://v2.mangapark.net";

impl<'a> MangaParser<'a> for MangaPark {
    type MangaError = MangaParkError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query
    ) -> Result<ISearch<IMangaResult>, <MangaPark as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaPark as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaPark as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
