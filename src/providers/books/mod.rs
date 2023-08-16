pub mod libgen;

pub trait BookParser {
    type BookError;

    async fn search(&self, query: &str, page: usize) -> Result<LibgenResult, Self::BookError>;
}

pub use libgen::*;

use crate::models::LibgenResult;
