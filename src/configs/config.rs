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

  /// Creates default FileConfig
  pub fn default() -> Self {
    Self {
      fit_file: "source/car.fit".to_string(),
      background_image: "source/car.jpg".to_string(),
      output_file: "outputs/route.mp4".to_string(),
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
