use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaReader;

#[derive(Debug)]
pub enum MangaReaderError {}

const BASE_URL: &str = "https://mangareader.to";

impl<'a> MangaParser<'a> for MangaReader {
    type MangaError = MangaReaderError;

    async fn search(
        &self,
        _args: MangaConfig<'a>,
    ) -> Result<ISearch<IMangaResult>, <MangaReader as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaReader as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaReader as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
