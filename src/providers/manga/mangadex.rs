use super::MangaParser;
use crate::models::{IMangaChapterPage, IMangaInfo};

pub struct MangaDex;

#[derive(Debug)]
pub enum MangaDexError {}

impl MangaParser for MangaDex {
    type MangaError = MangaDexError;

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

impl MangaDex {}
