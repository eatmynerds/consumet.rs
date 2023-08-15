use super::LightNovelParser;

pub struct ReadLightNovels;

#[derive(Debug)]
pub enum ReadLightNovelsError {}

impl LightNovelParser for ReadLightNovels {
    type LightNovelError = ReadLightNovelsError;

    async fn search(&self, _query: &str) -> Result<String, Self::LightNovelError> {
        todo!()
    }

    async fn fetch_light_novel_info(
        &self,
        _light_novel_url: &str,
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

impl ReadLightNovels {}
