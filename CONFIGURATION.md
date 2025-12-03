# Custom Configuration Guide

Learn how to customize your route videos using the new configuration types.

## Configuration Types

### 1. RouteScale
Controls the size and position of your route on the map.

```rust
use runarium::types::route_config::RouteScale;

// Preset options
let small = RouteScale::default();      // Small route, top-left
let medium = RouteScale::centered();    // Medium, centered
let large = RouteScale::large();        // Large, fills most of map

// Custom
let custom = RouteScale::new(
    0.4,   // scale: 40% of map size
    0.3,   // offset_x: 30% from left
    0.2,   // offset_y: 20% from top
);
```

### 2. RouteColor
Customize colors for different elements (BGRA format).

```rust
use runarium::types::route_config::RouteColor;

// Preset themes
let default = RouteColor::default();      // Red route, green marker
let blue = RouteColor::blue_scheme();     // Blue theme
let neon = RouteColor::neon_scheme();     // Vibrant neon colors

// Custom colors
let custom = RouteColor::new(
    [255.0, 0.0, 0.0, 0.0],      // Blue route line
    [0.0, 255.0, 255.0, 0.0],    // Yellow position marker
    [255.0, 255.0, 255.0, 0.0],  // White text
    [200.0, 100.0, 0.0, 0.0],    // Purple bars
);
```

**Common Colors (BGRA format):**
- Red: `[0.0, 0.0, 255.0, 0.0]`
- Green: `[0.0, 255.0, 0.0, 0.0]`
- Blue: `[255.0, 0.0, 0.0, 0.0]`
- Yellow: `[0.0, 255.0, 255.0, 0.0]`
- Cyan: `[255.0, 255.0, 0.0, 0.0]`
- Magenta: `[255.0, 0.0, 255.0, 0.0]`
- White: `[255.0, 255.0, 255.0, 0.0]`
- Black: `[0.0, 0.0, 0.0, 0.0]`

### 3. PaceDistConfig
Configure the pace and distance overlay.

```rust
use runarium::types::route_config::PaceDistConfig;

// Preset options
let default = PaceDistConfig::default();      // Both pace and distance
let large = PaceDistConfig::large_text();     // Larger text
let pace_only = PaceDistConfig::pace_only();  // Only show pace

// Custom
let custom = PaceDistConfig::new(
    0.7,   // font_scale: larger text
    2,     // thickness: bold
    None,  // position: auto
    true,  // show_pace
    false, // show_distance: hide distance
);
```

### 4. LapDataConfig
Configure the lap statistics panel.

**Note:** `font_scale` (0.5), `thickness` (1), and `bar_max_width` (200) are now fixed values.

```rust
use runarium::types::route_config::{LapDataConfig, Color};

// Preset options
let default = LapDataConfig::default();    // All stats, white text
let minimal = LapDataConfig::minimal();    // Pace only, white text
let detailed = LapDataConfig::detailed();  // All stats, positioned higher

// Custom
let custom = LapDataConfig::new(
    (0.5, 0.09),      // position: percentage (50% x, 9% y)
    Color::YellowGreen, // text_color: from Color enum
    true,             // show_heart_rate
    true,             // show_stride_length
    true,             // show_pace_bars
);
```

**Available Colors:**
- Basic: `Color::Black`, `Color::White`
- Primary: `Color::Red`, `Color::Green`, `Color::Blue`
- Secondary: `Color::Orange`, `Color::Yellow`, `Color::Violet`
- Compound: `Color::YellowGreen`, `Color::BlueGreen`, `Color::BlueViolet`, `Color::RedViolet`, `Color::RedOrange`, `Color::YellowOrange`

### 5. RouteVideoConfig
Complete configuration combining all settings, plus visibility flags.

```rust
use runarium::types::route_config::RouteVideoConfig;

// Preset configurations (no width parameter needed)
let default = RouteVideoConfig::default();
let minimalist = RouteVideoConfig::minimalist();
let detailed = RouteVideoConfig::detailed();
let neon = RouteVideoConfig::neon();

// Custom combination
let custom = RouteVideoConfig::new(
    RouteScale::centered(),
    RouteColor::blue_scheme(),
    PaceDistConfig::large_text(),
    LapDataConfig::detailed(),
    true,  // show_bottom_bar: pace/distance overlay
    true,  // show_route: progressive route animation
    true,  // show_lap_data: lap statistics panel
);

// Hide specific elements
let mut route_only = RouteVideoConfig::default();
route_only.show_bottom_bar = false;  // Hide pace/distance
route_only.show_lap_data = false;    // Hide lap stats
```

