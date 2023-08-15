use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource};

pub mod dramacool;
pub mod flixhq;
pub mod fmovies;
pub mod goku;
pub mod kissasian;
pub mod moviedhdwatch;
pub mod smashystream;
pub mod ummagurau;
pub mod viewasian;

pub trait MovieParser {
    type MovieError;

    async fn search(&self, query: &str) -> Result<ISearch<IMovieResult>, Self::MovieError>;

    async fn fetch_media_info(&self, media_id: &str) -> Result<IMovieInfo, Self::MovieError>;

    async fn fetch_episode_servers(
        &self,
        episode_id: &str,
    ) -> Result<Vec<IEpisodeServer>, Self::MovieError>;

    async fn fetch_episode_sources(&self, episode_id: &str) -> Result<ISource, Self::MovieError>;
}

pub use dramacool::*;
pub use flixhq::*;
pub use fmovies::*;
pub use goku::*;
pub use kissasian::*;
pub use moviedhdwatch::*;
pub use smashystream::*;
pub use ummagurau::*;
pub use viewasian::*;
