use crate::models::ILightNovelChapter;

use super::LightNovelParser;

pub struct ReadLightNovels;

#[derive(Debug)]
pub enum ReadLightNovelsError {}

const BASE_URL: &str = "https://readlightnovels.net";

impl LightNovelParser for ReadLightNovels {
    type LightNovelError = ReadLightNovelsError;

    async fn search(&self, _query: &str) -> Result<String, Self::LightNovelError> {
        todo!()
    }

    async fn fetch_light_novel_info(
        &self,
        _light_novel_url: &str,
        _chapter_page: isize,
    ) -> Result<String, Self::LightNovelError> {
        todo!()
    }

    async fn fetch_chapter_content(
        &self,
        _chapter_id: &str,
    ) -> Result<String, Self::LightNovelError> {
        todo!()
    }
}

impl ReadLightNovels {
    async fn fetch_chapters(
        &self,
        _novel_id: usize,
        _chapter_page: usize,
        _referer: &str,
    ) -> Vec<ILightNovelChapter> {
        todo!()
    }

    async fn fetch_all_chapters(&self, _novel_id: usize, _pages: usize, _referer: &str) -> String {
        todo!()
    }
}
