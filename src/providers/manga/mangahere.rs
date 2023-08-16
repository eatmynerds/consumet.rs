use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaHere;

#[derive(Debug)]
pub enum MangaHereError {}

const BASE_URL: &str = "https://mangahere.cc";

impl<'a> MangaParser<'a> for MangaHere {
    type MangaError = MangaHereError;

    // query, page = 1
    async fn search(
        &self,
        _args: MangaConfig<'a>,
    ) -> Result<ISearch<IMangaResult>, <MangaHere as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaHere as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaHere as MangaParser<'a>>::MangaError> {
        todo!()
    }
}

impl MangaHere {
    async fn extract_key(&self, _html: &str) -> String {
        todo!()
    }
}
