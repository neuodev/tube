//! Download videos from youtube
//! ```sh
//!     > tube.exe v1 v2
//!     > tube.exe id1 id2
//! ```
//!
//! ## Todo
//! 1. Make a prototype for downlaoding a single video given it id
//! 2. Make the user choose between diffrent qualities
//! 3. Download multiple videos at the same time

mod video;
use serde_json::{json, Value};
use std::env;
use video::Video;

// v1 example i8NETqtGHms
// v2 example NLtt4S9ErIA
const VIDEO_INFO_ENDPOINT: &str = "https://youtubei.googleapis.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

#[tokio::main]
async fn main() {
    println!("{:?}", env::args().collect::<Vec<String>>());
    let video_ids = env::args().skip(1).collect::<Vec<String>>();
    let video_id = video_ids.get(0).unwrap();
    let video = get_video_info(video_id).await;
    video.download().await;
}

async fn get_video_info(video_id: &String) -> Video {
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
