use anyhow::Result;
use axum::{
  extract::{DefaultBodyLimit, Multipart, State},
  http::{header, StatusCode},
  response::{IntoResponse, Json},
  routing::{get, post},
  Router,
};
use runarium::{
  configs::{
    image_config::RouteImageConfig,
    video_config::{
      Color, FileConfig, Font, LapDataConfig, PaceDistConfig, RouteColor,
      RouteScale, RouteVideoConfig,
    },
  },
  generators::{
    route_image::image_route_with_config,
    route_video::progressive_route_with_config,
  },
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, sync::Arc, time::Instant};
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};
use uuid::Uuid;

// In-memory storage
type VideoStore = Arc<Mutex<HashMap<String, Vec<u8>>>>;
type ImageStore = Arc<Mutex<HashMap<String, Vec<u8>>>>;
type AppState = (VideoStore, ImageStore);
#[derive(Debug, Serialize)]
struct VideoResponse {
  success: bool,
  message: String,
  download_url: Option<String>,
  video_id: Option<String>,
  generation_time_ms: Option<u128>,
}

#[derive(Debug, Serialize)]
struct ImageResponse {
  success: bool,
  message: String,
  download_url: Option<String>,
  image_id: Option<String>,
  generation_time_ms: Option<u128>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
  error: String,
}

#[derive(Debug, Deserialize)]
struct VideoConfigParams {
  // Route scale
  #[serde(default = "default_scale")]
  scale: f64,
  #[serde(default = "default_offset_x")]
  offset_x_percent: f64,
  #[serde(default = "default_offset_y")]
  offset_y_percent: f64,
  
  // Colors (BGRA format)
  #[serde(default = "default_route_line_color")]
  route_line_color: [f64; 4],
  #[serde(default = "default_current_position_color")]
  current_position_color: [f64; 4],
  #[serde(default = "default_text_color")]
  text_color: [f64; 4],
  #[serde(default = "default_lap_bars_color")]
  lap_bars_color: [f64; 4],
  
  // Pace/Distance config
  #[serde(default = "default_pace_font_scale")]
  pace_font_scale: f64,
  #[serde(default = "default_pace_thickness")]
  pace_thickness: i32,
  
  // Lap data config
  #[serde(default = "default_lap_position_x")]
  lap_position_x: f64,
  #[serde(default = "default_lap_position_y")]
  lap_position_y: f64,
  #[serde(default = "default_lap_font_scale")]
  lap_font_scale: f64,
  #[serde(default = "default_lap_thickness")]
  lap_thickness: i32,
  
  // Display options
  #[serde(default = "default_true")]
  show_bottom_bar: bool,
  #[serde(default = "default_true")]
  show_route: bool,
  #[serde(default = "default_true")]
  show_lap_data: bool,
  #[serde(default = "default_true")]
  show_pace: bool,
  #[serde(default = "default_true")]
  show_distance: bool,
  #[serde(default = "default_true")]
  show_heart_rate: bool,
  #[serde(default = "default_true")]
  show_stride_length: bool,
  #[serde(default = "default_true")]
  show_pace_bars: bool,
}

fn default_scale() -> f64 { 0.2 }
fn default_offset_x() -> f64 { 0.1 }
fn default_offset_y() -> f64 { 0.1 }
fn default_route_line_color() -> [f64; 4] { [0.0, 0.0, 255.0, 0.0] } // Red
fn default_current_position_color() -> [f64; 4] { [0.0, 255.0, 0.0, 0.0] } // Green
fn default_text_color() -> [f64; 4] { [255.0, 255.0, 255.0, 0.0] } // White
fn default_lap_bars_color() -> [f64; 4] { [0.0, 165.0, 255.0, 0.0] } // Orange
fn default_pace_font_scale() -> f64 { 0.6 }
fn default_pace_thickness() -> i32 { 2 }
fn default_lap_position_x() -> f64 { 0.5 }
fn default_lap_position_y() -> f64 { 0.09 }
fn default_lap_font_scale() -> f64 { 0.5 }
fn default_lap_thickness() -> i32 { 1 }
fn default_true() -> bool { true }

