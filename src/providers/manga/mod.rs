use crate::models::{IMangaChapterPage, IMangaInfo, IMangaResult, ISearch};

pub mod asurascans;
pub mod brmangas;
pub mod comick;
pub mod flamescans;
pub mod mangadex;
pub mod mangahere;
pub mod mangahost;
pub mod mangakakalot;
pub mod mangapark;
pub mod mangapill;
pub mod mangareader;
pub mod mangasee123;

#[derive(Default, Clone, Debug)]
pub struct MangaConfig<'a> {
    query: Option<&'a str>,
    page: Option<usize>,
    limit: Option<usize>,
}

pub trait MangaParser<'a> {
    type MangaError;

    async fn search(
        &self,
        args: MangaConfig<'a>,
    ) -> Result<ISearch<IMangaResult>, Self::MangaError>;

    async fn fetch_manga_info(&self, manga_id: &str) -> Result<IMangaInfo, Self::MangaError>;

    async fn fetch_chapter_pages(
        &self,
        chapter_id: &str,
    ) -> Result<Vec<IMangaChapterPage>, Self::MangaError>;
}

pub use asurascans::*;
pub use brmangas::*;
pub use comick::*;
pub use flamescans::*;
pub use mangadex::*;
pub use mangahere::*;
pub use mangahost::*;
pub use mangakakalot::*;
pub use mangapark::*;
pub use mangapill::*;
pub use mangareader::*;
pub use mangasee123::*;
