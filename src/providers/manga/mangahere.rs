use super::MangaParser;
use crate::models::{IMangaChapterPage, IMangaInfo};

pub struct MangaHere;

#[derive(Debug)]
pub enum MangaHereError {}

impl MangaParser for MangaHere {
    type MangaError = MangaHereError;

    async fn search(&self, _query: &str) -> Result<String, Self::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(&self, _manga_id: &str) -> Result<IMangaInfo, Self::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, Self::MangaError> {
        todo!()
    }
}

impl MangaHere {}