// Health check endpoint
async fn health_check() -> &'static str {
  "OK"
}

// Generate video from uploaded files
async fn generate_video(
  State(state): State<AppState>,
  mut multipart: Multipart,
) -> Result<Json<VideoResponse>, (StatusCode, Json<ErrorResponse>)> {
  let store = &state.0; // video store
  // Generate unique ID for this video
  let video_id = Uuid::new_v4().to_string();
  
  // Use system temp directory for production compatibility
  let temp_base = env::temp_dir();
  let temp_dir = temp_base.join(format!("runarium_{}", video_id));

  // Create temp directory for processing only
  fs::create_dir_all(&temp_dir).map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Failed to create temp directory: {}", e),
      }),
    )
  })?;

  let mut fit_data = None;
  let mut background_data = None;
  let mut config_data: Option<String> = None;

  // Process uploaded files - keep in memory
  while let Some(field) = multipart.next_field().await.unwrap() {
    let name = field.name().unwrap_or("").to_string();
    let data = field.bytes().await.unwrap();

    match name.as_str() {
      "fit_file" => fit_data = Some(data),
      "background" => background_data = Some(data),
      "config" => config_data = Some(String::from_utf8_lossy(&data).to_string()),
      _ => {}
    }
  }

  // Parse config or use defaults
  let config_params: VideoConfigParams = match config_data {
    Some(json_str) => serde_json::from_str(&json_str).unwrap_or_else(|_| VideoConfigParams {
      scale: default_scale(),
      offset_x_percent: default_offset_x(),
      offset_y_percent: default_offset_y(),
      route_line_color: default_route_line_color(),
      current_position_color: default_current_position_color(),
      text_color: default_text_color(),
      lap_bars_color: default_lap_bars_color(),
      pace_font_scale: default_pace_font_scale(),
      pace_thickness: default_pace_thickness(),
      lap_position_x: default_lap_position_x(),
      lap_position_y: default_lap_position_y(),
      lap_font_scale: default_lap_font_scale(),
      lap_thickness: default_lap_thickness(),
      show_bottom_bar: default_true(),
      show_route: default_true(),
      show_lap_data: default_true(),
      show_pace: default_true(),
      show_distance: default_true(),
      show_heart_rate: default_true(),
      show_stride_length: default_true(),
      show_pace_bars: default_true(),
    }),
    None => VideoConfigParams {
      scale: default_scale(),
      offset_x_percent: default_offset_x(),
      offset_y_percent: default_offset_y(),
      route_line_color: default_route_line_color(),
      current_position_color: default_current_position_color(),
      text_color: default_text_color(),
      lap_bars_color: default_lap_bars_color(),
      pace_font_scale: default_pace_font_scale(),
      pace_thickness: default_pace_thickness(),
      lap_position_x: default_lap_position_x(),
      lap_position_y: default_lap_position_y(),
      lap_font_scale: default_lap_font_scale(),
      lap_thickness: default_lap_thickness(),
      show_bottom_bar: default_true(),
      show_route: default_true(),
      show_lap_data: default_true(),
      show_pace: default_true(),
      show_distance: default_true(),
      show_heart_rate: default_true(),
      show_stride_length: default_true(),
      show_pace_bars: default_true(),
    },
  };

  // Validate both files are uploaded
  let fit_bytes = fit_data.ok_or((
    StatusCode::BAD_REQUEST,
    Json(ErrorResponse {
      error: "Missing fit_file in request".to_string(),
    }),
  ))?;

  let background_bytes = background_data.ok_or((
    StatusCode::BAD_REQUEST,
    Json(ErrorResponse {
      error: "Missing background image in request".to_string(),
    }),
  ))?;

  // Write files temporarily for processing
  let fit_path = temp_dir.join("data.fit");
  let bg_path = temp_dir.join("background.jpg");
  let output_path = temp_dir.join("output.mp4");

  // Save temporarily
  write_bytes(&fit_path.to_string_lossy(), &fit_bytes).await.map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Failed to write temp FIT file: {}", e),
      }),
    )
  })?;

  write_bytes(&bg_path.to_string_lossy(), &background_bytes)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
          error: format!("Failed to write temp background: {}", e),
        }),
      )
    })?;

  // Create configuration with temp paths (using params from request)
  let route_scale = RouteScale::new(
    config_params.scale,
    config_params.offset_x_percent,
    config_params.offset_y_percent,
  );

  let colors = RouteColor::new(
    config_params.route_line_color,
    config_params.current_position_color,
    config_params.text_color,
    config_params.lap_bars_color,
  );

  let pace_dist = PaceDistConfig::new(
    config_params.pace_font_scale,
    config_params.pace_thickness,
    Font::Simplex, // font style
    None,          // position (auto-calculated)
    config_params.show_pace,
    config_params.show_distance,
  );

  let lap_data = LapDataConfig::new(
    (config_params.lap_position_x, config_params.lap_position_y),
    config_params.lap_font_scale,
    config_params.lap_thickness,
    Font::Simplex, // font style
    Color::White,  // text_color
    config_params.show_heart_rate,
    config_params.show_stride_length,
    config_params.show_pace_bars,
  );

  let file_config = FileConfig::new(
    fit_path.to_string_lossy().to_string(),
    bg_path.to_string_lossy().to_string(),
    output_path.to_string_lossy().to_string(),
  );

  let config = RouteVideoConfig::new(
    route_scale,
    colors,
    pace_dist,
    lap_data,
    file_config,
    config_params.show_bottom_bar,
    config_params.show_route,
    config_params.show_lap_data,
  );

  // Generate video (blocking operation) - track time
  let start_time = Instant::now();
  let video_result = tokio::task::spawn_blocking(move || {
    progressive_route_with_config(config)
  })
  .await
  .map_err(|e| {
    let _ = fs::remove_dir_all(&temp_dir);
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Task execution failed: {}", e),
      }),
    )
  })?;

  match video_result {
    Ok(_) => {
      let generation_time = start_time.elapsed().as_millis() / 1000;
      
      // Read generated video into memory
      let video_data = fs::read(&output_path).map_err(|e| {
        let _ = fs::remove_dir_all(&temp_dir);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(ErrorResponse {
            error: format!("Failed to read generated video: {}", e),
          }),
        )
      })?;

      // Store video in memory
      {
        let mut videos = store.lock().await;
        videos.insert(video_id.clone(), video_data);
      }

      // Clean up temp directory immediately
      let _ = fs::remove_dir_all(&temp_dir);

      Ok(Json(VideoResponse {
        success: true,
        message: "Video generated successfully".to_string(),
        download_url: Some(format!("/download-video/{}", video_id)),
        video_id: Some(video_id),
        generation_time_ms: Some(generation_time),
      }))
    }
    Err(e) => {
      // Clean up on error
      let _ = fs::remove_dir_all(&temp_dir);

      Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
          error: format!("Video generation failed: {}", e),
        }),
      ))
    }
  }
}

