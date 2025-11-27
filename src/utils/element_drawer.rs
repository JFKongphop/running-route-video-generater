use anyhow::Result;
use opencv::{core, imgproc, prelude::*};

pub struct RouteDrawer {
  pub width: i32,
  pub height: i32,
  pub font: i32,
  pub font_scale: f64,
  pub thickness: i32,
  pub margin: i32,
}

impl RouteDrawer {
  pub fn new(width: i32, height: i32) -> Self {
    Self {
      width,
      height,
      font: imgproc::FONT_HERSHEY_SIMPLEX,
      font_scale: 0.8,
      thickness: 2,
      margin: 20,
    }
  }

  pub fn line(
    &self,
    frame: &mut Mat,
    p1: core::Point,
    p2: core::Point,
    color: core::Scalar,
  ) -> Result<()> {
    imgproc::line(
      frame,
      p1,
      p2,
      color,
      4,
      imgproc::LINE_AA,
      0,
    )?;
    Ok(())
  }

  pub fn point(
    &self,
    frame: &mut Mat,
    point: core::Point,
    color: core::Scalar,
  ) -> Result<()> {
    imgproc::circle(
      frame,
      point,
      8,
      color,
      -1,
      imgproc::LINE_AA,
      0,
    )?;
    Ok(())
  }

  pub fn text_bar(
    &self,
    frame: &mut Mat,
    pace: &str,
    dist: &str,
  ) -> Result<()> {
    let mut baseline = 0;
    let text_size = imgproc::get_text_size(
      dist,
      self.font,
      self.font_scale,
      self.thickness,
      &mut baseline,
    )?;
    let bar_height = text_size.height + 30;

    let bg_top_left = core::Point::new(0, self.height - bar_height);
    let bg_bottom_right = core::Point::new(self.width, self.height);
    let rect = core::Rect::new(
      bg_top_left.x,
      bg_top_left.y,
      bg_bottom_right.x - bg_top_left.x,
      bg_bottom_right.y - bg_top_left.y,
    );

    // Draw semi-transparent black bar
    imgproc::rectangle(
      frame,
      rect,
      core::Scalar::new(0.0, 0.0, 0.0, 0.0),
      -1,
      imgproc::LINE_8,
      0,
    )?;

    // Draw left text (pace)
    let y_text = self.height - self.margin;
    imgproc::put_text(
      frame,
      pace,
      core::Point::new(self.margin, y_text),
      self.font,
      self.font_scale,
      core::Scalar::new(255.0, 255.0, 255.0, 0.0),
      self.thickness,
      imgproc::LINE_AA,
      false,
    )?;

    // Draw right text (distance)
    let text_size = imgproc::get_text_size(
      dist,
      self.font,
      self.font_scale,
      self.thickness,
      &mut baseline,
    )?;
    let x_right = self.width - text_size.width - self.margin;
    imgproc::put_text(
      frame,
      dist,
      core::Point::new(x_right, y_text),
      self.font,
      self.font_scale,
      core::Scalar::new(255.0, 255.0, 255.0, 0.0),
      self.thickness,
      imgproc::LINE_AA,
      false,
    )?;

    Ok(())
  }

  pub fn header(&self, frame: &mut Mat, x: i32, y: i32) -> Result<()> {
    let bluish_color = core::Scalar::new(255.0, 255.0, 0.0, 0.0);
    const LABELS: [(&str, i32); 4] = [
      ("KM  PACE", -20),
      ("BAR", 150),
      ("HR", 285),
      ("LENGTH", 320),
    ];

    let font_scale = 0.5;
    let y_start = y - 20;

    for (label, offset) in LABELS {
      self.text(
        frame,
        label,
        x + offset,
        y_start,
        font_scale,
        bluish_color,
      )?;
    }

    Ok(())
  }

  fn text(
    &self,
    frame: &mut Mat,
    text: &str,
    x: i32,
    y: i32,
    font_scale: f64,
    color: core::Scalar,
  ) -> Result<()> {
    imgproc::put_text(
      frame,
      text,
      core::Point::new(x, y),
      self.font,
      font_scale,
      color,
      self.thickness,
      imgproc::LINE_AA,
      false,
    )?;
    Ok(())
  }
}
