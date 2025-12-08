// Re-export all config types for public API
pub use super::config::{Color, FileConfig, Font, RouteColor, RouteScale};

/// Configuration for pace and distance display
#[derive(Debug, Clone)]
pub struct PaceDistConfig {
  /// Font scale for pace/distance text
  pub font_scale: f64,
  /// Thickness of the text
  pub thickness: i32,
  /// Font family for text
  pub font: Font,
  /// Position for the pace/distance bar (None = auto)
  pub position: Option<(i32, i32)>,
  /// Whether to show pace
  pub show_pace: bool,
  /// Whether to show distance
  pub show_distance: bool,
}

impl PaceDistConfig {
  /// Creates a new PaceDistConfig with custom settings
  pub fn new(
    font_scale: f64,
    thickness: i32,
    font: Font,
    position: Option<(i32, i32)>,
    show_pace: bool,
    show_distance: bool,
  ) -> Self {
    Self {
      font_scale,
      thickness,
      font,
      position,
      show_pace,
      show_distance,
    }
  }

  /// Creates default configuration
  pub fn default() -> Self {
    Self {
      font_scale: 0.5,
      thickness: 1,
      font: Font::Simplex,
      position: None,
      show_pace: true,
      show_distance: true,
    }
  }

  /// Creates configuration for large text
  pub fn large_text() -> Self {
    Self {
      font_scale: 0.8,
      thickness: 2,
      font: Font::Duplex,
      position: None,
      show_pace: true,
      show_distance: true,
    }
  }

  /// Creates configuration with only pace
  pub fn pace_only() -> Self {
    Self {
      font_scale: 0.5,
      thickness: 1,
      font: Font::Simplex,
      position: None,
      show_pace: true,
      show_distance: false,
    }
  }
}

/// Complete configuration for route video generation
#[derive(Debug, Clone)]
pub struct LapDataConfig {
  /// Position of the lap panel as percentage (x_percent, y_percent) where 0.0-1.0
  pub position: (f64, f64),
  /// Font scale (fixed at 0.5)
  pub font_scale: f64,
  /// Text thickness (fixed at 1)
  pub thickness: i32,
  /// Font family for text
  pub font: Font,
  /// Text color for lap data
  pub text_color: Color,
  /// Whether to show heart rate
  pub show_heart_rate: bool,
  /// Whether to show stride length
  pub show_stride_length: bool,
  /// Whether to show pace bars
  pub show_pace_bars: bool,
}

impl LapDataConfig {
  /// Creates a new LapDataConfig with custom settings
  pub fn new(
    position: (f64, f64),
    font_scale: f64,
    thickness: i32,
    font: Font,
    text_color: Color,
    show_heart_rate: bool,
    show_stride_length: bool,
    show_pace_bars: bool,
  ) -> Self {
    Self {
      position,
      font_scale,
      thickness,
      font,
      text_color,
      show_heart_rate,
      show_stride_length,
      show_pace_bars,
    }
  }

  /// Creates default configuration
  pub fn default() -> Self {
    Self {
      position: (0.5, 0.09), // 50% x, 9% y
      font_scale: 0.5,
      thickness: 1,
      font: Font::Simplex,
      text_color: Color::White,
      show_heart_rate: true,
      show_stride_length: true,
      show_pace_bars: true,
    }
  }

  /// Creates minimal configuration (pace only, no extras)
  pub fn minimal() -> Self {
    Self {
      position: (0.5, 0.09), // 50% x, 9% y
      font_scale: 0.5,
      thickness: 1,
      font: Font::Simplex,
      text_color: Color::White,
      show_heart_rate: false,
      show_stride_length: false,
      show_pace_bars: true,
    }
  }

  /// Creates detailed configuration (all stats, larger bars)
  pub fn detailed() -> Self {
    Self {
      position: (0.5, 0.07), // 50% x, 7% y
      font_scale: 0.5,
      thickness: 1,
      font: Font::Simplex,
      text_color: Color::White,
      show_heart_rate: true,
      show_stride_length: true,
      show_pace_bars: true,
    }
  }
}

/// Complete configuration for route video generation
#[derive(Debug, Clone)]
pub struct RouteVideoConfig {
  /// Route scaling and positioning
  pub route_scale: RouteScale,
  /// Color scheme
  pub colors: RouteColor,
  /// Pace and distance display settings
  pub pace_dist: PaceDistConfig,
  /// Lap statistics settings
  pub lap_data: LapDataConfig,
  /// File paths configuration
  pub file_config: FileConfig,
  /// Whether to show the bottom pace/distance bar
  pub show_bottom_bar: bool,
  /// Whether to show the progressive route animation
  pub show_route: bool,
  /// Whether to show the lap data panel
  pub show_lap_data: bool,
}

impl RouteVideoConfig {
  /// Creates a new RouteVideoConfig with all custom settings
  pub fn new(
    route_scale: RouteScale,
    colors: RouteColor,
    pace_dist: PaceDistConfig,
    lap_data: LapDataConfig,
    file_config: FileConfig,
    show_bottom_bar: bool,
    show_route: bool,
    show_lap_data: bool,
  ) -> Self {
    Self {
      route_scale,
      colors,
      pace_dist,
      lap_data,
      file_config,
      show_bottom_bar,
      show_route,
      show_lap_data,
    }
  }

  /// Creates default configuration
  pub fn default() -> Self {
    Self {
      route_scale: RouteScale::default(),
      colors: RouteColor::default(),
      pace_dist: PaceDistConfig::default(),
      lap_data: LapDataConfig::default(),
      show_bottom_bar: true,
      show_route: true,
      show_lap_data: true,
      file_config: FileConfig::default(),
    }
  }