// Generate image from uploaded files
async fn generate_image(
  State(state): State<AppState>,
  mut multipart: Multipart,
) -> Result<Json<ImageResponse>, (StatusCode, Json<ErrorResponse>)> {
  let store = &state.1; // image store
  // Generate unique ID for this image
  let image_id = Uuid::new_v4().to_string();
  
  // Use system temp directory for production compatibility
  let temp_base = env::temp_dir();
  let temp_dir = temp_base.join(format!("runarium_img_{}", image_id));

  // Create temp directory for processing only
  fs::create_dir_all(&temp_dir).map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Failed to create temp directory: {}", e),
      }),
    )
  })?;

  let mut fit_data = None;
  let mut background_data = None;
  let mut config_data: Option<String> = None;

  // Process uploaded files
  while let Some(field) = multipart.next_field().await.unwrap() {
    let name = field.name().unwrap_or("").to_string();
    let data = field.bytes().await.unwrap();

    match name.as_str() {
      "fit_file" => fit_data = Some(data),
      "background" => background_data = Some(data),
      "config" => config_data = Some(String::from_utf8_lossy(&data).to_string()),
      _ => {}
    }
  }

  // Parse config or use defaults
  let config_params: VideoConfigParams = match config_data {
    Some(json_str) => serde_json::from_str(&json_str).unwrap_or_else(|_| VideoConfigParams {
      scale: default_scale(),
      offset_x_percent: default_offset_x(),
      offset_y_percent: default_offset_y(),
      route_line_color: default_route_line_color(),
      current_position_color: default_current_position_color(),
      text_color: default_text_color(),
      lap_bars_color: default_lap_bars_color(),
      pace_font_scale: default_pace_font_scale(),
      pace_thickness: default_pace_thickness(),
      lap_position_x: default_lap_position_x(),
      lap_position_y: default_lap_position_y(),
      lap_font_scale: default_lap_font_scale(),
      lap_thickness: default_lap_thickness(),
      show_bottom_bar: default_true(),
      show_route: default_true(),
      show_lap_data: default_true(),
      show_pace: default_true(),
      show_distance: default_true(),
      show_heart_rate: default_true(),
      show_stride_length: default_true(),
      show_pace_bars: default_true(),
    }),
    None => VideoConfigParams {
      scale: default_scale(),
      offset_x_percent: default_offset_x(),
      offset_y_percent: default_offset_y(),
      route_line_color: default_route_line_color(),
      current_position_color: default_current_position_color(),
      text_color: default_text_color(),
      lap_bars_color: default_lap_bars_color(),
      pace_font_scale: default_pace_font_scale(),
      pace_thickness: default_pace_thickness(),
      lap_position_x: default_lap_position_x(),
      lap_position_y: default_lap_position_y(),
      lap_font_scale: default_lap_font_scale(),
      lap_thickness: default_lap_thickness(),
      show_bottom_bar: default_true(),
      show_route: default_true(),
      show_lap_data: default_true(),
      show_pace: default_true(),
      show_distance: default_true(),
      show_heart_rate: default_true(),
      show_stride_length: default_true(),
      show_pace_bars: default_true(),
    },
  };

  // Validate both files are uploaded
  let fit_bytes = fit_data.ok_or((
    StatusCode::BAD_REQUEST,
    Json(ErrorResponse {
      error: "Missing fit_file in request".to_string(),
    }),
  ))?;

  let background_bytes = background_data.ok_or((
    StatusCode::BAD_REQUEST,
    Json(ErrorResponse {
      error: "Missing background image in request".to_string(),
    }),
  ))?;

  // Write files temporarily for processing
  let fit_path = temp_dir.join("data.fit");
  let bg_path = temp_dir.join("background.jpg");
  let output_path = temp_dir.join("output.png");

  // Save temporarily
  write_bytes(&fit_path.to_string_lossy(), &fit_bytes).await.map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Failed to write temp FIT file: {}", e),
      }),
    )
  })?;

  write_bytes(&bg_path.to_string_lossy(), &background_bytes)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
          error: format!("Failed to write temp background: {}", e),
        }),
      )
    })?;

  // Create configuration for image
  let route_scale = RouteScale::new(
    config_params.scale,
    config_params.offset_x_percent,
    config_params.offset_y_percent,
  );

  let colors = RouteColor::new(
    config_params.route_line_color,
    config_params.current_position_color,
    config_params.text_color,
    config_params.lap_bars_color,
  );

  let lap_data = LapDataConfig::new(
    (config_params.lap_position_x, config_params.lap_position_y),
    config_params.lap_font_scale,
    config_params.lap_thickness,
    Font::Simplex,
    Color::White,
    config_params.show_heart_rate,
    config_params.show_stride_length,
    config_params.show_pace_bars,
  );

  let file_config = FileConfig::new(
    fit_path.to_string_lossy().to_string(),
    bg_path.to_string_lossy().to_string(),
    output_path.to_string_lossy().to_string(),
  );

  let config = RouteImageConfig::with_lap_data(
    route_scale,
    colors,
    file_config,
    2, // line_thickness
    lap_data,
  );

  // Generate image (blocking operation) - track time
  let start_time = Instant::now();
  let image_result = tokio::task::spawn_blocking(move || {
    image_route_with_config(config)
  })
  .await
  .map_err(|e| {
    let _ = fs::remove_dir_all(&temp_dir);
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        error: format!("Task execution failed: {}", e),
      }),
    )
  })?;

  match image_result {
    Ok(_) => {
      let generation_time = start_time.elapsed().as_millis() / 1000;
      
      // Read generated image into memory
      let image_data = fs::read(&output_path).map_err(|e| {
        let _ = fs::remove_dir_all(&temp_dir);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(ErrorResponse {
            error: format!("Failed to read generated image: {}", e),
          }),
        )
      })?;

      // Store image in memory
      {
        let mut images = store.lock().await;
        images.insert(image_id.clone(), image_data);
      }

      // Clean up temp directory immediately
      let _ = fs::remove_dir_all(&temp_dir);

      Ok(Json(ImageResponse {
        success: true,
        message: "Image generated successfully".to_string(),
        download_url: Some(format!("/download-image/{}", image_id)),
        image_id: Some(image_id),
        generation_time_ms: Some(generation_time),
      }))
    }
    Err(e) => {
      // Clean up on error
      let _ = fs::remove_dir_all(&temp_dir);

      Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
          error: format!("Image generation failed: {}", e),
        }),
      ))
    }
  }
}

