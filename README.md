# Runarium 🏃‍♂️

A Rust library for generating animated videos and static images from GPS running/cycling data. Convert your FIT files into beautiful route visualizations with customizable configurations.

## Features

- 📍 Parse GPS data from FIT files (Garmin, Polar, etc.)
- 🎬 Generate animated route videos with progressive drawing
- 🖼️ Generate static route images
- 📊 Display real-time statistics (pace, heart rate, distance)
- 🗺️ Overlay routes on custom background images
- 🎨 Fully customizable colors, fonts, and styling
- ⚡ Fast rendering with OpenCV
- ⚙️ Configuration-based API for easy customization

## Prerequisites

Before installing, you need to have OpenCV installed on your system.

### macOS

```bash
# Install OpenCV via Homebrew
brew install opencv

# Install LLVM (required for Rust bindings)
brew install llvm

# Set up environment variables (add to ~/.zshrc or ~/.bash_profile)
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export SDKROOT="$(xcrun --show-sdk-path)"
export CPATH="$SDKROOT/usr/include"
export CPLUS_INCLUDE_PATH="$SDKROOT/usr/include/c++/v1:$SDKROOT/usr/include"
```

### Linux (Ubuntu/Debian)

```bash
# Install OpenCV development files
sudo apt-get update
sudo apt-get install libopencv-dev clang libclang-dev

# Set environment variable
export LIBCLANG_PATH=/usr/lib/llvm-14/lib  # Adjust version as needed
```

### Windows

```bash
# Install via vcpkg
vcpkg install opencv4

# Set environment variables
set OPENCV_LINK_LIBS=opencv_world4
set OPENCV_LINK_PATHS=C:\path\to\vcpkg\installed\x64-windows\lib
set OPENCV_INCLUDE_PATHS=C:\path\to\vcpkg\installed\x64-windows\include
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
runarium = "0.1.0"  # Replace with actual version
```

Or install via cargo:

```bash
cargo add runarium
```

## Usage

### Quick Start - Video Generation

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
    0.2,  // scale
    0.1,  // offset_x_percent
    0.1   // offset_y_percent
  );
  
  // Configure colors (BGRA format)
  let colors = RouteColor::new(
    [0.0, 0.0, 255.0, 0.0],     // route_line_color (Red)
    [0.0, 255.0, 0.0, 0.0],     // current_position_color (Green)
    [255.0, 255.0, 255.0, 0.0], // text_color (White)
    [0.0, 165.0, 255.0, 0.0]    // lap_bars_color (Orange)
  );
  
  // Configure pace/distance display
  let pace_dist = PaceDistConfig::new(
    0.6,          // pace_font_scale
    2,            // pace_thickness
    Font::Simplex,// font_face
    None,         // position (auto)
    true,         // show_pace
    true          // show_distance
  );
  
  // Configure lap data panel
  let lap_data = LapDataConfig::new(
    (0.5, 0.09),  // position (x, y)
    0.5,          // lap_font_scale
    1,            // lap_thickness
    Font::Simplex,// font_face
    Color::White, // text_color
    true,         // show_heart_rate
    true,         // show_stride_length
    true          // show_pace_bars
  );
  
  // Set file paths
  let file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/video.mp4".to_string(),
  );
  
  // Create and run configuration
  let config = RouteVideoConfig::new(
    route_scale,  // route scale settings
    colors,       // color configuration
    pace_dist,    // pace/distance config
    lap_data,     // lap data panel config
    file_config,  // file paths
    true,         // show_bottom_bar
    true,         // show_route
    true          // show_lap_data
  );
  
  progressive_route_with_config(config)?;
  Ok(())
}
```

### Quick Start - Static Image

```rust
use anyhow::Result;
use runarium::configs::image_config::RouteImageConfig;
use runarium::configs::video_config::{FileConfig, RouteColor, RouteScale};
use runarium::generators::route_image::image_route_with_config;

