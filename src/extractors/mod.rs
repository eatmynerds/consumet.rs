use crate::models::ExtractConfig;

pub mod asianload;
pub mod bilibili;
pub mod filemoon;
pub mod gogocdn;
pub mod kwik;
pub mod mixdrop;
pub mod mp4upload;
pub mod rapidcloud;
pub mod smashystream;
pub mod streamhub;
pub mod streamlare;
pub mod streamsb;
pub mod streamtape;
pub mod streamwish;
pub mod vidcloud;
pub mod vidmoly;
pub mod vizcloud;

pub trait VideoExtractor {
    type VideoSource;

    /// takes video link
    /// returns video sources (video links) available
    async fn extract(
        &mut self,
        video_url: String,
        args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource>;
}

pub use asianload::*;
pub use bilibili::*;
pub use filemoon::*;
pub use gogocdn::*;
pub use kwik::*;
pub use mixdrop::*;
pub use mp4upload::*;
pub use rapidcloud::*;
pub use smashystream::*;
pub use streamhub::*;
pub use streamlare::*;
pub use streamsb::*;
pub use streamtape::*;
pub use streamwish::*;
pub use vidcloud::*;
pub use vidmoly::*;
pub use vizcloud::*;
