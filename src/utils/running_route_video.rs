#![allow(unused_variables)]
use anyhow::{Context, Result};
use fitparser::{Value, profile::MesgNum};
use opencv::{core, imgcodecs, imgproc, prelude::*, videoio};
use std::fs::File;
use std::time::Instant;

fn draw_rec(dist_text: &str, height: f64, width: f64, current_frame: &mut Mat) {
  let font = imgproc::FONT_HERSHEY_SIMPLEX;
  let font_scale = 0.8;
  let thickness = 2;
  let margin_x = 40;
  let margin_y = 30;
  let text_color = core::Scalar::new(255.0, 255.0, 255.0, 0.0); // white
  let bg_color = core::Scalar::new(0.0, 0.0, 0.0, 0.0); // black
  let alpha = 0.5; // transparency

  // Measure text height
  let text_size = imgproc::get_text_size(&dist_text, font, font_scale, thickness, &mut 0).unwrap();
  let bar_height = text_size.height + margin_y;

  // --- Draw bottom background bar ---
  let bg_top_left = core::Point::new(0, (height as i32) - bar_height);
  let bg_bottom_right = core::Point::new(width as i32, height as i32);
  let rect = core::Rect::new(
    bg_top_left.x,
    bg_top_left.y,
    bg_bottom_right.x - bg_top_left.x,
    bg_bottom_right.y - bg_top_left.y,
  );
  imgproc::rectangle(current_frame, rect, bg_color, -1, imgproc::LINE_8, 0).unwrap();
}

fn speed_to_pace(speed: f32) -> String {
  if speed <= 0.0 {
    return String::from("0:00");
  }
  let pace_seconds = 1000.0 / speed;
  let minutes = (pace_seconds / 60.0).floor() as u32;
  let seconds = (pace_seconds % 60.0).round() as u32;
  format!("{}:{:02}", minutes, seconds)
}

// semicircle -> degrees
fn semicircles_to_degrees(semicircles: i32) -> f64 {
  (semicircles as f64) * (180.0 / (i32::MAX as f64 + 1.0))
}

fn read_fit_file(file_path: &str) -> Result<(Vec<String>, Vec<(f64, f64)>, Vec<f64>)> {
  let mut paces = Vec::new();
  let mut gps_points = Vec::new();
  let mut distances = Vec::new();
  let mut fp = File::open(file_path)?;
  for data in fitparser::from_reader(&mut fp)? {
    if data.kind() == MesgNum::Record {
      let mut lat: Option<f64> = None;
      let mut lon: Option<f64> = None;
      let mut pace: Option<String> = None;

      for field in data.fields() {
        match field.name() {
          "enhanced_speed" => match field.value() {
            Value::Float32(v) => pace = Some(speed_to_pace(*v)),
            Value::Float64(v) => pace = Some(speed_to_pace(*v as f32)),
            _ => {}
          },
          "position_lat" => {
            if let Value::SInt32(v) = field.value() {
              lat = Some(semicircles_to_degrees(*v));
            }
          }
          "position_long" => {
            if let Value::SInt32(v) = field.value() {
              lon = Some(semicircles_to_degrees(*v));
            }
          }
          "distance" => {
            if let Value::Float64(v) = field.value() {
              distances.push(*v);
            }
          }
          _ => {}
        }
      }

      if let (Some(lat), Some(lon), Some(pace)) = (lat, lon, pace) {
        gps_points.push((lat, lon));
        paces.push(pace);
      }
    }
  }
  Ok((paces, gps_points, distances))
}