// Download generated video (one-time download, then remove)
async fn download_video(
  State(state): State<AppState>,
  axum::extract::Path(video_id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
  let store = &state.0; // video store
  // Get and remove video from memory in one operation
  let video_data = {
    let mut videos = store.lock().await;
    videos.remove(&video_id)
  };

  match video_data {
    Some(data) => {
      let mut headers = axum::http::HeaderMap::new();
      headers.insert(header::CONTENT_TYPE, "video/mp4".parse().unwrap());
      headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"route-{}.mp4\"", video_id)
          .parse()
          .unwrap(),
      );
      Ok((headers, data))
    }
    None => Err((
      StatusCode::NOT_FOUND,
      Json(ErrorResponse {
        error: "Video not found or already downloaded".to_string(),
      }),
    )),
  }
}

// Download generated image (one-time download, then remove)
async fn download_image(
  State(state): State<AppState>,
  axum::extract::Path(image_id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
  let store = &state.1; // image store
  // Get and remove image from memory in one operation
  let image_data = {
    let mut images = store.lock().await;
    images.remove(&image_id)
  };

  match image_data {
    Some(data) => {
      let mut headers = axum::http::HeaderMap::new();
      headers.insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
      headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"route-{}.png\"", image_id)
          .parse()
          .unwrap(),
      );
      Ok((headers, data))
    }
    None => Err((
      StatusCode::NOT_FOUND,
      Json(ErrorResponse {
        error: "Image not found or already downloaded".to_string(),
      }),
    )),
  }
}

