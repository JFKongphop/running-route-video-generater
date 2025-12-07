use super::config::{FileConfig, RouteColor, RouteScale};

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
    }
  }
}
