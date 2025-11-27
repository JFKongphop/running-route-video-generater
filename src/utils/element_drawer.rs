use anyhow::Result;
use opencv::{core, imgproc, prelude::*};

enum Align {
  Left,
  Right,
}

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
    // ----- Draw background bar -----
    let mut baseline = 0;
    let text_size = imgproc::get_text_size(
      dist,
      self.font,
      self.font_scale,
      self.thickness,
      &mut baseline,
    )?;
    let bar_height = text_size.height + 30;

    let rect = core::Rect::new(
      0,
      self.height - bar_height,
      self.width,
      bar_height,
    );

    let black = core::Scalar::new(0.0, 0.0, 0.0, 0.0);
    self.rectangle(frame, rect, black)?;

    let white = core::Scalar::new(255.0, 255.0, 255.0, 0.0);
    let y_text = self.height - self.margin;
    let items = vec![(pace, Align::Left), (dist, Align::Right)];
    for (text, align) in items {
      let x = match align {
        Align::Left => self.margin,
        Align::Right => {
          let mut base = 0;
          let size = imgproc::get_text_size(
            text,
            self.font,
            self.font_scale,
            self.thickness,
            &mut base,
          )?;
          self.width - size.width - self.margin
        }
      };

      self.text(
        frame,
        text,
        x,
        y_text,
        self.font_scale,
        white,
      )?;
    }

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

  fn rectangle(
    &self,
    frame: &mut Mat,
    rect: core::Rect,
    color: core::Scalar,
  ) -> Result<()> {
    imgproc::rectangle(
      frame,
      rect,
      color,
      -1,
      imgproc::LINE_AA,
      0,
    )?;
    Ok(())
  }
}
