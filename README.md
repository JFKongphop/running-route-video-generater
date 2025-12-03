# Runarium 🏃‍♂️

A Rust library for generating animated videos from GPS running/cycling data. Convert your FIT files into beautiful route visualization videos with real-time statistics.

## Features

- 📍 Parse GPS data from FIT files (Garmin, Polar, etc.)
- 🎬 Generate animated route videos with progressive drawing
- 📊 Display real-time statistics (pace, heart rate, distance)
- 🗺️ Overlay routes on custom background images
- ⚡ Fast video encoding with OpenCV

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

### Basic Example

```rust
use runarium::generators::route_video::generate_progressive_route;
use runarium::utils::performance::measure;
use anyhow::Result;

fn main() -> Result<()> {
    // Configure visualization parameters
    let route_scale = 0.2;        // Scale factor for route (20% of original)
    let offset_x_percent = 0.1;   // 10% horizontal offset
    let offset_y_percent = 0.1;   // 10% vertical offset

    // Generate the video with performance measurement
    measure("Video generation", || {
        generate_progressive_route(
            route_scale,
            offset_x_percent,
            offset_y_percent,
        )
    })?;

    Ok(())
}
```

### File Structure Required

```
your-project/
├── source/
│   ├── car.fit           # Your FIT file with GPS data
│   └── map.png          # Background map image
└── outputs/
    └── car.mp4          # Generated video (created automatically)
```

### Generate Static Route Image

```rust
use runarium::generators::running_route_image::generate_running_route_image;
use anyhow::Result;

fn main() -> Result<()> {
    let route_scale = 0.2;
    let offset_x_percent = 0.1;
    let offset_y_percent = 0.1;

    generate_running_route_image(
        route_scale,
        offset_x_percent,
        offset_y_percent,
    )?;

    Ok(())
}
```

## API Reference

### Main Functions

#### `generate_progressive_route`

Generates an animated video showing the route being drawn progressively.

```rust
pub fn generate_progressive_route(
    route_scale: f64,
    offset_x_percent: f64,
    offset_y_percent: f64,
) -> Result<()>
```

**Parameters:**
- `route_scale`: Scale factor for route visualization (0.0-1.0 recommended)
- `offset_x_percent`: Horizontal offset as percentage of image width (0.0-1.0)
- `offset_y_percent`: Vertical offset as percentage of image height (0.0-1.0)

**Returns:**
- `Ok(())`: Video successfully created at `outputs/car.mp4`
- `Err`: If FIT file reading, video encoding, or drawing operations fail

**Output Features:**
- Animated route drawing (red line)
- Current position marker (green dot)
- Lap statistics panel (pace, heart rate, stride length)
- Real-time pace and distance overlay

#### `generate_running_route_image`

Generates a static image of the complete route.

```rust
pub fn generate_running_route_image(
    route_scale: f64,
    offset_x_percent: f64,
    offset_y_percent: f64,
) -> Result<()>
```

**Parameters:** Same as `generate_progressive_route`

**Returns:** 
- `Ok(())`: Image successfully created at `outputs/running_route.png`
- `Err`: If processing fails

### Utility Functions

#### Performance Measurement

```rust
use runarium::utils::performance::measure;

measure("Operation name", || {
    // Your code here
    Ok(())
})?;
```

#### FIT File Reading

```rust
use runarium::utils::read_file::fit_reader;

let (route_data, lap_data) = fit_reader("path/to/file.fit")?;
```

## Customization

### Adjust Video Parameters

The generated video has the following default settings:
- Frame rate: 30 FPS
- Resolution: Based on input map image
- Codec: H.264 (MP4V)
- One frame per GPS point

### Customize Colors and Styles

You can modify the drawing colors and styles by using the `Drawer` utility:

```rust
use runarium::utils::element_drawer::Drawer;
use opencv::core::Mat;

let mut drawer = Drawer::new(&mut frame);

// Custom colors (BGRA format)
let red = drawer.color([0.0, 0.0, 255.0, 0.0]);
let green = drawer.color([0.0, 255.0, 0.0, 0.0]);
let blue = drawer.color([255.0, 0.0, 0.0, 0.0]);
```

## Examples

### Example 1: Generate Video with Custom Scaling

```rust
use runarium::generators::route_video::generate_progressive_route;
use anyhow::Result;

fn main() -> Result<()> {
    // Larger route, centered on map
    generate_progressive_route(0.5, 0.25, 0.25)?;
    Ok(())
}
```

### Example 2: Batch Processing Multiple Routes

```rust
use runarium::generators::route_video::generate_progressive_route;
use anyhow::Result;

fn main() -> Result<()> {
    let routes = vec![
        ("route1.fit", 0.2, 0.1, 0.1),
        ("route2.fit", 0.3, 0.2, 0.2),
    ];

    for (file, scale, x, y) in routes {
        println!("Processing {}", file);
        // Note: You'll need to modify the code to accept file paths
        generate_progressive_route(scale, x, y)?;
    }

    Ok(())
}
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
- Processing ~5000 GPS points: ~25-30 seconds
- Video encoding: Real-time (30 FPS)
- Memory usage: ~100-200 MB

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

- [ ] Support for multiple file formats (GPX, TCX)
- [ ] Customizable color schemes
- [ ] Configuration file support
- [ ] Command-line interface
- [ ] Web-based visualization
- [ ] Real-time preview
