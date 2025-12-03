/// Example: Generate an animated route video
/// 
/// This example shows how to create an animated video from a FIT file
/// with progressive route drawing and real-time statistics.
///
/// Required files:
/// - source/car.fit (your GPS data)
/// - source/map.png (background map image)
///
/// Output: outputs/car.mp4

use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route;
use runarium::utils::performance::measure;

fn main() -> Result<()> {
    println!("🎬 Generating route animation video...\n");

    // Configuration
    let route_scale = 0.2;        // Route size (20% of map)
    let offset_x_percent = 0.1;   // 10% from left
    let offset_y_percent = 0.1;   // 10% from top

    // Generate video with performance measurement
    measure("Total execution", || {
        generate_progressive_route(
            route_scale,
            offset_x_percent,
            offset_y_percent,
        )
    })?;

    println!("\n✅ Video generated successfully!");
    println!("📁 Location: outputs/car.mp4");

    Ok(())
}