fn main() -> Result<()> {
  let route_scale = RouteScale::new(
    0.2,  // scale
    0.1,  // offset_x_percent
    0.1   // offset_y_percent
  );
  
  let colors = RouteColor::new(
    [0.0, 0.0, 255.0, 0.0],     // route_line_color (Red)
    [0.0, 255.0, 0.0, 0.0],     // current_position_color (Green)
    [255.0, 255.0, 255.0, 0.0], // text_color (White)
    [0.0, 165.0, 255.0, 0.0]    // lap_bars_color (Orange)
  );
  
  let file_config = FileConfig::new(
    "source/example.fit".to_string(),
    "source/example.jpg".to_string(),
    "outputs/route.png".to_string(),
  );
  
  let config = RouteImageConfig::new(
    route_scale,  // route scale settings
    colors,       // color configuration
    file_config,  // file paths
    2             // line_thickness
  );
  
  image_route_with_config(config)?;
  Ok(())
}
```

### File Structure Required

```
your-project/
├── source/
│   ├── example.fit       # Your FIT file with GPS data
│   └── example.jpg       # Background map image
└── outputs/
    ├── video.mp4         # Generated video (created automatically)
    └── route.png         # Generated image (created automatically)
```

## Configuration

See [CONFIGURATION.md](CONFIGURATION.md) for detailed configuration options including:
- Route scaling and positioning
- Color customization
- Font styles
- Lap data display options
- Visibility controls

### Available Colors

```rust
Color::Black, Color::White, Color::Red, Color::Green, Color::Blue,
Color::Orange, Color::Yellow, Color::Violet, Color::YellowGreen,
Color::BlueGreen, Color::BlueViolet, Color::RedViolet,
Color::RedOrange, Color::YellowOrange
```

### Available Fonts

```rust
Font::Simplex, Font::Plain, Font::Duplex, Font::Complex,
Font::Triplex, Font::ComplexSmall, Font::ScriptSimplex,
Font::ScriptComplex, Font::Italic
```

## HTTP Server

Runarium includes an HTTP server for generating videos and images via REST API.

### Running the Server

```bash
# Local development
cargo run --example server --release

# Using Docker
make docker-build
make docker-up

# Using Makefile
make server
```

### API Endpoints

- `POST /generate-video` - Generate animated route video
- `POST /generate-image` - Generate static route image
- `GET /download-video/:video_id` - Download generated video (one-time)
- `GET /download-image/:image_id` - Download generated image (one-time)
- `GET /health` - Health check

### Quick Test

```bash
# Test video generation
make api-test-video

# Test video with custom config
make api-test-video-config

# Test image generation
make api-test-image

# Test image with custom config
make api-test-image-config
```

### Example cURL Request

```bash
# Generate video with custom configuration
curl -X POST http://localhost:3000/generate-video \
  -F "fit_file=@source/example.fit" \
  -F "background=@source/example.jpg" \
  -F 'config={
    "scale": 0.3,
    "offset_x_percent": 0.15,
    "offset_y_percent": 0.15,
    "route_line_color": [255.0, 0.0, 0.0, 0.0],
    "show_lap_data": true,
    "show_pace": true
  }'

# Download the video (replace VIDEO_ID with response id)
curl -o output.mp4 http://localhost:3000/download-video/VIDEO_ID
```

See [SERVER.md](SERVER.md) for complete API documentation.

## Examples

Run the included examples:

```bash
# Generate a video with configuration
cargo run --example video_config --release

# Generate a static image with configuration
cargo run --example image_config --release
```

## API Reference

### Video Generation

#### `progressive_route_with_config`

Generates an animated video showing the route being drawn progressively with full configuration control.

```rust
pub fn progressive_route_with_config(config: RouteVideoConfig) -> Result<()>
```

**Configuration includes:**
- Route scale and positioning (`RouteScale`)
- Colors for route, markers, text, and bars (`RouteColor`)
- Pace/distance display settings (`PaceDistConfig`)
- Lap statistics panel settings (`LapDataConfig`)
- File paths (`FileConfig`)
- Visibility flags (show_bottom_bar, show_route, show_lap_data)

**Output Features:**
- Animated route drawing
- Current position marker
- Real-time pace and distance overlay
- Lap statistics panel with heart rate, stride length, and pace bars

### Image Generation

#### `image_route_with_config`

Generates a static image of the complete route with customizable styling.

```rust
pub fn image_route_with_config(config: RouteImageConfig) -> Result<()>
```

**Configuration includes:**
- Route scale and positioning (`RouteScale`)
- Route line color (`RouteColor`)
- File paths (`FileConfig`)
- Line thickness

### Legacy API

Simple functions without configuration are still available:

```rust
// Video generation (simple)
pub fn progressive_route(
    route_scale: f64,
    offset_x_percent: f64,
    offset_y_percent: f64,
) -> Result<()>

