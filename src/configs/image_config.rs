use super::config::{FileConfig, RouteColor, RouteScale};
use super::video_config::LapDataConfig;

/// Configuration for route image generation
#[derive(Debug, Clone)]
pub struct RouteImageConfig {
  /// Route scale and positioning
  pub route_scale: RouteScale,
  /// Route colors
  pub colors: RouteColor,
  /// File paths configuration
  pub file_config: FileConfig,
  /// Line thickness for route
  pub line_thickness: i32,
  /// Lap data configuration (optional)
  pub lap_data: Option<LapDataConfig>,
  /// Whether to show lap data panel
  pub show_lap_data: bool,
}

impl RouteImageConfig {
  /// Creates a new RouteImageConfig
  pub fn new(
    route_scale: RouteScale,
    colors: RouteColor,
    file_config: FileConfig,
    line_thickness: i32,
  ) -> Self {
    Self {
      route_scale,
      colors,
      file_config,
      line_thickness,
      lap_data: None,
      show_lap_data: false,
    }
  }

  /// Creates a new RouteImageConfig with lap data
  pub fn with_lap_data(
    route_scale: RouteScale,
    colors: RouteColor,
    file_config: FileConfig,
    line_thickness: i32,
    lap_data: LapDataConfig,
  ) -> Self {
    Self {
      route_scale,
      colors,
      file_config,
      line_thickness,
      lap_data: Some(lap_data),
      show_lap_data: true,
    }
  }

  /// Creates default configuration
  pub fn default(
    fit_file: String,
    background_image: String,
    output_file: String,
  ) -> Self {
    Self {
      route_scale: RouteScale::default(),
      colors: RouteColor::default(),
      file_config: FileConfig::new(fit_file, background_image, output_file),
      line_thickness: 2,
      lap_data: None,
      show_lap_data: false,
    }
  }
}
