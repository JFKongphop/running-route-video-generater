# Custom Configuration Guide

Learn how to customize your route videos and images using configuration types.

## Usage Examples

### Generate Progressive Route Video

```rust
use anyhow::Result;
use runarium::configs::video_config::{
  Color, FileConfig, Font, LapDataConfig, PaceDistConfig,
  RouteColor, RouteScale, RouteVideoConfig,
};
use runarium::generators::route_video::progressive_route_with_config;

fn main() -> Result<()> {
  // Configure route scale and position
  let route_scale = RouteScale::new(
    0.2, // scale: 20% of map size
    0.1, // offset_x_percent: 10% from left
    0.1, // offset_y_percent: 10% from top
  );

  // Configure colors (BGRA format)
  let colors = RouteColor::new(
    [0.0, 0.0, 255.0, 0.0],     // route_line: Red
    [0.0, 255.0, 0.0, 0.0],     // current_position: Green
    [255.0, 255.0, 255.0, 0.0], // text: White
    [0.0, 165.0, 255.0, 0.0],   // lap_bars: Orange
  );

  // Configure pace and distance display
  let pace_dist = PaceDistConfig::new(
    0.6,           // font_scale
    2,             // thickness
    Font::Simplex, // font style
    None,          // position (auto-calculated)
    true,          // show_pace
    true,          // show_distance
  );

  // Configure lap data panel
  let lap_data = LapDataConfig::new(
    (0.5, 0.09),   // position (x_percent, y_percent)
    0.5,           // font_scale
    1,             // thickness
    Font::Simplex, // font style
    Color::White,  // text_color
    true,          // show_heart_rate
    true,          // show_stride_length
    true,          // show_pace_bars
  );

  // Configure file paths
  let file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/video.mp4".to_string(),
  );

  // Combine all configurations
  let config = RouteVideoConfig::new(
    route_scale,
    colors,
    pace_dist,
    lap_data,
    file_config,
    true, // show_bottom_bar
    true, // show_route
    true, // show_lap_data
  );

  progressive_route_with_config(config)?;
  Ok(())
}
```

### Generate Static Route Image

```rust
use anyhow::Result;
use runarium::configs::image_config::RouteImageConfig;
use runarium::configs::video_config::{FileConfig, RouteColor, RouteScale};
use runarium::generators::route_image::image_route_with_config;

fn main() -> Result<()> {
  // Configure route scale and position
  let route_scale = RouteScale::new(
    0.2, // scale: 20% of map size
    0.1, // offset_x_percent: 10% from left
    0.1, // offset_y_percent: 10% from top
  );

  // Configure colors (BGRA format)
  let colors = RouteColor::new(
    [0.0, 0.0, 255.0, 0.0],     // route_line: Red
    [0.0, 255.0, 0.0, 0.0],     // current_position: Green
    [255.0, 255.0, 255.0, 0.0], // text: White
    [0.0, 165.0, 255.0, 0.0],   // lap_bars: Orange
  );

  // Configure file paths
  let file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/route.png".to_string(),
  );

  // Create image configuration (simple, no lap data)
  let config = RouteImageConfig::new(
    route_scale,
    colors,
    file_config,
    2, // line_thickness
  );

  image_route_with_config(config)?;
  Ok(())
}
```

### Generate Static Route Image with Lap Data

```rust
use anyhow::Result;
use runarium::configs::image_config::RouteImageConfig;
use runarium::configs::video_config::{
  Color, FileConfig, Font, LapDataConfig, RouteColor, RouteScale,
};
use runarium::generators::route_image::image_route_with_config;

fn main() -> Result<()> {
  let route_scale = RouteScale::new(0.2, 0.1, 0.1);
  
  let colors = RouteColor::new(
    [0.0, 0.0, 255.0, 0.0],
    [0.0, 255.0, 0.0, 0.0],
    [255.0, 255.0, 255.0, 0.0],
    [0.0, 165.0, 255.0, 0.0],
  );
  
  let file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/route_with_laps.png".to_string(),
  );

  // Configure lap data panel
  let lap_data = LapDataConfig::new(
    (0.5, 0.09),   // position
    0.5,           // font_scale
    1,             // thickness
    Font::Simplex, // font
    Color::White,  // text_color
    true,          // show_heart_rate
    true,          // show_stride_length
    true,          // show_pace_bars
  );

  // Create image configuration with lap data
  let config = RouteImageConfig::with_lap_data(
    route_scale,
    colors,
    file_config,
    2, // line_thickness
    lap_data,
  );

  image_route_with_config(config)?;
  Ok(())
}
```

## Configuration Reference

**Common Colors (BGRA format):**
- Red: `[0.0, 0.0, 255.0, 0.0]`
- Green: `[0.0, 255.0, 0.0, 0.0]`
- Blue: `[255.0, 0.0, 0.0, 0.0]`
- Yellow: `[0.0, 255.0, 255.0, 0.0]`
- Cyan: `[255.0, 255.0, 0.0, 0.0]`
- Magenta: `[255.0, 0.0, 255.0, 0.0]`
- White: `[255.0, 255.0, 255.0, 0.0]`
- Black: `[0.0, 0.0, 0.0, 0.0]`

**Available Color Enum:**
- Basic: `Color::Black`, `Color::White`
- Primary: `Color::Red`, `Color::Green`, `Color::Blue`
- Secondary: `Color::Orange`, `Color::Yellow`, `Color::Violet`
- Compound: `Color::YellowGreen`, `Color::BlueGreen`, `Color::BlueViolet`, `Color::RedViolet`, `Color::RedOrange`, `Color::YellowOrange`

**Available Font Styles:**
- `Font::Simplex` - Normal size sans-serif
- `Font::Plain` - Small size sans-serif
- `Font::Duplex` - More complex variant of Simplex
- `Font::Complex` - Normal size serif
- `Font::Triplex` - More complex variant of Complex
- `Font::ComplexSmall` - Smaller version of Complex
- `Font::ScriptSimplex` - Hand-writing style
- `Font::ScriptComplex` - More complex variant of ScriptSimplex
- `Font::Italic` - Italic variant of Simplex