// Image generation (simple)
pub fn route_image(
    route_scale: f64,
    offset_x_percent: f64,
    offset_y_percent: f64,
) -> Result<()>
```

### Utility Functions

#### Performance Measurement

```rust
use runarium::utils::performance::measure;

measure("Operation name", || {
    // Your code here
    Ok(())
})?;
```

## Project Structure

```
runarium/
├── src/
│   ├── configs/          # Configuration types
│   │   ├── config.rs     # Shared configs (RouteScale, RouteColor, FileConfig)
│   │   ├── image_config.rs   # Image-specific config
│   │   └── video_config.rs   # Video-specific config
│   ├── generators/       # Main generation functions
│   │   ├── route_image.rs    # Static image generation
│   │   └── route_video.rs    # Animated video generation
│   ├── types/            # Data types
│   │   ├── drawer_data.rs    # Drawing utilities data
│   │   └── fit_data.rs       # FIT file data structures
│   └── utils/            # Utility functions
│       ├── converter.rs      # Coordinate conversion
│       ├── creator.rs        # Image/video creation
│       ├── element_drawer.rs # Drawing utilities
│       ├── performance.rs    # Performance measurement
│       └── read_file.rs      # FIT file reading
├── examples/
│   ├── video_config.rs   # Video generation example
│   ├── image_config.rs   # Image generation example
│   └── server.rs         # HTTP API server
├── CONFIGURATION.md      # Detailed configuration guide
├── SERVER.md             # HTTP server documentation
├── Dockerfile            # Docker container setup
└── docker-compose.yml    # Docker orchestration
```

## Troubleshooting

### OpenCV Not Found

```
error: failed to run custom build command for `opencv`
```

**Solution:** Make sure OpenCV is installed and environment variables are set:
```bash
# macOS
brew install opencv llvm
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"

# Check if OpenCV is found
pkg-config --modversion opencv4
```

### LIBCLANG_PATH Error

```
couldn't find any valid shared libraries matching: ['libclang.dylib']
```

**Solution:** Set the LIBCLANG_PATH environment variable:
```bash
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
```

Make it permanent by adding to `~/.zshrc`:
```bash
echo 'export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"' >> ~/.zshrc
source ~/.zshrc
```

### FIT File Not Found

```
ERROR: field size: 1 is not a multiple of the base type
```

**Solution:** Ensure your FIT file is in the `source/` directory and is a valid FIT file format.

## Performance

Typical performance metrics:
- Processing ~5000 GPS points: ~12 seconds
- Video encoding: Real-time (30 FPS)
- Image generation: <1 second
- Memory usage: ~100-200 MB

## Module Overview

- **configs**: Configuration types for customizing output
- **generators**: Core functions for video and image generation
- **types**: Data structures for FIT data and drawing
- **utils**: Helper utilities for file I/O, conversion, and rendering

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your license here]

## Credits

Built with:
- [opencv-rust](https://github.com/twistedfall/opencv-rust) - OpenCV bindings for Rust
- [fitparser](https://github.com/stadelmanma/fitparse-rs) - FIT file parser
- [image](https://github.com/image-rs/image) - Image processing

## Roadmap

- [x] Support for FIT file parsing
- [x] Animated video generation
- [x] Static image generation
- [x] Customizable color schemes
- [x] Configuration-based API
- [x] Font customization
- [x] HTTP REST API server
- [x] Docker deployment support
- [ ] Support for multiple file formats (GPX, TCX)
- [ ] Command-line interface
- [ ] Preset configuration templates
- [ ] Web-based visualization
- [ ] Real-time preview
