use anyhow::Result;
use opencv::{core, imgcodecs, imgproc, prelude::*};

pub fn speed_to_pace(speed: f32) -> String {
  if speed <= 0.0 {
    return String::from("0:00");
  }
  let pace_seconds = 1000.0 / speed;
  let minutes = (pace_seconds / 60.0).floor() as u32;
  let seconds = (pace_seconds % 60.0).round() as u32;
  format!("{}:{:02}", minutes, seconds)
}

pub fn semicircles_to_degrees(semicircles: i32) -> f64 {
  (semicircles as f64) * (180.0 / (i32::MAX as f64 + 1.0))
}

pub fn get_bounds(points: &Vec<(f64, f64)>) -> ((f64, f64), (f64, f64)) {
  let (lat_min, lat_max) = points.iter().fold(
    (f64::INFINITY, f64::NEG_INFINITY),
    |(mn, mx), (lat, _)| (mn.min(*lat), mx.max(*lat)),
  );

  let (lon_min, lon_max) = points.iter().fold(
    (f64::INFINITY, f64::NEG_INFINITY),
    |(mn, mx), (_, lon)| (mn.min(*lon), mx.max(*lon)),
  );

  ((lat_min, lat_max), (lon_min, lon_max))
}

pub fn load_and_resize_image(
  path: &str,
  max_dim: i32,
) -> Result<(Mat, i32, i32)> {
  // Load the background image
  let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)?;
  let size = img.size()?;
  let (orig_w, orig_h) = (size.width as f64, size.height as f64);

  // Compute scale factor to fit within max_dim
  let max_side = orig_w.max(orig_h);
  let scale = (max_dim as f64 / max_side).min(1.0);

  let width = (orig_w * scale) as i32;
  let height = (orig_h * scale) as i32;

  // Resize using high-quality interpolation
  let mut resized = Mat::default();
  imgproc::resize(
    &img,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  Ok((resized, width, height))
}