  /// Creates a minimalist configuration
  pub fn minimalist() -> Self {
    Self {
      route_scale: RouteScale::default(),
      colors: RouteColor::default(),
      pace_dist: PaceDistConfig::pace_only(),
      lap_data: LapDataConfig::minimal(),
      show_bottom_bar: true,
      show_route: true,
      show_lap_data: true,
      file_config: FileConfig::default(),
    }
  }

  /// Creates a detailed configuration with all features
  pub fn detailed() -> Self {
    Self {
      route_scale: RouteScale::large(),
      colors: RouteColor::default(),
      pace_dist: PaceDistConfig::large_text(),
      lap_data: LapDataConfig::detailed(),
      show_bottom_bar: true,
      show_route: true,
      show_lap_data: true,
      file_config: FileConfig::default(),
    }
  }

  /// Creates a vibrant neon-themed configuration
  pub fn neon() -> Self {
    Self {
      route_scale: RouteScale::centered(),
      colors: RouteColor::neon_scheme(),
      pace_dist: PaceDistConfig::default(),
      lap_data: LapDataConfig::default(),
      show_bottom_bar: true,
      show_route: true,
      show_lap_data: true,
      file_config: FileConfig::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_color_to_bgra() {
    assert_eq!(
      Color::Red.to_bgra(),
      [0.0, 0.0, 255.0, 0.0]
    );
    assert_eq!(
      Color::Green.to_bgra(),
      [0.0, 255.0, 0.0, 0.0]
    );
    assert_eq!(
      Color::Blue.to_bgra(),
      [255.0, 0.0, 0.0, 0.0]
    );
    assert_eq!(
      Color::White.to_bgra(),
      [255.0, 255.0, 255.0, 0.0]
    );
    assert_eq!(
      Color::Black.to_bgra(),
      [0.0, 0.0, 0.0, 0.0]
    );
  }

  #[test]
  fn test_route_scale_presets() {
    let default = RouteScale::default();
    assert_eq!(default.scale, 0.2);
    assert_eq!(default.offset_x_percent, 0.1);
    assert_eq!(default.offset_y_percent, 0.1);

    let centered = RouteScale::centered();
    assert_eq!(centered.scale, 0.4);
    assert_eq!(centered.offset_x_percent, 0.3);
    assert_eq!(centered.offset_y_percent, 0.3);

    let large = RouteScale::large();
    assert_eq!(large.scale, 0.7);
    assert_eq!(large.offset_x_percent, 0.15);
    assert_eq!(large.offset_y_percent, 0.15);
  }

  #[test]
  fn test_route_scale_custom() {
    let custom = RouteScale::new(0.5, 0.2, 0.3);
    assert_eq!(custom.scale, 0.5);
    assert_eq!(custom.offset_x_percent, 0.2);
    assert_eq!(custom.offset_y_percent, 0.3);
  }

  #[test]
  fn test_lap_data_position_percentages() {
    let config = LapDataConfig::default();

    // Positions should be between 0.0 and 1.0
    assert!(config.position.0 >= 0.0 && config.position.0 <= 1.0);
    assert!(config.position.1 >= 0.0 && config.position.1 <= 1.0);

    // Calculate pixel positions for 1920x1080
    let x = config.position.0 * 1920.0;
    let y = config.position.1 * 1080.0;

    assert!(x >= 0.0 && x <= 1920.0);
    assert!(y >= 0.0 && y <= 1080.0);
  }

  #[test]
  fn test_pace_dist_config() {
    let default = PaceDistConfig::default();
    assert!(default.show_pace);
    assert!(default.show_distance);

    let pace_only = PaceDistConfig::pace_only();
    assert!(pace_only.show_pace);
    assert!(!pace_only.show_distance);
  }

  #[test]
  fn test_route_video_config_presets() {
    let default = RouteVideoConfig::default();
    assert_eq!(default.show_bottom_bar, true);
    assert_eq!(default.show_route, true);
    assert_eq!(default.show_lap_data, true);
    assert_eq!(
      default.file_config.fit_file,
      "source/car.fit"
    );
    assert_eq!(
      default.file_config.background_image,
      "source/car.jpg"
    );
    assert_eq!(
      default.file_config.output_file,
      "outputs/car.mp4"
    );

    let minimalist = RouteVideoConfig::minimalist();
    assert!(minimalist.show_bottom_bar);

    let neon = RouteVideoConfig::neon();
    assert!(neon.show_route);
  }

  #[test]
  fn test_route_video_config_custom() {
    let file_config = FileConfig::new(
      "test.fit".to_string(),
      "test.jpg".to_string(),
      "test.mp4".to_string(),
    );

    let config = RouteVideoConfig::new(
      RouteScale::new(0.5, 0.1, 0.2),
      RouteColor::default(),
      PaceDistConfig::default(),
      LapDataConfig::default(),
      file_config,
      true,
      false,
      true,
    );

    assert_eq!(config.show_route, false);
    assert_eq!(config.file_config.fit_file, "test.fit");
    assert_eq!(
      config.file_config.background_image,
      "test.jpg"
    );
    assert_eq!(
      config.file_config.output_file,
      "test.mp4"
    );
  }

  #[test]
  fn test_visibility_flags() {
    let mut config = RouteVideoConfig::default();
    config.show_bottom_bar = false;
    config.show_route = false;
    config.show_lap_data = false;

    assert!(!config.show_bottom_bar);
    assert!(!config.show_route);
    assert!(!config.show_lap_data);
  }
}
