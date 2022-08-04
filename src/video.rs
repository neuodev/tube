use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use std::time::Duration;
use std::{io::copy, process};
use terminal_menu::{button, label, list, menu, mut_menu, run, TerminalMenuItem};

const VIDEO_INFO_ENDPOINT: &str = "https://youtubei.googleapis.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFormat {
    url: String,
    width: i32,
    height: i32,
    #[serde(rename = "qualityLabel")]
    quality: String,
    fps: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDetails {
    #[serde(rename = "videoId")]
    video_id: String,
    title: String,
    #[serde(rename = "lengthSeconds")]
    length_in_sec: String,
    author: String,
    #[serde(rename = "viewCount")]
    viwe_count: String,
}

#[derive(Debug, Clone)]
pub struct Video {
    pub formats: Vec<VideoFormat>,
    pub details: VideoDetails,
}

impl Video {
    pub async fn download(&self, format: &VideoFormat) {
        println!("Download {}...", self.details.title);
        let url = &format.url;
        let filename = format!("{}.mp4", self.details.title);
        let resp = reqwest::get(url.as_str()).await.expect("request failed");
        let mut out = File::create(filename).expect("FAilled to create the output file");
        let inp = resp.bytes().await.expect("Filed to read file bytes");
        copy(&mut inp.as_ref(), &mut out).expect("failed to copy content");
    }
    pub fn select_video_format(&mut self) -> VideoFormat {
        let qualities = self
            .formats
            .clone()
            .into_iter()
            .map(|f| f.quality)
            .collect::<Vec<String>>();

        let mut items: Vec<TerminalMenuItem> = vec![
            label(format!("Title: {}", self.details.title)),
            label(format!("Author: {}", self.details.author)),
            label(format!(
                "Length: {:?}",
                Duration::from_secs(self.details.length_in_sec.parse::<u64>().unwrap())
            )),
            label("Choose video quality:"),
        ];
        for quality in qualities {
            items.push(button(quality.as_str()));
        }
        let menu = menu(items);
        run(&menu);

        self.formats
            .clone()
            .into_iter()
            .find(|f| f.quality.as_str() == mut_menu(&menu).selected_item_name())
            .unwrap()
    }

    pub async fn get_video_info(video_id: &String) -> Video {
        let client = reqwest::Client::new();
        let body = json!({
            "context": {
                "client": {
                    "hl": "en",
                    "clientName": "WEB",
                    "clientVersion": "2.20210721.00.00",
                    "mainAppWebInfo": {
                        "graftUrl": format!("/watch?v={}", video_id)
                    }
                }
            },
            "videoId": format!("{}", video_id)
        });
        let res = client
            .post(VIDEO_INFO_ENDPOINT)
            .body(body.to_string())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let data: Value = serde_json::from_str(res.as_str()).unwrap();
        let streaming_data = data
            .get("streamingData")
            .expect("`streamingData` field not found!");
        let formats = streaming_data
            .get("formats")
            .expect("`formats` field not found")
            .clone();
        let formats = serde_json::from_value::<Vec<_>>(formats).unwrap();
        let details = data
            .get("videoDetails")
            .expect("`videoDetails` field not found")
            .clone();
        let details = serde_json::from_value(details).expect("Unable to parse video details");

        Video { details, formats }
    }
}