// Helper function to write bytes to file
async fn write_bytes(path: &str, data: &[u8]) -> Result<()> {
  let mut file = File::create(path).await?;
  file.write_all(data).await?;
  file.flush().await?;
  Ok(())
}

#[tokio::main]
async fn main() {
  // In-memory storage
  let video_store: VideoStore = Arc::new(Mutex::new(HashMap::new()));
  let image_store: ImageStore = Arc::new(Mutex::new(HashMap::new()));

  let app = Router::new()
    .route("/", get(health_check))
    .route("/health", get(health_check))
    .route("/generate-video", post(generate_video))
    .route("/generate-image", post(generate_image))
    .route("/download-video/:video_id", get(download_video))
    .route("/download-image/:image_id", get(download_image))
    .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100MB limit
    .with_state((video_store, image_store));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

  println!("🚀 Server running on http://localhost:3000");
  println!("📍 Endpoints:");
  println!("   POST /generate-video - Upload files and generate video");
  println!("   POST /generate-image - Upload files and generate route image");
  println!("   GET  /download-video/:video_id - Download generated video (one-time)");
  println!("   GET  /download-image/:image_id - Download generated image (one-time)");
  println!("   GET  /health - Health check");
  println!();
  println!("⚠️  Files are stored in memory and deleted after download");

  axum::serve(listener, app).await.unwrap();
}
