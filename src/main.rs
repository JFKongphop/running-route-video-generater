use anyhow::Result;
use running_route_video_generater::generators::running_route_image::generate_running_route_image;
use running_route_video_generater::generators::route_video::generate_progressive_route;
use running_route_video_generater::utils::performance::measure;

fn main() -> Result<()> {
  let route_scale = 0.2;
  let offset_x_percent = 0.1;
  let offset_y_percent = 0.1;

  // measure("Total execution", || {
  //   generate_running_route_image(
  //     route_scale,
  //     offset_x_percent,
  //     offset_y_percent,
  //   )
  // })?;

  measure("Total execution", || {
    generate_progressive_route(
      route_scale,
      offset_x_percent,
      offset_y_percent,
    )
  })?;

  Ok(())
}
