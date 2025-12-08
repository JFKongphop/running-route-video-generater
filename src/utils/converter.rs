use anyhow::Result;
use opencv::{core, imgcodecs, imgproc, prelude::*};

pub fn speed_to_pace(speed: f32) -> String {
  if speed <= 0.0 {
    return String::from("0:00");
  }
  let pace_seconds = 1000.0 / speed;
  let minutes = (pace_seconds / 60.0).floor() as u32;
  let seconds = (pace_seconds % 60.0).round() as u32;
  format!("{}:{:02}", minutes, seconds)
}

pub fn semicircles_to_degrees(semicircles: i32) -> f64 {
  (semicircles as f64) * (180.0 / (i32::MAX as f64 + 1.0))
}

pub fn get_bounds(points: &Vec<(f64, f64)>) -> ((f64, f64), (f64, f64)) {
  let (lat_min, lat_max) = points.iter().fold(
    (f64::INFINITY, f64::NEG_INFINITY),
    |(mn, mx), (lat, _)| (mn.min(*lat), mx.max(*lat)),
  );

  let (lon_min, lon_max) = points.iter().fold(
    (f64::INFINITY, f64::NEG_INFINITY),
    |(mn, mx), (_, lon)| (mn.min(*lon), mx.max(*lon)),
  );

  ((lat_min, lat_max), (lon_min, lon_max))
}

pub fn load_and_resize_image(
  path: &str,
  max_dim: i32,
) -> Result<(Mat, i32, i32)> {
  // Load the background image
  let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)?;
  let size = img.size()?;
  let (orig_w, orig_h) = (size.width as f64, size.height as f64);

  // Compute scale factor to fit within max_dim
  let max_side = orig_w.max(orig_h);
  let scale = (max_dim as f64 / max_side).min(1.0);

  let width = (orig_w * scale) as i32;
  let height = (orig_h * scale) as i32;

  // Resize using high-quality interpolation
  let mut resized = Mat::default();
  imgproc::resize(
    &img,
    &mut resized,
    core::Size::new(width, height),
    0.0,
    0.0,
    imgproc::INTER_LANCZOS4,
  )?;

  Ok((resized, width, height))
}

pub fn string_space(size: usize, index: usize, pace: &str) -> String {
  let max_digits = count_digits_iterative(size);
  let current_digits = count_digits_iterative(index);
  let padding = " ".repeat(max_digits - current_digits + 3); // +1 space separator

  format!("{}{}{}", index, padding, pace)
}

pub fn convert_pace_to_sec(pace: &str) -> f32 {
  let (min, sec) = pace.split_once(':').unwrap();

  let minutes: f32 = min.parse().unwrap();
  let seconds: f32 = sec.parse().unwrap();

  minutes * 60.0 + seconds
}

fn count_digits_iterative(mut num: usize) -> usize {
  if num == 0 {
    return 1; // 0 has one digit
  }
  let mut count = 0;
  while num > 0 {
    num /= 10; // For decimal digits
    count += 1;
  }
  count
}

