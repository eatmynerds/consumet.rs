use crate::models::{ExtractConfig, IVideo, VideoExtractor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mp4Upload {
    sources: Vec<IVideo>,
}

impl VideoExtractor for Mp4Upload {
    type VideoSource = Mp4Upload;

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

        Ok(Self {
            sources: self.sources.clone(),
        })
    }
}
