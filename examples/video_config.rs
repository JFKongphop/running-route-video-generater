/// Example: Progressive Route Video Generation
///
/// This example demonstrates how to generate an animated route video
/// with custom configuration.
///
/// Required files:
/// - source/example.fit (your GPS data)
/// - source/example.jpg (background map image)
///
/// Output: outputs/video.mp4
use anyhow::Result;
use runarium::configs::video_config::{
  Color, FileConfig, Font, LapDataConfig, PaceDistConfig, RouteColor,
  RouteScale, RouteVideoConfig,
};
use runarium::generators::route_video::progressive_route_with_config;

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

  // Configure pace and distance display
  let pace_dist = PaceDistConfig::new(
    0.6,           // font_scale
    2,             // thickness
    Font::Simplex, // font style
    None,          // position (auto-calculated)
    true,          // show_pace
    true,          // show_distance
  );

  // Configure lap data panel
  let lap_data = LapDataConfig::new(
    (0.5, 0.09),   // position (x_percent, y_percent)
    0.5,           // font_scale
    1,             // thickness
    Font::Simplex, // font style
    Color::White,  // text_color
    200,           // bar_max_width
    true,          // show_heart_rate
    true,          // show_stride_length
    true,          // show_pace_bars
  );

  // Configure file paths
  let file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/video.mp4".to_string(),
  );

  // Combine all configurations
  let config = RouteVideoConfig::new(
    route_scale,
    colors,
    pace_dist,
    lap_data,
    file_config,
    true, // show_bottom_bar
    true, // show_route
    true, // show_lap_data
  );

  progressive_route_with_config(config)?;
  Ok(())
}
