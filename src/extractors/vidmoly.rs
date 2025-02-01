use crate::models::{ExtractConfig, IVideo, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VidMoly {
    pub sources: Vec<IVideo>,
}

impl VideoExtractor for VidMoly {
    type VideoSource = VidMoly;

    // NOTE: Only needs video_url param
    async fn extract(
        &mut self,
        _video_url: &str,
        _args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource> {
        self.sources.push(IVideo {
            url: None,
            quality: None,
            is_m3u8: None,
            is_dash: None,
            size: None,
            other: None,
        });

        Ok(Self {
            sources: self.sources.clone(),
        })
    }
}
