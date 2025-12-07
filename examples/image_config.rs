/// Example: Static Route Image Generation
///
/// This example demonstrates how to generate a static route image
/// with custom configuration.
///
/// Required files:
/// - source/car.fit (your GPS data)
/// - source/car.jpg (background map image)
///
/// Output: outputs/route.png
use anyhow::Result;
use runarium::configs::image_config::RouteImageConfig;
use runarium::configs::video_config::{FileConfig, RouteColor, RouteScale};
use runarium::generators::route_image::image_route_with_config;

fn main() -> Result<()> {
  // Configure route scale and position
  let route_scale = RouteScale::new(
    0.2, // scale: 20% of map size
    0.1, // offset_x_percent: 10% from left
    0.1, // offset_y_percent: 10% from top
  );

  // Configure colors (BGRA format)
  let colors = RouteColor::new(
    [0.0, 0.0, 255.0, 0.0],     // route_line: Red
    [0.0, 255.0, 0.0, 0.0],     // current_position: Green
    [255.0, 255.0, 255.0, 0.0], // text: White
    [0.0, 165.0, 255.0, 0.0],   // lap_bars: Orange
  );

  // Configure file paths
  let file_config = FileConfig::new(
    "source/car.fit".to_string(),
    "source/car.jpg".to_string(),
    "outputs/route.png".to_string(),
  );

  // Create image configuration
  let config = RouteImageConfig::new(
    route_scale,
    colors,
    file_config,
    2, // line_thickness
  );

  image_route_with_config(config)?;
  Ok(())
}
