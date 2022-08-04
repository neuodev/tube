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
use std::env;
use video::Video;

// v1 example i8NETqtGHms
// v2 example NLtt4S9ErIA

#[tokio::main]
async fn main() {
    let video_ids = env::args().skip(1).collect::<Vec<String>>();
    let video_id = video_ids.get(0).unwrap();
    let mut video = Video::get_video_info(video_id).await;
    let format = video.select_video_format();
    video.download(&format).await;
}
