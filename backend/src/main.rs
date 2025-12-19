mod converter;

use axum::{
    extract::{Query},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
    http::{StatusCode, header},
    body::Body,
};
use axum_extra::extract::Multipart;
use converter::VideoConverter;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use serde::Deserialize;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Deserialize)]
struct ConvertParams {
    format: Option<String>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/convert", post(convert_video));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(r#"
        <h1>Video Converter</h1>
        <form action="/convert" method="post" enctype="multipart/form-data">
            <label>
                Upload file:
                <input type="file" name="file" required>
            </label>
            <br>
            <label>
                Target Format (e.g., mp4, avi, gif):
                <input type="text" name="format" value="mp4" required>
            </label>
            <br>
            <input type="submit" value="Upload and Convert">
        </form>
    "#)
}

struct CleanupStream<S> {
    inner: S,
    path: PathBuf,
}

impl<S: Stream + Unpin> Stream for CleanupStream<S> {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

impl<S> Drop for CleanupStream<S> {
    fn drop(&mut self) {
        let path = self.path.clone();
        // Spawn a blocking task to remove the file
        let _ = tokio::task::spawn_blocking(move || {
             if let Err(e) = std::fs::remove_file(&path) {
                 eprintln!("Failed to remove temp file {:?}: {}", path, e);
             } else {
                 println!("Removed temp file {:?}", path);
             }
        });
    }
}

#[axum::debug_handler]
async fn convert_video(
    Query(params): Query<ConvertParams>,
    mut multipart: Multipart,
) -> Result<Response, (StatusCode, String)> {
    let ffmpeg_path = std::env::var("FFMPEG_PATH").unwrap_or_else(|_| "ffmpeg".to_string());

    // Create temp directory if it doesn't exist
    let temp_dir = "temp_uploads";
    fs::create_dir_all(temp_dir).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut input_file_path = PathBuf::new();
    let mut target_format = params.format.clone().unwrap_or_else(|| "mp4".to_string());

    // If format is passed as query param, use it. But form might send it as field.
    // Let's handle multipart fields.

    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        let name = field.name().unwrap_or("").to_string();

        if name == "format" {
            if let Ok(txt) = field.text().await {
                 if !txt.is_empty() {
                     target_format = txt;
                 }
            }
        } else if name == "file" {
            let file_name = field.file_name().unwrap_or("video.bin").to_string();
            let id = Uuid::new_v4();
            let sanitized_name = Path::new(&file_name).file_name().unwrap_or_default().to_string_lossy();
            input_file_path = PathBuf::from(temp_dir).join(format!("{}_{}", id, sanitized_name));

            let mut file = File::create(&input_file_path).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let mut stream = field;
            while let Some(chunk) = stream.chunk().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))? {
                file.write_all(&chunk).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            }
        }
    }

    if input_file_path.as_os_str().is_empty() {
         return Err((StatusCode::BAD_REQUEST, "No file uploaded".to_string()));
    }

    // Validate format
    if !target_format.chars().all(|c| c.is_alphanumeric()) {
         // Cleanup input file if format is invalid
         let _ = fs::remove_file(&input_file_path).await;
         return Err((StatusCode::BAD_REQUEST, "Invalid format".to_string()));
    }

    // Convert
    let output_filename = format!("{}.{}", input_file_path.file_stem().unwrap().to_string_lossy(), target_format);
    let output_path = PathBuf::from(temp_dir).join(&output_filename);

    let converter = VideoConverter::new(&ffmpeg_path);

    // Blocking call to convert (should be done in spawn_blocking for better concurrency, but keep simple for now)
    let input_path_clone = input_file_path.clone();
    let output_path_clone = output_path.clone();
    let format_clone = target_format.clone();

    let result = tokio::task::spawn_blocking(move || {
        converter.convert(&input_path_clone, &output_path_clone, &format_clone)
    }).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Cleanup input file immediately after conversion attempt
    let _ = fs::remove_file(&input_file_path).await;

    if let Err(e) = result {
        // Also try to cleanup output if it exists (partial write)
        let _ = fs::remove_file(&output_path).await;
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Conversion failed: {}", e)));
    }

    // Return the file
    let file = File::open(&output_path).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let stream = tokio_util::io::ReaderStream::new(file);

    // Wrap stream to cleanup file on drop
    let cleanup_stream = CleanupStream {
        inner: stream,
        path: output_path,
    };

    let body = Body::from_stream(cleanup_stream);

    let headers = [
        (header::CONTENT_TYPE, "application/octet-stream"),
        (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", output_filename)),
    ];

    Ok((headers, body).into_response())
}
