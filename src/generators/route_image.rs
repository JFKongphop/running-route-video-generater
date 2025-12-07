use crate::configs::RouteImageConfig;
use crate::types::fit_data::RouteData;
use crate::utils::creator::image_creator;
use crate::utils::{
  converter::{get_bounds, load_and_resize_image},
  element_drawer::Drawer,
  read_file::fit_reader,
};
use anyhow::Result;
use opencv::{core, imgproc, prelude::*};

/// Generates a static route image from FIT file data.
///
/// Creates an image showing the complete route path overlaid on a background image.
///
/// # Arguments
/// * `route_scale` - Scale factor for route visualization (0.0-1.0 recommended)
/// * `offset_x_percent` - Horizontal offset as percentage of image width
/// * `offset_y_percent` - Vertical offset as percentage of image height
///
/// # Returns
/// * `Ok(())` - Image successfully created and saved
/// * `Err` - If FIT file reading, image loading, or drawing operations fail
///
/// # Output
/// Creates `outputs/route.png` with:
/// - Complete route path (red line)
/// - Route overlaid on background image
pub fn route_image(
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

  image_creator(output_file, &route_image)?;

  println!(
    "✅ Image created: {} with {} points",
    output_file,
    pixel_points.len()
  );

  Ok(())
}

/// Generates a static route image from FIT file data using custom configuration.
///
/// Creates an image showing the complete route path overlaid on a background image,
/// with customizable colors, scaling, and positioning.
///
/// # Arguments
/// * `config` - Route image configuration containing:
///   - `route_scale` - Scale and positioning settings
///   - `colors` - Color scheme for route elements
///   - `file_config` - Input/output file paths
///   - `line_thickness` - Thickness of the route line
///
/// # Returns
/// * `Ok(())` - Image successfully created and saved
/// * `Err` - If FIT file reading, image loading, or drawing operations fail
///
/// # Output
/// Creates an image file at the configured output path with:
/// - Complete route path with custom color and thickness
/// - Route overlaid on background image
/// - Customizable route positioning and scale
pub fn image_route_with_config(config: RouteImageConfig) -> Result<()> {
  // Read FIT file
  let (route, _lap) = fit_reader(&config.file_config.fit_file)?;
  let RouteData {
    paces: _,
    gps_points: points,
    distances: _,
  } = route;

  // -------- Normalize coordinates --------
  let ((lat_min, lat_max), (lon_min, lon_max)) = get_bounds(&points);

  // -------- Load background image ----------
  let (bg_image, width, height) = load_and_resize_image(
    &config.file_config.background_image,
    1080,
  )?;

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

    let x = ((config.route_scale.offset_x_percent
      + nx * config.route_scale.scale)
      * width as f64) as i32;
    let y = ((config.route_scale.offset_y_percent
      + (1.0 - ny) * config.route_scale.scale)
      * width as f64) as i32;
    core::Point::new(x, y)
  };

  let pixel_points: Vec<core::Point> =
    points.iter().map(|&(la, lo)| to_px(la, lo)).collect();

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

  // Draw route path with configured color
  let route_color = drawer.color(config.colors.route_line);
  let pts = core::Vector::<core::Point>::from_iter(pixel_points.clone());
  let mut all_pts = core::Vector::<core::Vector<core::Point>>::new();
  all_pts.push(pts);

  imgproc::polylines(
    &mut route_image,
    &all_pts,
    false,
    route_color,
    config.line_thickness,
    imgproc::LINE_AA,
    0,
  )?;

  // Save image
  image_creator(
    &config.file_config.output_file,
    &route_image,
  )?;

  println!(
    "✅ Image created: {} with {} points",
    config.file_config.output_file,
    pixel_points.len()
  );

  Ok(())
}
