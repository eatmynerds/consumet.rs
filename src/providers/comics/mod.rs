pub mod getcomics;

pub trait ComicParser {
    type ComicError;

    async fn search(&self, query: &str, page: usize) -> Result<String, Self::ComicError>;
}

pub use getcomics::*;
