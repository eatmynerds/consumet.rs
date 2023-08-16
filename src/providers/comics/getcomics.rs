use super::ComicParser;

pub struct GetComics;

#[derive(Debug)]
pub enum GetComicsError {}

const BASE_URL: &str = "https://getcomics.info";

impl ComicParser for GetComics {
    type ComicError = GetComicsError;

    async fn search(&self, _query: &str, _page: usize) -> Result<String, Self::ComicError> {
        todo!()
    }
}
