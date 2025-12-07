use crate::{
  configs::RouteVideoConfig,
  types::{
    drawer_data::{PositionRect, Rect, SizeRect},
    fit_data::{LapData, RouteData},
  },
  utils::{
    converter::{convert_pace_to_sec, get_bounds, load_and_resize_image, pace_percentage, string_space},
    creator::video_creator,
    element_drawer::Drawer,
    performance::processed,
    read_file::fit_reader,
  },
};
use anyhow::Result;
use opencv::{core, imgproc, prelude::*};

/// Generates an animated video of a running route with lap statistics overlay.
///
/// Creates a video showing progressive route drawing on a background image,
/// with real-time pace/distance info and a lap statistics panel.
///
/// # Arguments
/// * `route_scale` - Scale factor for route visualization (0.0-1.0 recommended)
/// * `offset_x_percent` - Horizontal offset as percentage of image width
/// * `offset_y_percent` - Vertical offset as percentage of image height
///
/// # Returns
/// * `Ok(())` - Video successfully created and saved
/// * `Err` - If FIT file reading, video encoding, or drawing operations fail
///
/// # Output
/// Creates `outputs/car.mp4` with:
/// - Animated route drawing (red line)
/// - Current position marker (green dot)
/// - Lap statistics panel (pace, heart rate, stride length)
/// - Real-time pace and distance overlay
pub fn progressive_route(
  route_scale: f64,
  offset_x_percent: f64,
  offset_y_percent: f64,
) -> Result<()> {
  /********** Read and extract data **********/
  #[rustfmt::skip]
  let (route, lap) = fit_reader("source/car.fit")?;
  let RouteData {
    paces,
    gps_points: points,
    distances,
  } = route;
  let LapData {
    avg_heart_rate,
    enhanced_avg_speed,
    avg_step_length,
  } = lap;

  /********** Normalize coordinates **********/
  #[rustfmt::skip]
  let (
    (lat_min, lat_max),
    (lon_min, lon_max)
  ) = get_bounds(&points);

  /********** Get backgrund image **********/
  let (bg_image, width, height) =
    load_and_resize_image("source/car.jpg", 1080)?;

  /********** Coordinate normalization to image space **********/
  let to_px = |lat: f64, lon: f64| -> core::Point {
    let nx = if lon_max != lon_min {
      (lon - lon_min) / (lon_max - lon_min)
    } else {
      0.5
    };
    let ny = if lat_max != lat_min {
      (lat - lat_min) / (lat_max - lat_min)
    } else {
      0.5
    };

    let x = ((offset_x_percent + nx * route_scale) * width as f64) as i32;
    #[rustfmt::skip]
    let y = ((offset_y_percent + (1.0 - ny) * route_scale) * width as f64) as i32;
    core::Point::new(x, y)
  };

  /********** Initialized video generator **********/
  #[rustfmt::skip]
  let pixel_points: Vec<core::Point> = points
    .iter()
    .map(|&(la, lo)| to_px(la, lo))
    .collect();
  let fps = (pixel_points.len() / 15) as f64;
  let output_file = "outputs/simple.mp4";
  let mut video = video_creator(width, height, fps, output_file)?;

  /********** Initialized frame **********/
  // let mut path_frame = Mat::zeros(height, width, core::CV_8UC3)?.to_mat()?;
  let mut resized = Mat::default();
  imgproc::resize(
    &bg_image,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  let mut path_frame = resized.clone();
  let drawer = Drawer::new(width, height);

  let font = crate::configs::video_config::Font::Simplex;
  let font_scale = 0.5;
  let thickness = 1;

  let pace_seconds: Vec<f32> = enhanced_avg_speed
    .iter()
    .map(|p| convert_pace_to_sec(p))
    .collect();

  let start_x = width / 2;
  let start_y = 100;
  let min_val = *pace_seconds
    .iter()
    .min_by(|a, b| a.total_cmp(b))
    .expect("Failed to find min pace");
  let min_denominator = (min_val / 30.0).floor() * 30.0;
  let bar_max_width = 200;

  /********** Create lap data **********/
  drawer
    .header(&mut path_frame, start_x, start_y, font_scale, thickness, font)
    .expect("Failed to draw header!");

  let green_color = drawer.color([0.0, 255.0, 0.0, 0.0]);
  let white_color = drawer.color([255.0, 255.0, 255.0, 0.0]);
  let size_of_speeds = enhanced_avg_speed.len();
  for (i, pace) in enhanced_avg_speed.iter().enumerate() {
    let size = drawer.text_size(pace, font_scale, thickness, font)?;
    let x = start_x - size.width / 2;
    let y = start_y + i as i32 * (size.height + 5);

    let pace_space = string_space(size_of_speeds, i + 1, pace);
    drawer
      .text(
        &mut path_frame,
        &pace_space,
        x,
        y,
        font_scale,
        thickness,
        font,
        white_color,
      )
      .expect("Failed to draw pace");

    let hr = &format!("{}", avg_heart_rate[i]);
    drawer
      .text(
        &mut path_frame,
        hr,
        x + 300,
        y,
        font_scale,
        thickness,
        font,
        white_color,
      )
      .expect("Failed to draw heart rate");

    let lenght_meters = avg_step_length[i] / 10.0;
    let stride_length = &format!("{}", lenght_meters);
    drawer
      .text(
        &mut path_frame,
        stride_length,
        x + 350,
        y,
        font_scale,
        thickness,
        font,
        white_color,
      )
      .expect("Failed to draw stride length");

    let percent = pace_percentage(min_denominator, pace_seconds[i]);
    let bar_width = (percent * bar_max_width as f32) as i32;
    let bar_height = size.height;
    let bar_x = x + size.width + 60;
    let bar_y = y - size.height;
    let rect = Rect {
      pos: PositionRect { x: bar_x, y: bar_y },
      size: SizeRect {
        width: bar_width,
        height: bar_height,
      },
    };
    drawer
      .rectangle(&mut path_frame, rect, green_color)
      .expect("Failed to draw bar");
  }

  /********** Create progressive route **********/
  let red_color = drawer.color([0.0, 0.0, 255.0, 0.0]);
  for (i, point) in pixel_points.iter().enumerate() {
    if i > 0 {
      drawer.line(
        &mut path_frame,
        pixel_points[i - 1],
        *point,
        red_color,
      )?;
    }

    let mut current_frame = path_frame.clone();
    drawer.point(&mut current_frame, *point, green_color)?;

    if i < paces.len() && i < distances.len() {
      let pace_text = format!("Pace: {} min/km", paces[i]);
      let dist_text = format!("Dist: {:.2} km", distances[i] / 1000.0);
      let font_scale = 0.8;
      let thickness = 2;

      drawer.text_bar(
        &mut current_frame,
        &pace_text,
        &dist_text,
        font_scale,
        thickness,
        font,
      )?;
    }

    video.write(&current_frame)?;
    processed(i, pixel_points.clone());
  }

  video.release()?;
  println!(
    "✅ Video created: {} with {} points",
    output_file,
    pixel_points.len()
  );
  Ok(())
}

/// Generates an animated video with custom configuration.
///
/// This function provides full control over all aspects of the video generation
/// including colors, positioning, and what data to display.
///
/// # Arguments
/// * `config` - RouteVideoConfig containing all customization options
///
/// # Returns
/// * `Ok(())` - Video successfully created and saved
/// * `Err` - If FIT file reading, video encoding, or drawing operations fail
///
/// # Example
/// ```no_run
/// use runarium::types::route_config::RouteVideoConfig;
/// use runarium::generators::route_video::progressive_route_with_config;
///
/// let config = RouteVideoConfig::default(1080);
/// progressive_route_with_config(config)?;
/// ```
pub fn progressive_route_with_config(
  config: RouteVideoConfig,
) -> Result<()> {
  /********** Read and extract data **********/
  #[rustfmt::skip]
  let (route, lap) = fit_reader(&config.file_config.fit_file)?;
  let RouteData {
    paces,
    gps_points: points,
    distances,
  } = route;
  let LapData {
    avg_heart_rate,
    enhanced_avg_speed,
    avg_step_length,
  } = lap;

  /********** Normalize coordinates **********/
  #[rustfmt::skip]
  let (
    (lat_min, lat_max),
    (lon_min, lon_max)
  ) = get_bounds(&points);

  /********** Get background image **********/
  let (bg_image, width, height) =
    load_and_resize_image(&config.file_config.background_image, 1080)?;

  /********** Coordinate normalization to image space **********/
  let to_px = |lat: f64, lon: f64| -> core::Point {
    let nx = if lon_max != lon_min {
      (lon - lon_min) / (lon_max - lon_min)
    } else {
      0.5
    };
    let ny = if lat_max != lat_min {
      (lat - lat_min) / (lat_max - lat_min)
    } else {
      0.5
    };

    let x = ((config.route_scale.offset_x_percent
      + nx * config.route_scale.scale)
      * width as f64) as i32;
    #[rustfmt::skip]
    let y = ((config.route_scale.offset_y_percent + (1.0 - ny) * config.route_scale.scale) * width as f64) as i32;
    core::Point::new(x, y)
  };

  /********** Initialized video generator **********/
  #[rustfmt::skip]
  let pixel_points: Vec<core::Point> = points
    .iter()
    .map(|&(la, lo)| to_px(la, lo))
    .collect();
  let fps = (pixel_points.len() / 15) as f64;
  let mut video = video_creator(width, height, fps, &config.file_config.output_file)?;

  /********** Initialized frame **********/
  let mut resized = Mat::default();
  imgproc::resize(
    &bg_image,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  let mut path_frame = resized.clone();
  let drawer = Drawer::new(width, height);

  let pace_seconds: Vec<f32> = enhanced_avg_speed
    .iter()
    .map(|p| convert_pace_to_sec(p))
    .collect();

  let start_x = (config.lap_data.position.0 * width as f64) as i32;
  let start_y = (config.lap_data.position.1 * height as f64) as i32;
  let min_val = *pace_seconds
    .iter()
    .min_by(|a, b| a.total_cmp(b))
    .expect("Failed to find min pace");
  let min_denominator = (min_val / 30.0).floor() * 30.0;

  /********** Create lap data **********/
  if config.show_lap_data {
    drawer
      .header(&mut path_frame, start_x, start_y, config.lap_data.font_scale, config.lap_data.thickness, config.lap_data.font)
      .expect("Failed to draw header!");

    let text_color = drawer.color(config.lap_data.text_color.to_bgra());
    let bar_color = drawer.color(config.colors.lap_bars);
    let size_of_speeds = enhanced_avg_speed.len();

    for (i, pace) in enhanced_avg_speed.iter().enumerate() {
      let size = drawer.text_size(pace, config.lap_data.font_scale, config.lap_data.thickness, config.lap_data.font)?;
      let x = start_x - size.width / 2;
      let y = start_y + i as i32 * (size.height + 5);

      // Draw pace
      let pace_space = string_space(size_of_speeds, i + 1, pace);
      drawer
        .text(
          &mut path_frame,
          &pace_space,
          x,
          y,
          config.lap_data.font_scale,
          config.lap_data.thickness,
          config.lap_data.font,
          text_color,
        )
        .expect("Failed to draw pace");

      // Draw heart rate if enabled
      if config.lap_data.show_heart_rate {
        let hr = &format!("{}", avg_heart_rate[i]);
        drawer
          .text(
            &mut path_frame,
            hr,
            x + 300,
            y,
            config.lap_data.font_scale,
            config.lap_data.thickness,
            config.lap_data.font,
            text_color,
          )
          .expect("Failed to draw heart rate");
      }

      // Draw stride length if enabled
      if config.lap_data.show_stride_length {
        let length_meters = avg_step_length[i] / 10.0;
        let stride_length = &format!("{}", length_meters);
        drawer
          .text(
            &mut path_frame,
            stride_length,
            x + 350,
            y,
            config.lap_data.font_scale,
            config.lap_data.thickness,
            config.lap_data.font,
            text_color,
          )
          .expect("Failed to draw stride length");
      }

      // Draw pace bars if enabled
      if config.lap_data.show_pace_bars {
        let percent = pace_percentage(min_denominator, pace_seconds[i]);
        let bar_width = (percent * config.lap_data.bar_max_width as f32) as i32;
        let bar_height = size.height;
        let bar_x = x + size.width + 60;
        let bar_y = y - size.height;
        let rect = Rect {
          pos: PositionRect { x: bar_x, y: bar_y },
          size: SizeRect {
            width: bar_width,
            height: bar_height,
          },
        };
        drawer
          .rectangle(&mut path_frame, rect, bar_color)
          .expect("Failed to draw bar");
      }
    }
  }

  /********** Create progressive route **********/
  let route_color = drawer.color(config.colors.route_line);
  let position_color = drawer.color(config.colors.current_position);

  for (i, point) in pixel_points.iter().enumerate() {
    if config.show_route {
      if i > 0 {
        drawer.line(
          &mut path_frame,
          pixel_points[i - 1],
          *point,
          route_color,
        )?;
      }
    }

    let mut current_frame = path_frame.clone();
    if config.show_route {
      drawer.point(
        &mut current_frame,
        *point,
        position_color,
      )?;
    }

    // Draw pace and distance overlay if enabled
    if config.show_bottom_bar && i < paces.len() && i < distances.len() {
      if config.pace_dist.show_pace || config.pace_dist.show_distance {
        let pace_text = if config.pace_dist.show_pace {
          format!("Pace: {} min/km", paces[i])
        } else {
          String::new()
        };

        let dist_text = if config.pace_dist.show_distance {
          format!("Dist: {:.2} km", distances[i] / 1000.0)
        } else {
          String::new()
        };

        drawer.text_bar(
          &mut current_frame,
          &pace_text,
          &dist_text,
          config.pace_dist.font_scale,
          config.pace_dist.thickness,
          config.pace_dist.font,
        )?;
      }
    }

    video.write(&current_frame)?;
    processed(i, pixel_points.clone());
  }

  video.release()?;
  println!(
    "✅ Video created: {} with {} points",
    config.file_config.output_file,
    pixel_points.len()
  );
  Ok(())
}
