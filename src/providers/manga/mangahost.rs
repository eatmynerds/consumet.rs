use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaHost;

#[derive(Debug)]
pub enum MangaHostError {}

const BASE_URL: &str = "https://mangahosted.com";

impl<'a> MangaParser<'a> for MangaHost {
    type MangaError = MangaHostError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query
    ) -> Result<ISearch<IMangaResult>, <MangaHost as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaHost as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaHost as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
