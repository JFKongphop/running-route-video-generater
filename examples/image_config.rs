/// Example: Static Route Image Generation
///
/// This example demonstrates how to generate a static route image
/// with custom configuration including optional lap data display.
///
/// Required files:
/// - source/example.fit (your GPS data)
/// - source/example.jpg (background map image)
///
/// Output: outputs/route.png
use anyhow::Result;
use runarium::configs::image_config::RouteImageConfig;
use runarium::configs::video_config::{
  Color, FileConfig, Font, LapDataConfig, RouteColor, RouteScale,
};
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
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/route.png".to_string(),
  );

  // Configure lap data panel (optional)
  let lap_data = LapDataConfig::new(
    (0.5, 0.09),   // position (x_percent, y_percent)
    0.5,           // font_scale
    1,             // thickness
    Font::Simplex, // font style
    Color::White,  // text_color
    true,          // show_heart_rate
    true,          // show_stride_length
    true,          // show_pace_bars
  );

  // Create image configuration with lap data
  let config = RouteImageConfig::with_lap_data(
    route_scale,
    colors,
    file_config,
    2, // line_thickness
    lap_data,
  );

  image_route_with_config(config)?;
  Ok(())
}
