use anyhow::Result;
use opencv::{core, prelude::*, videoio};

pub fn image_creator() {}

pub fn video_creator(
  width: i32,
  height: i32,
  fps: f64,
  output_file: &str,
) -> Result<videoio::VideoWriter> {
  let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
  let video_size = core::Size::new(width, height);
  let video = videoio::VideoWriter::new(
    output_file,
    fourcc,
    fps,
    video_size,
    true,
  )?;

  Ok(video)
}
