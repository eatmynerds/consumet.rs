use super::{MangaConfig, MangaParser};
use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub struct MangaKakaLot;

#[derive(Debug)]
pub enum MangaKakaLotError {}

const BASE_URL: &str = "https://mangakakalot.com";

impl<'a> MangaParser<'a> for MangaKakaLot {
    type MangaError = MangaKakaLotError;

    async fn search(
        &self,
        _args: MangaConfig<'a>, // query
    ) -> Result<ISearch<IMangaResult>, <MangaKakaLot as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_manga_info(
        &self,
        _manga_id: &str,
    ) -> Result<IMangaInfo, <MangaKakaLot as MangaParser<'a>>::MangaError> {
        todo!()
    }

    async fn fetch_chapter_pages(
        &self,
        _chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, <MangaKakaLot as MangaParser<'a>>::MangaError> {
        todo!()
    }
}
