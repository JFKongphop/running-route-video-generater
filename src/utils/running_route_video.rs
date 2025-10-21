use anyhow::Result;
#[rustfmt::skip]
use opencv::{
  core, 
  imgproc, 
  prelude::*, 
  videoio
};
use std::time::Instant;
#[rustfmt::skip]
use crate::utils::{
  converter::{
    get_bounds, 
    load_and_resize_image
  },
  element_drawer::RouteDrawer,
  read_file::fit_reader,
};

pub fn generate_running_route_video(
  route_scale: f64,
  offset_x_percent: f64,
  offset_y_percent: f64,
) -> Result<()> {
  let start = Instant::now();

  #[rustfmt::skip]
  let (
    paces, 
    points, 
    distances
  ) = fit_reader("source/data.fit")?;

  // -------- Normalize coordinates --------
  #[rustfmt::skip]
  let (
    (lat_min, lat_max), 
    (lon_min, lon_max)
  ) = get_bounds(&points);

  // -------- Use background image ----------
  let (bg_image, width, height) = load_and_resize_image("source/bg.jpg", 1080)?;
  let output_file = "outputs/running.mp4";

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
    #[rustfmt::skip]
    let y = ((offset_y_percent + (1.0 - ny) * route_scale) * width as f64) as i32;
    core::Point::new(x, y)
  };

  #[rustfmt::skip]
  let pixel_points: Vec<core::Point> = points
    .iter()
    .map(|&(la, lo)| to_px(la, lo))
    .collect();
  let fps = (pixel_points.len() / 15) as f64;

  // -------- Initialize video writer --------
  let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
  let video_size = core::Size::new(width, height);
  let mut video = videoio::VideoWriter::new(
    output_file,
    fourcc,
    fps,
    video_size,
    true,
  )?;

  // let mut path_frame = Mat::zeros(height, width, core::CV_8UC3)?.to_mat()?;
  let mut resized = Mat::default();
  imgproc::resize(
    &bg_image,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  let mut path_frame = resized.clone();
  let drawer = RouteDrawer::new(width, height);

  // -------- Progressive drawing --------
  for (i, point) in pixel_points.iter().enumerate() {
    if i > 0 {
      drawer.draw_line(
        &mut path_frame,
        pixel_points[i - 1],
        *point,
        core::Scalar::new(0.0, 0.0, 255.0, 0.0),
      )?;
    }

    let mut current_frame = path_frame.clone();
    drawer.draw_point(
      &mut current_frame,
      *point,
      core::Scalar::new(0.0, 255.0, 0.0, 0.0),
    )?;

    if i < paces.len() && i < distances.len() {
      let pace_text = format!("Pace: {} min/km", paces[i]);
      let dist_text = format!("Dist: {:.2} km", distances[i] / 1000.0);

      drawer.draw_text_bar(
        &mut current_frame,
        &pace_text,
        &dist_text,
      )?;
    }

    video.write(&current_frame)?;
    if (i + 1) % 100 == 0 {
      println!(
        "Processed {}/{} points",
        i + 1,
        pixel_points.len()
      );
    }
  }

  video.release()?;
  println!(
    "✅ Video created: {} with {} points",
    output_file,
    pixel_points.len()
  );
  println!(
    "⏱️ Time: {:.2}s",
    start.elapsed().as_secs_f64()
  );
  Ok(())
}
