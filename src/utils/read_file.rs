use crate::utils::converter::{semicircles_to_degrees, speed_to_pace};
use anyhow::Result;
use fitparser::{Value, profile::MesgNum};
use std::fs::File;

pub fn fit_reader(
  file_path: &str,
) -> Result<(Vec<String>, Vec<(f64, f64)>, Vec<f64>)> {
  let mut paces = Vec::new();
  let mut gps_points = Vec::new();
  let mut distances = Vec::new();
  let mut fp = File::open(file_path)?;

  for data in fitparser::from_reader(&mut fp)? {
    if data.kind() == MesgNum::Record {
      let mut lat: Option<f64> = None;
      let mut lon: Option<f64> = None;
      let mut pace: Option<String> = None;

      for field in data.fields() {
        match field.name() {
          "enhanced_speed" => match field.value() {
            Value::Float32(v) => pace = Some(speed_to_pace(*v)),
            Value::Float64(v) => pace = Some(speed_to_pace(*v as f32)),
            _ => {}
          },
          "position_lat" => {
            if let Value::SInt32(v) = field.value() {
              lat = Some(semicircles_to_degrees(*v));
            }
          }
          "position_long" => {
            if let Value::SInt32(v) = field.value() {
              lon = Some(semicircles_to_degrees(*v));
            }
          }
          "distance" => {
            if let Value::Float64(v) = field.value() {
              distances.push(*v);
            }
          }
          _ => {}
        }
      }

      if let (Some(lat), Some(lon), Some(pace)) = (lat, lon, pace) {
        gps_points.push((lat, lon));
        paces.push(pace);
      }
    }
  }
  Ok((paces, gps_points, distances))
}
