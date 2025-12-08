use std::fs::File;

use anyhow::Result;
use fitparser::{profile::MesgNum, Value};

use crate::{
  types::fit_data::{LapData, RouteData},
  utils::converter::{semicircles_to_degrees, speed_to_pace},
};

pub fn fit_reader(file_path: &str) -> Result<(RouteData, LapData)> {
  let mut paces = Vec::new();
  let mut gps_points = Vec::new();
  let mut distances = Vec::new();

  let mut avg_heart_rate = Vec::new();
  let mut enhanced_avg_speed = Vec::new();
  let mut avg_step_length = Vec::new();

  let mut fp = File::open(file_path)?;

  for data in fitparser::from_reader(&mut fp)? {
    match data.kind() {
      MesgNum::Lap => {
        let mut hr = None;
        let mut speed = None;
        let mut length = None;

        for field in data.fields() {
          match (field.name(), field.value()) {
            ("avg_heart_rate", Value::UInt8(v)) => hr = Some(*v),
            ("enhanced_avg_speed", Value::Float64(v)) => {
              speed = Some(speed_to_pace(*v as f32))
            }
            ("avg_step_length", Value::Float64(v)) => length = Some(*v),
            _ => {}
          }
        }

        if let (Some(hr), Some(speed), Some(length)) = (hr, speed, length) {
          avg_heart_rate.push(hr);
          enhanced_avg_speed.push(speed);
          avg_step_length.push(length);
        }
      }

      MesgNum::Record => {
        let mut lat = None;
        let mut lon = None;
        let mut pace = None;

        for field in data.fields() {
          match (field.name(), field.value()) {
            ("enhanced_speed", Value::Float32(v)) => {
              pace = Some(speed_to_pace(*v))
            }
            ("enhanced_speed", Value::Float64(v)) => {
              pace = Some(speed_to_pace(*v as f32))
            }
            ("position_lat", Value::SInt32(v)) => {
              lat = Some(semicircles_to_degrees(*v))
            }
            ("position_long", Value::SInt32(v)) => {
              lon = Some(semicircles_to_degrees(*v))
            }
            ("distance", Value::Float64(v)) => distances.push(*v),
            _ => {}
          }
        }

        if let (Some(lat), Some(lon), Some(pace)) = (lat, lon, pace) {
          gps_points.push((lat, lon));
          paces.push(pace);
        }
      }

      _ => {}
    }
  }

  Ok((
    RouteData {
      paces,
      gps_points,
      distances,
    },
    LapData {
      avg_heart_rate,
      enhanced_avg_speed,
      avg_step_length,
    },
  ))
}
