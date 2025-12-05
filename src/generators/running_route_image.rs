use crate::types::fit_data::RouteData;
use crate::utils::{
  converter::{get_bounds, load_and_resize_image},
  element_drawer::Drawer,
  read_file::fit_reader,
};
use anyhow::Result;
use opencv::{core, imgcodecs, imgproc, prelude::*};

pub fn generate_running_route_image(
  route_scale: f64,
  offset_x_percent: f64,
  offset_y_percent: f64,
) -> Result<()> {
  #[rustfmt::skip]
  let (route, _lap) = fit_reader("source/car.fit")?;
  let RouteData {
    paces: _,
    gps_points: points,
    distances: _,
  } = route;

  // -------- Normalize coordinates --------
  #[rustfmt::skip]
  let (
    (lat_min, lat_max),
    (lon_min, lon_max)
  ) = get_bounds(&points);

  // -------- Use background image ----------
  let (bg_image, width, height) =
    load_and_resize_image("source/car.jpg", 1080)?;
  let output_file = "outputs/route.png";

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

  // -------- Initialize image --------
  let mut resized = Mat::default();
  imgproc::resize(
    &bg_image,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  let mut route_image = resized.clone();
  let drawer = Drawer::new(width, height);

  // Draw route path
  let red_color = drawer.color([0.0, 0.0, 255.0, 0.0]);
  let green_color = drawer.color([0.0, 255.0, 0.0, 0.0]);

  let pts = core::Vector::<core::Point>::from_iter(pixel_points.clone());
  let mut all_pts = core::Vector::<core::Vector<core::Point>>::new();
  all_pts.push(pts);

  imgproc::polylines(
    &mut route_image,
    &all_pts,
    false,
    red_color,
    2,
    imgproc::LINE_AA,
    0,
  )?;

  // Save image
  imgcodecs::imwrite(
    output_file,
    &route_image,
    &core::Vector::new(),
  )?;

  println!(
    "✅ Image created: {} with {} points",
    output_file,
    pixel_points.len()
  );

  Ok(())
}
