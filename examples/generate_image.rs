/// Example: Generate a static route image
///
/// This example shows how to create a static image of your complete route
/// overlaid on a map.
///
/// Required files:
/// - source/car.fit (your GPS data)
/// - source/map.png (background map image)
///
/// Output: outputs/running_route.png
use anyhow::Result;
use runarium::generators::route_image::generate_running_route_image;

fn main() -> Result<()> {
  println!("🗺️  Generating route image...\n");

  // Configuration
  let route_scale = 0.2; // Route size (20% of map)
  let offset_x_percent = 0.1; // 10% from left
  let offset_y_percent = 0.1; // 10% from top

  // Generate image
  generate_running_route_image(
    route_scale,
    offset_x_percent,
    offset_y_percent,
  )?;

  println!("\n✅ Image generated successfully!");
  println!("📁 Location: outputs/running_route.png");

  Ok(())
}