pub fn pace_percentage(numer: f32, denum: f32) -> f32 {
  numer / denum
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_speed_to_pace_valid() {
    // Speed in m/s to pace in min/km
    // 3.33 m/s ≈ 5:00 min/km (12 km/h)
    assert_eq!(speed_to_pace(3.33), "5:00");

    // 2.77 m/s ≈ 6:00 min/km (10 km/h)
    assert_eq!(speed_to_pace(2.77), "6:01");

    // 5.0 m/s = 3:20 min/km (18 km/h)
    assert_eq!(speed_to_pace(5.0), "3:20");
  }

  #[test]
  fn test_speed_to_pace_zero_and_negative() {
    assert_eq!(speed_to_pace(0.0), "0:00");
    assert_eq!(speed_to_pace(-1.0), "0:00");
  }

  #[test]
  fn test_semicircles_to_degrees() {
    // 0 semicircles = 0 degrees
    assert_eq!(semicircles_to_degrees(0), 0.0);

    // i32::MAX semicircles ≈ 180 degrees
    let max_degrees = semicircles_to_degrees(i32::MAX);
    assert!(max_degrees > 179.9 && max_degrees < 180.0);

    // Half of i32::MAX ≈ 90 degrees
    let half_max = semicircles_to_degrees(i32::MAX / 2);
    assert!(half_max > 89.9 && half_max < 90.0);

    // Negative values
    let neg_degrees = semicircles_to_degrees(-i32::MAX);
    assert!(neg_degrees < -179.9 && neg_degrees > -180.0);
  }

  #[test]
  fn test_get_bounds() {
    let points = vec![(10.0, 20.0), (5.0, 25.0), (15.0, 15.0), (8.0, 30.0)];

    let ((lat_min, lat_max), (lon_min, lon_max)) = get_bounds(&points);

    assert_eq!(lat_min, 5.0);
    assert_eq!(lat_max, 15.0);
    assert_eq!(lon_min, 15.0);
    assert_eq!(lon_max, 30.0);
  }

  #[test]
  fn test_get_bounds_single_point() {
    let points = vec![(10.0, 20.0)];
    let ((lat_min, lat_max), (lon_min, lon_max)) = get_bounds(&points);

    assert_eq!(lat_min, 10.0);
    assert_eq!(lat_max, 10.0);
    assert_eq!(lon_min, 20.0);
    assert_eq!(lon_max, 20.0);
  }

  #[test]
  fn test_string_space() {
    // Testing with size=100 (3 digits), various indices
    // max_digits=3, current_digits=1 -> padding = 3-1+3 = 5 spaces
    assert_eq!(
      string_space(100, 1, "5:00"),
      "1     5:00"
    );
    // max_digits=3, current_digits=2 -> padding = 3-2+3 = 4 spaces
    assert_eq!(
      string_space(100, 10, "5:30"),
      "10    5:30"
    );
    // max_digits=3, current_digits=3 -> padding = 3-3+3 = 3 spaces
    assert_eq!(
      string_space(100, 100, "6:00"),
      "100   6:00"
    );

    // Testing with size=10 (2 digits)
    // max_digits=2, current_digits=1 -> padding = 2-1+3 = 4 spaces
    assert_eq!(string_space(10, 1, "4:30"), "1    4:30");
    // max_digits=2, current_digits=1 -> padding = 2-1+3 = 4 spaces
    assert_eq!(string_space(10, 5, "5:00"), "5    5:00");
  }

  #[test]
  fn test_convert_pace_to_sec() {
    assert_eq!(convert_pace_to_sec("5:00"), 300.0);
    assert_eq!(convert_pace_to_sec("6:30"), 390.0);
    assert_eq!(convert_pace_to_sec("3:20"), 200.0);
    assert_eq!(convert_pace_to_sec("0:45"), 45.0);
  }

  #[test]
  fn test_count_digits_iterative() {
    assert_eq!(count_digits_iterative(0), 1);
    assert_eq!(count_digits_iterative(1), 1);
    assert_eq!(count_digits_iterative(9), 1);
    assert_eq!(count_digits_iterative(10), 2);
    assert_eq!(count_digits_iterative(99), 2);
    assert_eq!(count_digits_iterative(100), 3);
    assert_eq!(count_digits_iterative(999), 3);
    assert_eq!(count_digits_iterative(1000), 4);
  }

  #[test]
  fn test_pace_percentage() {
    assert_eq!(pace_percentage(50.0, 100.0), 0.5);
    assert_eq!(pace_percentage(25.0, 100.0), 0.25);
    assert_eq!(pace_percentage(100.0, 100.0), 1.0);
    assert_eq!(pace_percentage(75.0, 50.0), 1.5);
  }

  #[test]
  fn test_pace_percentage_zero() {
    assert_eq!(pace_percentage(0.0, 100.0), 0.0);
    // Division by zero - will return infinity
    assert!(pace_percentage(100.0, 0.0).is_infinite());
  }
}
