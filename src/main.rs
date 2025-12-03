use anyhow::Result;
use runarium::generators::route_video::{
  generate_progressive_route, generate_progressive_route_with_config,
};
use runarium::types::route_config::{
  Color, LapDataConfig, PaceDistConfig, RouteColor, RouteScale,
  RouteVideoConfig,
};
use runarium::utils::performance::measure;

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
    0.6,  // font_scale
    2,    // thickness
    None, // position (auto-calculated)
    true, // show_pace
    true, // show_distance
  );

  // Configure lap data panel
  // Note: font_scale (0.5), thickness (1), and bar_max_width (200) are fixed
  let lap_data = LapDataConfig::new(
    (0.5, 0.09),  // position (x_percent, y_percent)
    Color::White, // text_color: Choose from Color enum
    true,         // show_heart_rate
    true,         // show_stride_length
    true,         // show_pace_bars
  );

  // Combine all configurations
  let config = RouteVideoConfig::new(
    route_scale,
    colors,
    pace_dist,
    lap_data,
    true,  // show_bottom_bar
    true,  // show_route
    true,  // show_lap_data
    "source/car.fit".to_string(),      // fit_file
    "source/car.jpg".to_string(),      // background_image
    "outputs/config.mp4".to_string(),     // output_file
  );

  measure("Total execution", || {
    generate_progressive_route(
      route_scale.scale,
      route_scale.offset_x_percent,
      route_scale.offset_y_percent,
    );
    generate_progressive_route_with_config(config)
  })?;

  Ok(())
}