pub fn generate_running_route_video() -> Result<()> {
  let start = Instant::now();

  let (paces, points, distances) = read_fit_file("source/data.fit")?;

  // -------- Normalize coordinates --------
  let (lat_min, lat_max) = points
    .iter()
    .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), (lat, _)| {
      (mn.min(*lat), mx.max(*lat))
    });
  let (lon_min, lon_max) = points
    .iter()
    .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), (_, lon)| {
      (mn.min(*lon), mx.max(*lon))
    });

  // -------- Use background image ----------
  let bg_image = imgcodecs::imread("source/bg.jpg", imgcodecs::IMREAD_COLOR)?;
  let size = bg_image.size()?;
  let (orig_w, orig_h) = (size.width, size.height);

  // Choose a target resolution (keeping aspect ratio)
  let max_dim = 1080; // desired maximum side
  let max_side = orig_w.max(orig_h) as f64;
  let scale = (max_dim as f64 / max_side).min(1.0);
  let width = (orig_w as f64 * scale) as i32; 
  let height = (orig_h as f64 * scale) as i32;
  let output_file = "outputs/running.mp4";

  // -------- Convert all points to pixel coords --------
  // --- Define route scaling and position (percentages) ---
  let route_scale = 0.2; // route will take up 80% of image dimension
  let offset_x_percent = 0.1; // start drawing at 10% from left
  let offset_y_percent = 0.1; // start drawing at 10% from top

  // --- Coordinate normalization to image space ---
  let to_px = |lat: f64, lon: f64| -> core::Point {
    let nx = if lon_max != lon_min {
      (lon - lon_min) / (lon_max - lon_min)
    } else {
      0.5
    };
    let ny = if lat_max != lat_min {
      (lat - lat_min) / (lat_max - lat_min)
    } else {
      0.5
    };

    let x = ((offset_x_percent + nx * route_scale) * width as f64) as i32;
    let y = ((offset_y_percent + (1.0 - ny) * route_scale) * width as f64) as i32;
    core::Point::new(x, y)
  };

  let pixel_points: Vec<core::Point> = points.iter().map(|&(la, lo)| to_px(la, lo)).collect();
  let fps = (pixel_points.len() / 15) as f64;

  // -------- Initialize video writer --------
  let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
  let mut video = videoio::VideoWriter::new(
    output_file,
    fourcc,
    fps,
    core::Size::new(width, height),
    true,
  )
  .context("creating video writer")?;

  // -------- Create persistent frame (accumulated path) --------
  // Optional for black bg 
  // let mut path_frame = Mat::zeros(height, width, core::CV_8UC3)?.to_mat()?;

  let mut resized = Mat::default();

  // Perform resize safely (source and destination are different)
  imgproc::resize(
    &bg_image,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  // Now use the resized image
  let mut path_frame = resized.clone();

  // -------- Progressive drawing --------
  for (i, point) in pixel_points.iter().enumerate() {
    if i > 0 {
      imgproc::line(
        &mut path_frame,
        pixel_points[i - 1],
        *point,
        core::Scalar::new(0.0, 0.0, 255.0, 0.0), // red line
        4,
        imgproc::LINE_AA,
        0,
      )?;
    }

    // Clone the accumulated path image
    let mut current_frame = path_frame.clone();

    // Draw current point marker
    imgproc::circle(
      &mut current_frame,
      *point,
      8,
      core::Scalar::new(0.0, 255.0, 0.0, 0.0), // green
      -1,
      imgproc::LINE_AA,
      0,
    )?;

    // --- 🟢 Add pace and distance text ---
    if i < paces.len() && i < distances.len() {
      let pace_text = format!("Pace: {} min/km", paces[i]);
      let dist_text = format!("Dist: {:.2} km", distances[i] / 1000.0);

      // Get text sizes
      let font = imgproc::FONT_HERSHEY_SIMPLEX;
      let font_scale = 0.7;
      let thickness = 2;
      let margin = 20;

      // Bottom-left corner
      let y_text = height - margin;

      draw_rec(&dist_text, height as f64, width as f64, &mut current_frame);

      imgproc::put_text(
        &mut current_frame,
        &pace_text,
        core::Point::new(margin, y_text),
        font,
        font_scale,
        core::Scalar::new(255.0, 255.0, 255.0, 0.0), // white
        thickness,
        imgproc::LINE_AA,
        false,
      )?;

      // Bottom-right corner
      let mut baseline = 0;
      let text_size =
        imgproc::get_text_size(&dist_text, font, font_scale, thickness, &mut baseline)?;
      let x_right = width - text_size.width - margin;

      imgproc::put_text(
        &mut current_frame,
        &dist_text,
        core::Point::new(x_right, y_text),
        font,
        font_scale,
        core::Scalar::new(255.0, 255.0, 255.0, 0.0),
        thickness,
        imgproc::LINE_AA,
        false,
      )?;
    }

    // Write frame to video
    video.write(&current_frame)?;

    if (i + 1) % 100 == 0 {
      println!("Processed {}/{} points", i + 1, pixel_points.len());
    }
  }

  video.release()?;
  println!(
    "✅ Video created: {} with {} points",
    output_file,
    pixel_points.len()
  );
  println!("⏱️ Time: {:.2}s", start.elapsed().as_secs_f64());
  Ok(())
}
