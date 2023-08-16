use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaSee123;

#[derive(Debug)]
pub enum MangaSee123Error {}

const BASE_URL: &str = "https://mangasee123.com";

impl<'a> MangaParser<'a> for MangaSee123 {
    type MangaError = MangaSee123Error;

    async fn search(
        &self,
        _args: MangaConfig<'a>,
    ) -> Result<ISearch<IMangaResult>, <MangaSee123 as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaSee123 as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaSee123 as MangaParser<'a>>::MangaError> {
        todo!()
    }
}

impl MangaSee123 {
    async fn process_script_tag_variable(&self, _script: &str, _variable: &str) -> String {
        todo!()
    }

    async fn process_chapter_number(&self, _chapter: &str) -> String {
        todo!()
    }

    async fn process_chapter_for_image_url(&self, _chapter: &str) -> String {
        todo!()
    }
}
