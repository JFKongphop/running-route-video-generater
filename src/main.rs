use running_route_video_generater::generators::running_route_video::generate_running_route_video;

fn main() {
  let route_scale = 0.2;
  let offset_x_percent = 0.1;
  let offset_y_percent = 0.1;

  generate_running_route_video(
    route_scale,
    offset_x_percent,
    offset_y_percent,
  );
}
