use std::io::copy;
use std::fs::File;

#[tokio::main]
async fn main() {
    // let resp = reqwest::get("https://rr2---sn-uxaxjvhxbt2u-j5pld.googlevideo.com/videoplayback?expire=1659621134&ei=rnrrYrv4LZLmWLaltoAG&ip=41.34.26.128&id=o-ABRUkbjEHH28APiyw-IJKECijL4qMHW2UgLr9ZwZnqzY&itag=22&source=youtube&requiressl=yes&mh=ZP&mm=31%2C29&mn=sn-uxaxjvhxbt2u-j5pld%2Csn-hgn7yn76&ms=au%2Crdu&mv=m&mvi=2&pl=20&initcwndbps=277500&spc=lT-KhulYiq7kppt8HrDs0mTTRSDccdU&vprv=1&mime=video%2Fmp4&ns=lEu_FXiUi37N531ZYk7VmL8H&cnr=14&ratebypass=yes&dur=74.118&lmt=1654352978337974&mt=1659599090&fvip=4&fexp=24001373%2C24007246&c=WEB&rbqsm=fr&txp=5432434&n=YndwRR1ZbmP1fvjj&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cspc%2Cvprv%2Cmime%2Cns%2Ccnr%2Cratebypass%2Cdur%2Clmt&sig=AOq0QJ8wRgIhAPnFxir5ZDVxFvj2NnKDFzl5kOzzWdkoj0uZjX9wY5MIAiEA0zVLmGSFqzaM_lWTJypijQqYgXH5ty8BQ1kO8qIWwbI%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRgIhAKIEiFZTusoggn3yZAlFjoWDY93fJN8pkKnqLFD59TGKAiEAz67OafJRFRQYVOtWtbCk7Lyj2fQDgn2_xufvgh7Eufc%3D").await.expect("request failed");
    // let mut out = File::create("rust.mp4").expect("FAilled to create the output file");
    // let inp = resp.bytes().await.expect("Filed to read file bytes");
    // copy(&mut inp.as_ref(), &mut out).expect("failed to copy content");
    get_video_info().await;
}


async fn get_video_info() {
    let client = reqwest::Client::new();
    let res = client.post("https://youtubei.googleapis.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8").body(r#"
    {
        "context": {
          "client": {
           "hl": "en",
           "clientName": "WEB",
           "clientVersion": "2.20210721.00.00",
           "mainAppWebInfo": {
               "graftUrl": "/watch?v=TQDHGswF67Q"
           }
          }
         },
         "videoId": "TQDHGswF67Q"
       }
    "#).send().await.unwrap();

    println!("{}", res.text().await.unwrap());
}
