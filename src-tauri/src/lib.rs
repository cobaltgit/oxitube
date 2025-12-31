use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[tauri::command]
async fn download(url: String, quality: String, filter: String, out_file_path: String) {
    let opts = VideoOptions {
        quality: match quality.as_str() {
            "lowest" => match filter.as_str() {
                "video" => VideoQuality::LowestVideo,
                "audio" => VideoQuality::LowestAudio,
                _ => VideoQuality::Lowest,
            },
            _ => match filter.as_str() {
                "video" => VideoQuality::HighestVideo,
                "audio" => VideoQuality::HighestAudio,
                _ => VideoQuality::Highest,
            },
        },
        filter: match filter.as_str() {
            "video" => VideoSearchOptions::Video,
            "audio" => VideoSearchOptions::Audio,
            _ => VideoSearchOptions::VideoAudio,
        },
        ..Default::default()
    };

    let video = Video::new_with_options(url, opts).unwrap();
    let info = video.get_info().await.unwrap();

    let filename = info.video_details.title;
    let fullpath = Path::new(&out_file_path).join(filename);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&fullpath)
        .unwrap();

    let stream = video.stream().await.unwrap();
    let stream_bytes = stream.content_length();

    let mut downloaded_bytes = 0;
    let mut progress = 0; // todo: possible progress bar?

    while let Some(chunk) = stream.chunk().await.unwrap() {
        downloaded_bytes += file.write(&chunk).unwrap();
        progress = ((downloaded_bytes as f64 / stream_bytes as f64) * 100.0).round() as i64;
        // update progress bar here
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
