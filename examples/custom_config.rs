/// Example: Using custom configuration types for route video generation
///
/// This example demonstrates how to use the new configuration types:
/// - RouteScale: Control route size and position
/// - RouteColor: Customize colors
/// - PaceDistConfig: Configure pace/distance display
/// - LapDataConfig: Configure lap statistics panel (with Color enum)
/// - RouteVideoConfig: Complete configuration with visibility flags
///
/// Required files:
/// - source/car.fit (your GPS data)
/// - source/car.jpg (background map image)
///
/// Output: outputs/car.mp4
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route_with_config;
use runarium::types::route_config::{
  Color, LapDataConfig, PaceDistConfig, RouteColor, RouteScale,
  RouteVideoConfig,
};
use runarium::utils::performance::measure;

fn main() -> Result<()> {
  println!("🎨 Generating video with custom configuration...\n");

  // Example 1: Using preset configurations
  println!("📍 Example 1: Default configuration");
  let mut config = RouteVideoConfig::default();
  config.fit_file = "source/car.fit".to_string();
  config.background_image = "source/car.jpg".to_string();
  config.output_file = "outputs/example1_default.mp4".to_string();
  measure("Default config", || {
    generate_progressive_route_with_config(config)
  })?;

  // Example 2: Minimalist configuration
  println!("\n📍 Example 2: Minimalist configuration");
  let mut config = RouteVideoConfig::minimalist();
  config.fit_file = "source/car.fit".to_string();
  config.background_image = "source/car.jpg".to_string();
  config.output_file = "outputs/example2_minimalist.mp4".to_string();
  measure("Minimalist config", || {
    generate_progressive_route_with_config(config)
  })?;

  // Example 3: Detailed configuration
  println!("\n📍 Example 3: Detailed configuration");
  let mut config = RouteVideoConfig::detailed();
  config.fit_file = "source/car.fit".to_string();
  config.background_image = "source/car.jpg".to_string();
  config.output_file = "outputs/example3_detailed.mp4".to_string();
  measure("Detailed config", || {
    generate_progressive_route_with_config(config)
  })?;

  // Example 4: Neon theme
  println!("\n📍 Example 4: Neon theme");
  let mut config = RouteVideoConfig::neon();
  config.fit_file = "source/car.fit".to_string();
  config.background_image = "source/car.jpg".to_string();
  config.output_file = "outputs/example4_neon.mp4".to_string();
  measure("Neon theme", || {
    generate_progressive_route_with_config(config)
  })?;

  // Example 5: Fully custom configuration
  println!("\n📍 Example 5: Fully custom configuration");

  // Custom route scale - large, centered
  let route_scale = RouteScale::new(
    0.6, // 60% of map size
    0.2, // 20% from left
    0.2, // 20% from top
  );

  // Custom colors - blue theme
  let colors = RouteColor::new(
    [255.0, 100.0, 0.0, 0.0],   // Blue route
    [0.0, 255.0, 255.0, 0.0],   // Yellow marker
    [255.0, 255.0, 255.0, 0.0], // White text
    [200.0, 200.0, 0.0, 0.0],   // Cyan bars
  );

  // Custom pace/distance display
  let pace_dist = PaceDistConfig::new(
    0.7,  // Larger font (not used, fixed at 0.5)
    2,    // Thicker text (not used, fixed at 1)
    None, // Auto position
    true, // Show pace
    true, // Show distance
  );

  // Custom lap data panel
  // Note: font_scale, thickness, bar_max_width are fixed (0.5, 1, 200)
  let lap_data = LapDataConfig::new(
    (0.5, 0.07),        // Position as percentage (50% x, 7% y)
    Color::YellowGreen, // Text color from Color enum
    true,               // Show heart rate
    true,               // Show stride length
    true,               // Show pace bars
  );

  let config = RouteVideoConfig::new(
    route_scale,
    colors,
    pace_dist,
    lap_data,
    true, // show_bottom_bar
    true, // show_route
    true, // show_lap_data
    "source/car.fit".to_string(),
    "source/car.jpg".to_string(),
    "outputs/example5_custom.mp4".to_string(),
  );

  measure("Custom config", || {
    generate_progressive_route_with_config(config)
  })?;

  // Example 6: Hide specific elements
  println!("\n📍 Example 6: Route only (no stats)");
  let mut config = RouteVideoConfig::default();
  config.fit_file = "source/car.fit".to_string();
  config.background_image = "source/car.jpg".to_string();
  config.output_file = "outputs/example6_route_only.mp4".to_string();
  config.show_bottom_bar = false; // Hide pace/distance bar
  config.show_lap_data = false; // Hide lap statistics
  measure("Route only", || {
    generate_progressive_route_with_config(config)
  })?;

  println!("\n✅ All examples completed!");
  println!("\n💡 Configuration Tips:");
  println!("  RouteScale:");
  println!("    - scale: 0.2 (small), 0.4 (medium), 0.6 (large)");
  println!("    - offset: 0.0-1.0 (percentage of width/height)");
  println!("\n  RouteColor (BGRA format):");
  println!("    - Red: [0.0, 0.0, 255.0, 0.0]");
  println!("    - Green: [0.0, 255.0, 0.0, 0.0]");
  println!("    - Blue: [255.0, 0.0, 0.0, 0.0]");
  println!("    - Yellow: [0.0, 255.0, 255.0, 0.0]");
  println!("\n  LapDataConfig:");
  println!("    - position: (x%, y%) as 0.0-1.0 percentages");
  println!(
    "    - text_color: Color::White, Color::Red, Color::YellowGreen, etc."
  );
  println!(
    "    - font_scale, thickness, bar_max_width are FIXED (0.5, 1, 200)"
  );
  println!("\n  RouteVideoConfig:");
  println!("    - show_bottom_bar: true/false (pace/distance overlay)");
  println!("    - show_route: true/false (progressive route line)");
  println!("    - show_lap_data: true/false (lap statistics panel)");
  println!("\n  Available Colors:");
  println!("    Black, White, Red, Orange, Yellow, YellowGreen,");
  println!("    Green, BlueGreen, Blue, BlueViolet, Violet,");
  println!("    RedViolet, RedOrange, YellowOrange");

  Ok(())
}
