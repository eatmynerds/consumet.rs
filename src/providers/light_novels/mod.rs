pub mod readlightnovels;

pub trait LightNovelParser {
    type LightNovelError;

    async fn search(&self, query: &str) -> Result<String, Self::LightNovelError>;

    async fn fetch_light_novel_info(
        &self,
        light_novel_url: &str,
        chapter_page: isize,
    ) -> Result<String, Self::LightNovelError>;

    async fn fetch_chapter_content(
        &self,
        chapter_id: &str,
    ) -> Result<String, Self::LightNovelError>;
}

pub use readlightnovels::*;
