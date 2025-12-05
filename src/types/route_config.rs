/// Configuration for route scaling and positioning on the map
#[derive(Debug, Clone, Copy)]
pub struct RouteScale {
  /// Scale factor for route visualization (0.0-1.0 recommended)
  pub scale: f64,
  /// Horizontal offset as percentage of image width (0.0-1.0)
  pub offset_x_percent: f64,
  /// Vertical offset as percentage of image height (0.0-1.0)
  pub offset_y_percent: f64,
}

impl RouteScale {
  /// Creates a new RouteScale with custom values
  pub fn new(scale: f64, offset_x_percent: f64, offset_y_percent: f64) -> Self {
    Self {
      scale,
      offset_x_percent,
      offset_y_percent,
    }
  }

  /// Creates a default RouteScale (small route, top-left corner)
  pub fn default() -> Self {
    Self {
      scale: 0.2,
      offset_x_percent: 0.1,
      offset_y_percent: 0.1,
    }
  }

  /// Creates a centered RouteScale with medium size
  pub fn centered() -> Self {
    Self {
      scale: 0.4,
      offset_x_percent: 0.3,
      offset_y_percent: 0.3,
    }
  }

  /// Creates a large RouteScale that fills most of the map
  pub fn large() -> Self {
    Self {
      scale: 0.7,
      offset_x_percent: 0.15,
      offset_y_percent: 0.15,
    }
  }
}

/// Color configuration for different route elements (BGRA format)
#[derive(Debug, Clone, Copy)]
pub struct RouteColor {
  /// Color for the route line (default: red)
  pub route_line: [f64; 4],
  /// Color for the current position marker (default: green)
  pub current_position: [f64; 4],
  /// Color for text elements (default: white)
  pub text: [f64; 4],
  /// Color for lap statistics bars (default: green)
  pub lap_bars: [f64; 4],
}

impl RouteColor {
  /// Creates a new RouteColor with custom colors
  pub fn new(
    route_line: [f64; 4],
    current_position: [f64; 4],
    text: [f64; 4],
    lap_bars: [f64; 4],
  ) -> Self {
    Self {
      route_line,
      current_position,
      text,
      lap_bars,
    }
  }

  /// Creates default colors (red route, green marker, white text)
  pub fn default() -> Self {
    Self {
      route_line: [0.0, 0.0, 255.0, 0.0], // Red
      current_position: [0.0, 255.0, 0.0, 0.0], // Green
      text: [255.0, 255.0, 255.0, 0.0],   // White
      lap_bars: [0.0, 255.0, 0.0, 0.0],   // Green
    }
  }

  /// Creates a blue color scheme
  pub fn blue_scheme() -> Self {
    Self {
      route_line: [255.0, 0.0, 0.0, 0.0], // Blue
      current_position: [255.0, 255.0, 0.0, 0.0], // Cyan
      text: [255.0, 255.0, 255.0, 0.0],   // White
      lap_bars: [255.0, 128.0, 0.0, 0.0], // Orange-Blue
    }
  }

  /// Creates a vibrant neon scheme
  pub fn neon_scheme() -> Self {
    Self {
      route_line: [255.0, 0.0, 255.0, 0.0], // Magenta
      current_position: [0.0, 255.0, 255.0, 0.0], // Yellow
      text: [255.0, 255.0, 255.0, 0.0],     // White
      lap_bars: [255.0, 0.0, 255.0, 0.0],   // Magenta
    }
  }
}

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

/// Color options for lap data text
#[derive(Debug, Clone, Copy)]
pub enum Color {
  Black,
  White,
  Red,
  Orange,
  Yellow,
  YellowGreen,
  Green,
  BlueGreen,
  Blue,
  BlueViolet,
  Violet,
  RedViolet,
  RedOrange,
  YellowOrange,
}

