use crate::models::{IEpisodeServer, IMovieInfo, IMovieResult, ISearch, ISource, StreamingServers};

pub mod dramacool;
pub mod flixhq;
pub mod fmovies;
pub mod goku;
pub mod kissasian;
pub mod moviedhdwatch;
pub mod smashystream;
pub mod ummagurau;
pub mod viewasian;

#[derive(Default, Clone, Debug)]
pub struct MovieConfig<'a> {
    pub query: Option<&'a str>,
    pub page: Option<i8>,
    pub media_id: Option<&'a str>,
    pub episode_id: Option<&'a str>,
    pub server: Option<StreamingServers>,
    pub r#type: Option<&'a str>,
    pub season: Option<usize>,
    pub episode: Option<usize>,
    pub tmdb_id: Option<&'a str>,
}

pub trait MovieParser<'a> {
    type MovieError;

    async fn search(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISearch<IMovieResult>, Self::MovieError>;

    async fn fetch_media_info(&self, args: MovieConfig<'a>)
        -> Result<IMovieInfo, Self::MovieError>;

    async fn fetch_episode_servers(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<Vec<IEpisodeServer>, Self::MovieError>;

    async fn fetch_episode_sources(
        &self,
        args: MovieConfig<'a>,
    ) -> Result<ISource, Self::MovieError>;
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
