use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{io::copy, process};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoFormat {
    url: String,
    width: i32,
    height: i32,
    quality: String,
    fps: i32,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct Video {
    pub formats: Vec<VideoFormat>,
    pub details: VideoDetails,
}

impl Video {
    pub async fn download(&self) {
        println!("download {}...", self.details.title);
        let url = &self.formats.get(0).unwrap().url;
        let filename = format!("{}.mp4", self.details.title);
        let resp = reqwest::get(url.as_str()).await.expect("request failed");
        let mut out = File::create(filename).expect("FAilled to create the output file");
        let inp = resp.bytes().await.expect("Filed to read file bytes");
        copy(&mut inp.as_ref(), &mut out).expect("failed to copy content");
    }
}
