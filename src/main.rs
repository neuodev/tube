mod video;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::borrow::Cow;
use std::fs::ReadDir;
use std::io::Write;
use std::{thread};
use std::{time::Duration};
use std::path::Path;
use url::{Url};
use video::Video;

#[tokio::main]
async fn main() {
    let mut video_ids = vec![];
    loop {
        let url = match user_input("Video Url: ").await.as_str() {
            "" => break,
            val => val.to_string()
        };
        let querys = Url::parse(url.as_str()).expect(format!("{} is not a valid url", url).as_str());
        if let Some((Cow::Borrowed(key), Cow::Borrowed(val))) = querys.query_pairs().next() {
            if key == "v" {
                video_ids.push(val.to_string())
            }
        };
    }
    println!("Will download {} video(s)", video_ids.len());
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
        thread::sleep(Duration::from_millis(500));
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


async fn user_input(prefix: &str) -> String {
    print!("{}", prefix);
    ::std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    ::std::io::stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}