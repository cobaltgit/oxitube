use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn download(
    app: AppHandle,
    url: String,
    quality: String,
    filter: String,
    out_file_path: String,
) -> Result<(), String> {
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

    let video =
        Video::new_with_options(url, opts).map_err(|e| format!("Failed to get video: {e}"))?;
    let info = video
        .get_info()
        .await
        .map_err(|e| format!("Failed to get video info: {e}"))?;

    let filename = info.video_details.title;
    let fullpath = Path::new(&out_file_path).join(filename);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&fullpath)
        .map_err(|e| format!("Failed to open file: {e}"))?;

    let stream = video
        .stream()
        .await
        .map_err(|e| format!("Failed to get video stream: {e}"))?;
    let stream_bytes = stream.content_length();

    let mut downloaded_bytes = 0;
    let mut progress = 0; // todo: possible progress bar?

    while let Some(chunk) = stream
        .chunk()
        .await
        .map_err(|e| format!("Failed to get stream chunk: {e}"))?
    {
        downloaded_bytes += file
            .write(&chunk)
            .map_err(|e| format!("Failed to write stream chunk: {e}"))?;
        progress = ((downloaded_bytes as f64 / stream_bytes as f64) * 100.0).round() as i64;
        app.emit("download-progress", progress).unwrap();
        println!("{:?}%", progress);
    }

    app.emit("download-complete", ()).unwrap();
    Ok(())
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
