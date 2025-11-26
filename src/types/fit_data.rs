#[derive(Debug)]
pub struct RouteData {
  pub paces: Vec<String>,
  pub gps_points: Vec<(f64, f64)>,
  pub distances: Vec<f64>,
}

#[derive(Debug)]
pub struct LapData {
  pub avg_heart_rate: Vec<u8>,
  pub enhanced_avg_speed: Vec<String>,
  pub avg_step_length: Vec<f64>,
}
