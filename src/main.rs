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
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use std::{env, time::Duration};
use video::Video;

// v1 example i8NETqtGHms
// v2 example NLtt4S9ErIA

#[tokio::main]
async fn main() {
    let video_ids = env::args().skip(1).collect::<Vec<String>>();
    println!("Fetching vidoes info...");

    let mut videos = vec![];
    for id in &video_ids {
        let video = Video::get_video_info(id).await;
        videos.push(video);
    }

    let mut formats = vec![];
    for video in &videos {
        let format = video.select_video_format();
        formats.push(format);
        thread::sleep(Duration::from_secs(1));
    }

    let mut handlers = vec![];
    let m = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
    .progress_chars("#>-");

    for (idx, (video, format)) in videos.into_iter().zip(formats.into_iter()).enumerate() {
        let (len, res) = video.fetch(&format).await;
        let pb = m.insert(idx, ProgressBar::new(len));
        pb.set_style(sty.clone());
        pb.set_message(format!("Downloading {}", video.details.title));

        let handler = tokio::spawn(async move {
            video.download(res, &pb, len).await;
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.await.unwrap()
    }

    m.clear().unwrap()
}
