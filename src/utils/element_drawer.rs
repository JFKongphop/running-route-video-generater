use crate::types::{
  drawer_data::{PositionRect, Rect, SizeRect},
  route_config::Font,
};
use anyhow::Result;
use opencv::{core, imgproc, prelude::*};

enum Align {
  Left,
  Right,
}

pub struct Drawer {
  pub width: i32,
  pub height: i32,
  pub line: i32,
}

impl Drawer {
  pub fn new(width: i32, height: i32) -> Self {
    Self {
      width,
      height,
      line: imgproc::LINE_AA,
    }
  }

  pub fn line(
    &self,
    frame: &mut Mat,
    p1: core::Point,
    p2: core::Point,
    color: core::Scalar,
  ) -> Result<()> {
    imgproc::line(frame, p1, p2, color, 4, self.line, 0)?;
    Ok(())
  }

  pub fn point(
    &self,
    frame: &mut Mat,
    point: core::Point,
    color: core::Scalar,
  ) -> Result<()> {
    imgproc::circle(frame, point, 8, color, -1, self.line, 0)?;
    Ok(())
  }

  pub fn text_bar(
    &self,
    frame: &mut Mat,
    pace: &str,
    dist: &str,
    font_scale: f64,
    thickness: i32,
    font: Font,
  ) -> Result<()> {
    // ----- draw background bar -----
    let text_size = self.text_size(dist, font_scale, thickness, font)?;
    let bar_height = text_size.height + 30;
    let rect = Rect {
      pos: PositionRect {
        x: 0,
        y: self.height - bar_height,
      },
      size: SizeRect {
        width: self.width,
        height: bar_height,
      },
    };
    let black_color = self.color([0.0; 4]);
    self.rectangle(frame, rect, black_color)?;

    // ----- draw pace and distance -----
    let white_color = self.color([255.0, 255.0, 255.0, 0.0]);
    let margin = 20;
    let y_text = self.height - margin;
    let items = vec![(pace, Align::Left), (dist, Align::Right)];
    for (text, align) in items {
      let x = match align {
        Align::Left => margin,
        Align::Right => {
          let size = self.text_size(text, font_scale, thickness, font)?;
          self.width - size.width - margin
        }
      };

      self.text(
        frame,
        text,
        x,
        y_text,
        font_scale,
        thickness,
        font,
        white_color,
      )?;
    }

    Ok(())
  }

  pub fn header(&self, frame: &mut Mat, x: i32, y: i32, font_scale: f64, thickness: i32, font: Font) -> Result<()> {
    let bluish_color = self.color([255.0, 255.0, 0.0, 0.0]);
    const LABELS: [(&str, i32); 4] = [
      ("KM   PACE", -20),
      ("BAR", 150),
      ("HR", 285),
      ("LENGTH", 320),
    ];

    let y_start = y - 20;

    for (label, offset) in LABELS {
      self.text(
        frame,
        label,
        x + offset,
        y_start,
        font_scale,
        thickness,
        font,
        bluish_color,
      )?;
    }

    Ok(())
  }

  pub fn text(
    &self,
    frame: &mut Mat,
    text: &str,
    x: i32,
    y: i32,
    font_scale: f64,
    thickness: i32,
    font: Font,
    color: core::Scalar,
  ) -> Result<()> {
    imgproc::put_text(
      frame,
      text,
      core::Point::new(x, y),
      font.to_opencv(),
      font_scale,
      color,
      thickness,
      self.line,
      false,
    )?;
    Ok(())
  }

  pub fn rectangle(
    &self,
    frame: &mut Mat,
    rect: Rect,
    color: core::Scalar,
  ) -> Result<()> {
    let Rect { pos, size } = rect;
    let PositionRect { x, y } = pos;
    let SizeRect { width, height } = size;
    let rect = core::Rect::new(x, y, width, height);

    imgproc::rectangle(frame, rect, color, -1, self.line, 0)?;
    Ok(())
  }

  pub fn text_size(
    &self,
    text: &str,
    font_scale: f64,
    thickness: i32,
    font: Font,
  ) -> Result<core::Size> {
    let mut baseline = 0;
    let size = imgproc::get_text_size(
      text,
      font.to_opencv(),
      font_scale,
      thickness,
      &mut baseline,
    )?;
    Ok(size)
  }

  pub fn color(&self, bgra: [f64; 4]) -> core::Scalar {
    core::Scalar::new(bgra[0], bgra[1], bgra[2], bgra[3])
  }
}
