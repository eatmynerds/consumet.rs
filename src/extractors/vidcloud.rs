use crate::{
    models::{ExtractConfig, VideoExtractor},
    CLIENT,
};
use serde::{Deserialize, Serialize};

/// Contains both the Decrypted Sources and Subtitles
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VidCloud {
    pub sources: Vec<VidCloudSource>,
    pub subtitles: Vec<VidCloudSubtitle>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VidCloudSubtitle {
    pub url: String,
    pub lang: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VidCloudSource {
    pub url: String,
    pub quality: String,
    pub is_m3u8: bool,
}

/// Contains the Subtitles for the Sources
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub file: String,
    pub label: String,
    pub kind: String,
    pub default: Option<bool>,
}

/// Contains the Decrypted Sources File
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Video {
    pub file: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sources {
    pub sources: Vec<Video>,
    pub tracks: Option<Vec<Tracks>>,
    pub server: u8,
}

impl VideoExtractor for VidCloud {
    type VideoSource = VidCloud;

    // NOTE: Only needs video_url & is_alternativeparam
    async fn extract(
        &mut self,
        video_url: String,
        args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource> {
        let ExtractConfig {
            vis_cloud_helper: _,
            api_key: _,
            is_alternative: _,
            user_agent: _,
        } = args;

        let sources_str: String = CLIENT
            .get(format!("https://misc-embed-decrypt.v4sq52.easypanel.host/embed?embed_url={}&referrer=https://flixhq.to", video_url))
            .send()
            .await?
            .text()
            .await?;

        let decrypted_sources: Sources = serde_json::from_str(&sources_str) .expect("Failed to deserialize json");

        let mut temp_sources: Vec<VidCloudSource> = vec![];

        self.sources.push(VidCloudSource {
            url: decrypted_sources.sources[0].file.clone().unwrap(),
            quality: "auto".to_string(),
            is_m3u8: decrypted_sources.sources[0].file.clone().unwrap().contains(".m3u8"),
        });

        for source in decrypted_sources.sources {
            let data = CLIENT
                .get(&source.file.unwrap())
                .send()
                .await?
                .text()
                .await?;

            let urls: Vec<String> = data
                .lines()
                .filter(|line| line.contains(".m3u8"))
                .map(|line| line.to_string())
                .collect();

            let qualities: Vec<String> = data
                .lines()
                .filter(|line| line.contains("RESOLUTION="))
                .map(|line| line.to_string())
                .collect();

            let td_array: Vec<(&str, &str)> = qualities
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let f1 = s.split('x').nth(1).unwrap_or("");
                    let f2 = urls[i].as_str();
                    (f1, f2)
                })
                .collect();

            temp_sources.extend(td_array.iter().map(|&(f1, f2)| VidCloudSource {
                url: f2.to_string(),
                quality: f1.to_string(),
                is_m3u8: f2.contains(".m3u8"),
            }));

            self.sources.extend(temp_sources.iter().cloned());
        }

        let subtitles: Vec<VidCloudSubtitle> = decrypted_sources
            .tracks
            .unwrap()
            .iter()
            .map(|s| VidCloudSubtitle {
                url: s.file.clone(),
                lang: s.label.clone(),
            })
            .collect();

        self.subtitles.extend(subtitles);

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
