/// Example: Custom route scaling and positioning
///
/// This example demonstrates different scaling and positioning options
/// for your route visualization.
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route;

fn main() -> Result<()> {
  println!("🎯 Generating video with custom scaling...\n");

  // Option 1: Large route, centered
  println!("📍 Generating large centered route...");
  let route_scale = 0.5; // Larger route (50% of map)
  let offset_x_percent = 0.25; // Center horizontally
  let offset_y_percent = 0.25; // Center vertically

  generate_progressive_route(
    route_scale,
    offset_x_percent,
    offset_y_percent,
  )?;

  println!("\n✅ Video with custom scaling generated!");
  println!("📁 Location: outputs/car.mp4");

  println!("\n💡 Tips for scaling:");
  println!("  - route_scale: 0.1-0.3 = Small route");
  println!("  - route_scale: 0.3-0.5 = Medium route");
  println!("  - route_scale: 0.5-0.8 = Large route");
  println!("\n💡 Tips for positioning:");
  println!("  - offset 0.0 = Top/Left edge");
  println!("  - offset 0.25 = Centered");
  println!("  - offset 0.5 = Bottom/Right edge");

  Ok(())
}