impl Color {
  /// Get BGRA color value
  pub fn to_bgra(&self) -> [f64; 4] {
    match self {
      Color::Black => [0.0, 0.0, 0.0, 0.0],
      Color::White => [255.0, 255.0, 255.0, 0.0],
      Color::Red => [0.0, 0.0, 255.0, 0.0],
      Color::Orange => [0.0, 165.0, 255.0, 0.0],
      Color::Yellow => [0.0, 255.0, 255.0, 0.0],
      Color::YellowGreen => [47.0, 255.0, 173.0, 0.0],
      Color::Green => [0.0, 255.0, 0.0, 0.0],
      Color::BlueGreen => [128.0, 255.0, 0.0, 0.0],
      Color::Blue => [255.0, 0.0, 0.0, 0.0],
      Color::BlueViolet => [226.0, 43.0, 138.0, 0.0],
      Color::Violet => [211.0, 0.0, 148.0, 0.0],
      Color::RedViolet => [211.0, 0.0, 199.0, 0.0],
      Color::RedOrange => [0.0, 69.0, 255.0, 0.0],
      Color::YellowOrange => [0.0, 204.0, 255.0, 0.0],
    }
  }
}

/// Font family options for text rendering
#[derive(Debug, Clone, Copy)]
pub enum Font {
  /// Normal size sans-serif font
  Simplex,
  /// Small size sans-serif font
  Plain,
  /// Normal size sans-serif font (more complex than Simplex)
  Duplex,
  /// Normal size serif font
  Complex,
  /// Normal size serif font (more complex than Complex)
  Triplex,
  /// Smaller font than Complex
  ComplexSmall,
  /// Script style font
  ScriptSimplex,
  /// Script style font (more complex than ScriptSimplex)
  ScriptComplex,
  /// Italic font style
  Italic,
}

impl Font {
  /// Get OpenCV font constant
  pub fn to_opencv(&self) -> i32 {
    match self {
      Font::Simplex => 0,        // FONT_HERSHEY_SIMPLEX
      Font::Plain => 1,           // FONT_HERSHEY_PLAIN
      Font::Duplex => 2,          // FONT_HERSHEY_DUPLEX
      Font::Complex => 3,         // FONT_HERSHEY_COMPLEX
      Font::Triplex => 4,         // FONT_HERSHEY_TRIPLEX
      Font::ComplexSmall => 5,    // FONT_HERSHEY_COMPLEX_SMALL
      Font::ScriptSimplex => 6,   // FONT_HERSHEY_SCRIPT_SIMPLEX
      Font::ScriptComplex => 7,   // FONT_HERSHEY_SCRIPT_COMPLEX
      Font::Italic => 16,         // FONT_ITALIC (can be combined with others)
    }
  }
}

/// Configuration for lap statistics panel
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
  /// Maximum width for pace bars (fixed at 200)
  pub bar_max_width: i32,
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
    bar_max_width: i32,
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
      bar_max_width,
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
      bar_max_width: 200,
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
      bar_max_width: 200,
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
      bar_max_width: 200,
      show_heart_rate: true,
      show_stride_length: true,
      show_pace_bars: true,
    }
  }
}

/// File paths configuration
#[derive(Debug, Clone)]
pub struct FileConfig {
  /// Path to FIT file
  pub fit_file: String,
  /// Path to background image
  pub background_image: String,
  /// Output video file path
  pub output_file: String,
}

impl FileConfig {
  /// Creates a new FileConfig with custom paths
  pub fn new(
    fit_file: String,
    background_image: String,
    output_file: String,
  ) -> Self {
    Self {
      fit_file,
      background_image,
      output_file,
    }
  }

  /// Creates default file configuration
  pub fn default() -> Self {
    Self {
      fit_file: "source/car.fit".to_string(),
      background_image: "source/car.jpg".to_string(),
      output_file: "outputs/car.mp4".to_string(),
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
    assert_eq!(Color::Red.to_bgra(), [0.0, 0.0, 255.0, 0.0]);
    assert_eq!(Color::Green.to_bgra(), [0.0, 255.0, 0.0, 0.0]);
    assert_eq!(Color::Blue.to_bgra(), [255.0, 0.0, 0.0, 0.0]);
    assert_eq!(Color::White.to_bgra(), [255.0, 255.0, 255.0, 0.0]);
    assert_eq!(Color::Black.to_bgra(), [0.0, 0.0, 0.0, 0.0]);
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
    assert_eq!(default.file_config.fit_file, "source/car.fit");
    assert_eq!(default.file_config.background_image, "source/car.jpg");
    assert_eq!(default.file_config.output_file, "outputs/car.mp4");

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
    assert_eq!(config.file_config.background_image, "test.jpg");
    assert_eq!(config.file_config.output_file, "test.mp4");
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
