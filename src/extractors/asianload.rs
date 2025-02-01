use crate::models::{ExtractConfig, ISubtitle, IVideo, VideoExtractor};
use serde::{Deserialize, Serialize};

/// Contains both the Decrypted Sources and Subtitles
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AsianLoad {
    pub sources: Vec<IVideo>,
    pub subtitles: Vec<ISubtitle>,
}

impl VideoExtractor for AsianLoad {
    type VideoSource = AsianLoad;

    // NOTE: Only needs video_url param
    async fn extract(
        &mut self,
        _video_url: &str,
        args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource> {
        let ExtractConfig {
            vis_cloud_helper: _,
            api_key: _,
            is_alternative: _,
            user_agent: _,
        } = args;

        self.sources.push(IVideo {
            url: None,
            quality: None,
            is_m3u8: None,
            is_dash: None,
            size: None,
            other: None,
        });

        self.subtitles.push(ISubtitle {
            id: None,
            url: None,
            lang: None,
        });

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