## Usage Examples

### Example 1: Quick Start with Presets

```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route_with_config;
use runarium::types::route_config::RouteVideoConfig;

fn main() -> Result<()> {
    // Use a preset configuration
    let config = RouteVideoConfig::neon();
    generate_progressive_route_with_config(config)?;
    Ok()
}
```

### Example 2: Custom Colors

```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route_with_config;
use runarium::types::route_config::{RouteVideoConfig, RouteColor};

fn main() -> Result<()> {
    let mut config = RouteVideoConfig::default();
    
    // Change to blue color scheme
    config.colors = RouteColor::blue_scheme();
    
    generate_progressive_route_with_config(config)?;
    Ok()
}
```

### Example 3: Hide Specific Stats

```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route_with_config;
use runarium::types::route_config::RouteVideoConfig;

fn main() -> Result<()> {
    let mut config = RouteVideoConfig::default();
    
    // Hide heart rate and stride length
    config.lap_data.show_heart_rate = false;
    config.lap_data.show_stride_length = false;
    
    generate_progressive_route_with_config(config)?;
    Ok()
}
```

### Example 4: Large Route, Centered

```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route_with_config;
use runarium::types::route_config::{RouteVideoConfig, RouteScale};

fn main() -> Result<()> {
    let mut config = RouteVideoConfig::default();
    
    // Make route larger and centered
    config.route_scale = RouteScale::large();
    
    generate_progressive_route_with_config(config)?;
    Ok()
}
```

### Example 5: Fully Custom Configuration

```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route_with_config;
use runarium::types::route_config::*;

fn main() -> Result<()> {
    // Custom route scale
    let route_scale = RouteScale::new(0.5, 0.25, 0.25);
    
    // Custom colors - purple theme
    let colors = RouteColor::new(
        [255.0, 0.0, 128.0, 0.0],    // Purple route
        [0.0, 255.0, 255.0, 0.0],    // Yellow marker
        [255.0, 255.0, 255.0, 0.0],  // White text
        [255.0, 100.0, 200.0, 0.0],  // Pink bars
    );
    
    // Large text, pace only
    let pace_dist = PaceDistConfig::new(0.8, 2, None, true, false);
    
    // Detailed lap panel
    let lap_data = LapDataConfig::new(
        (540, 80), 0.6, 2, 300, true, true, true
    );
    
    let config = RouteVideoConfig::new(
        route_scale, colors, pace_dist, lap_data
    );
    
    generate_progressive_route_with_config(config)?;
    Ok(())
}
```

## Configuration Comparison

| Preset | Route Size | Colors | Lap Stats | Best For |
|--------|-----------|--------|-----------|----------|
| `default()` | Small, corner | Red/Green | All shown | General use |
| `minimalist()` | Small, corner | Red/Green | Pace only | Clean look |
| `detailed()` | Large, fills map | Red/Green | All + larger | Data analysis |
| `neon()` | Medium, centered | Magenta/Yellow | All shown | Eye-catching |

## Tips

1. **Route Positioning**: 
   - Use `offset` values of 0.25 to center the route
   - Use smaller `scale` (0.2-0.3) if you want to see more map
   - Use larger `scale` (0.5-0.7) to focus on the route

2. **Color Selection**:
   - Ensure good contrast against your map background
   - White text works on most backgrounds
   - Avoid similar colors for route and marker
   - Use `Color` enum for lap data text: `Color::White`, `Color::YellowGreen`, etc.

3. **Lap Data Configuration**:
   - Position uses percentages: `(0.5, 0.09)` = center horizontally, 9% from top
   - Font scale (0.5), thickness (1), and bar width (200) are fixed
   - Choose from 14 different text colors via `Color` enum

4. **Visibility Control**:
   - `show_bottom_bar`: Toggle pace/distance overlay
   - `show_route`: Toggle progressive route line animation
   - `show_lap_data`: Toggle lap statistics panel
   - Hiding elements can create cleaner, focused videos

5. **Performance**:
   - More lap stats = slightly slower rendering
   - Color choice doesn't affect performance
   - Route size doesn't affect rendering speed
   - Hiding elements doesn't improve performance significantly

## Run Examples

```bash
# Run the custom configuration example
cargo run --example custom_config --release
```

## Backward Compatibility

The original simple function still works:

```rust
use runarium::generators::route_video::generate_progressive_route;

// Old way (still supported)
generate_progressive_route(0.2, 0.1, 0.1)?;

// New way (more control)
let config = RouteVideoConfig::default(1080);
generate_progressive_route_with_config(config)?;
```
