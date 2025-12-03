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
      route_line: [0.0, 0.0, 255.0, 0.0],      // Red
      current_position: [0.0, 255.0, 0.0, 0.0], // Green
      text: [255.0, 255.0, 255.0, 0.0],         // White
      lap_bars: [0.0, 255.0, 0.0, 0.0],         // Green
    }
  }

  /// Creates a blue color scheme
  pub fn blue_scheme() -> Self {
    Self {
      route_line: [255.0, 0.0, 0.0, 0.0],       // Blue
      current_position: [255.0, 255.0, 0.0, 0.0], // Cyan
      text: [255.0, 255.0, 255.0, 0.0],         // White
      lap_bars: [255.0, 128.0, 0.0, 0.0],       // Orange-Blue
    }
  }

  /// Creates a vibrant neon scheme
  pub fn neon_scheme() -> Self {
    Self {
      route_line: [255.0, 0.0, 255.0, 0.0],     // Magenta
      current_position: [0.0, 255.0, 255.0, 0.0], // Yellow
      text: [255.0, 255.0, 255.0, 0.0],         // White
      lap_bars: [255.0, 0.0, 255.0, 0.0],       // Magenta
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
    position: Option<(i32, i32)>,
    show_pace: bool,
    show_distance: bool,
  ) -> Self {
    Self {
      font_scale,
      thickness,
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
      position: None,
      show_pace: true,
      show_distance: false,
    }
  }
}

/// Text color options for lap data
#[derive(Debug, Clone, Copy)]
pub enum TextColor {
  Black,
  White,
}

impl TextColor {
  /// Get BGRA color value
  pub fn to_bgra(&self) -> [f64; 4] {
    match self {
      TextColor::Black => [0.0, 0.0, 0.0, 0.0],
      TextColor::White => [255.0, 255.0, 255.0, 0.0],
    }
  }
}

/// Configuration for lap statistics panel
#[derive(Debug, Clone)]
pub struct LapDataConfig {
  /// Position of the lap panel as percentage (x_percent, y_percent) where 0.0-1.0
  pub position: (f64, f64),
  /// Text color for lap data
  pub text_color: TextColor,
  /// Whether to show heart rate
  pub show_heart_rate: bool,
  /// Whether to show stride length
  pub show_stride_length: bool,
  /// Whether to show pace bars
  pub show_pace_bars: bool,
}

impl LapDataConfig {
  /// Creates a new LapDataConfig with custom settings
  /// Note: font_scale (0.5), thickness (1), and bar_max_width (200) are fixed
  pub fn new(
    position: (f64, f64),
    text_color: TextColor,
    show_heart_rate: bool,
    show_stride_length: bool,
    show_pace_bars: bool,
  ) -> Self {
    Self {
      position,
      text_color,
      show_heart_rate,
      show_stride_length,
      show_pace_bars,
    }
  }

  /// Creates default configuration
  pub fn default() -> Self {
    Self {
      position: (0.5, 0.09),  // 50% x, 9% y
      text_color: TextColor::White,
      show_heart_rate: true,
      show_stride_length: true,
      show_pace_bars: true,
    }
  }

  /// Creates minimal configuration (pace only, no extras)
  pub fn minimal() -> Self {
    Self {
      position: (0.5, 0.09),  // 50% x, 9% y
      text_color: TextColor::White,
      show_heart_rate: false,
      show_stride_length: false,
      show_pace_bars: true,
    }
  }

  /// Creates detailed configuration (all stats, larger bars)
  pub fn detailed() -> Self {
    Self {
      position: (0.5, 0.07),  // 50% x, 7% y
      text_color: TextColor::White,
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
}

impl RouteVideoConfig {
  /// Creates a new RouteVideoConfig with all custom settings
  pub fn new(
    route_scale: RouteScale,
    colors: RouteColor,
    pace_dist: PaceDistConfig,
    lap_data: LapDataConfig,
  ) -> Self {
    Self {
      route_scale,
      colors,
      pace_dist,
      lap_data,
    }
  }

  /// Creates default configuration
  pub fn default() -> Self {
    Self {
      route_scale: RouteScale::default(),
      colors: RouteColor::default(),
      pace_dist: PaceDistConfig::default(),
      lap_data: LapDataConfig::default(),
    }
  }

  /// Creates a minimalist configuration
  pub fn minimalist() -> Self {
    Self {
      route_scale: RouteScale::default(),
      colors: RouteColor::default(),
      pace_dist: PaceDistConfig::pace_only(),
      lap_data: LapDataConfig::minimal(),
    }
  }

  /// Creates a detailed configuration with all features
  pub fn detailed() -> Self {
    Self {
      route_scale: RouteScale::large(),
      colors: RouteColor::default(),
      pace_dist: PaceDistConfig::large_text(),
      lap_data: LapDataConfig::detailed(),
    }
  }

  /// Creates a vibrant neon-themed configuration
  pub fn neon() -> Self {
    Self {
      route_scale: RouteScale::centered(),
      colors: RouteColor::neon_scheme(),
      pace_dist: PaceDistConfig::default(),
      lap_data: LapDataConfig::default(),
    }
  }
}
