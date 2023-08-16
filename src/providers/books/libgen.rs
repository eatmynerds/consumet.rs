use crate::models::LibgenResult;

use super::BookParser;

pub struct LibGen;

#[derive(Debug)]
pub enum LibGenError {}

const BASE_URL: &str = "http://libgen";
const DOWNLOAD_IP: &str = "http://62.182.86.140";

impl BookParser for LibGen {
    type BookError = LibGenError;

    async fn search(&self, _query: &str, _page: usize) -> Result<LibgenResult, Self::BookError> {
        todo!()
    }
}

impl LibGen {
    async fn scrape_book(&self, _book_url: &str) -> LibgenResult {
        todo!()
    }
}
