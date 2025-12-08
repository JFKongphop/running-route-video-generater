#![allow(unused)]

use anyhow::Result;
use runarium::{
  configs::{
    image_config::RouteImageConfig,
    video_config::{
      Color, FileConfig, Font, LapDataConfig, PaceDistConfig, RouteColor,
      RouteScale, RouteVideoConfig,
    },
  },
  generators::{
    route_image::{image_route_with_config, route_image},
    route_video::{progressive_route, progressive_route_with_config},
  },
  utils::performance::measure,
};

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
    Color::White,  // text_color: Choose from Color enum
    true,          // show_heart_rate
    true,          // show_stride_length
    true,          // show_pace_bars
  );

  // Configure file paths
  let video_file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/config.mp4".to_string(),
  );

  // Combine all video configurations
  let video_config = RouteVideoConfig::new(
    route_scale,
    colors,
    pace_dist,
    lap_data.clone(),
    video_file_config,
    true, // show_bottom_bar
    true, // show_route
    true, // show_lap_data
  );

  // Configure image file paths
  let image_file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/route.png".to_string(),
  );

  // Configure route image settings with lap data
  let image_config = RouteImageConfig::with_lap_data(
    route_scale,
    colors,
    image_file_config,
    2, // line_thickness
    lap_data,
  );

  measure("Total execution", || {
    // progressive_route(
    //   route_scale.scale,
    //   route_scale.offset_x_percent,
    //   route_scale.offset_y_percent,
    // )
    // progressive_route_with_config(video_config);
    image_route_with_config(image_config)
  })?;

  Ok(())
}
