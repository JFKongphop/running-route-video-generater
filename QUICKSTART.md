# Quick Start Guide

Get started with Runarium in 5 minutes!

## Step 1: Install OpenCV

### macOS
```bash
# Install OpenCV and LLVM
brew install opencv llvm

# Set up environment (add to ~/.zshrc)
cat >> ~/.zshrc << 'EOF'
# Runarium dependencies
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export SDKROOT="$(xcrun --show-sdk-path)"
export CPATH="$SDKROOT/usr/include"
export CPLUS_INCLUDE_PATH="$SDKROOT/usr/include/c++/v1:$SDKROOT/usr/include"
EOF

# Reload shell configuration
source ~/.zshrc
```

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install libopencv-dev clang libclang-dev

# Set environment variable
echo 'export LIBCLANG_PATH=/usr/lib/llvm-14/lib' >> ~/.bashrc
source ~/.bashrc
```

### Windows
```powershell
# Install vcpkg if not installed
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# Install OpenCV
.\vcpkg install opencv4:x64-windows
```

## Step 2: Create a New Project

```bash
cargo new my-route-video
cd my-route-video
```

## Step 3: Add Runarium Dependency

Add to `Cargo.toml`:
```toml
[dependencies]
runarium = "0.1.0"
anyhow = "1.0"
```

Or use cargo:
```bash
cargo add runarium anyhow
```

## Step 4: Prepare Your Files

Create the required directory structure:
```bash
mkdir -p source outputs
```

You need:
1. **FIT file** - Your GPS data (from Garmin, Strava, etc.)
2. **Map image** - Background map (PNG format)

```
my-route-video/
├── source/
│   ├── car.fit       ← Your GPS data
│   └── map.png      ← Background map
└── outputs/
    └── (generated files will go here)
```

### Where to get a map image?

**Option 1: Export from Strava**
1. Go to your activity on Strava
2. Right-click on the map → Save image as...
3. Save as `map.png`

**Option 2: Use OpenStreetMap**
1. Go to https://www.openstreetmap.org/export
2. Select your area
3. Click "Export" → Save as PNG

**Option 3: Google Maps Screenshot**
1. Navigate to your area on Google Maps
2. Take a screenshot
3. Crop and save as `map.png`

## Step 5: Write Your Code

Edit `src/main.rs`:

```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route;
use runarium::utils::performance::measure;

fn main() -> Result<()> {
    println!("🎬 Generating route video...\n");

    // Configure your route visualization
    let route_scale = 0.2;        // 20% of map size
    let offset_x_percent = 0.1;   // 10% from left
    let offset_y_percent = 0.1;   // 10% from top

    // Generate the video
    measure("Total execution", || {
        generate_progressive_route(
            route_scale,
            offset_x_percent,
            offset_y_percent,
        )
    })?;

    println!("\n✅ Done! Check outputs/car.mp4");
    Ok(())
}
```

## Step 6: Run It!

```bash
cargo run --release
```

You should see:
```
🎬 Generating route video...

Processed 100/5199 points
Processed 200/5199 points
...
✅ Video created: outputs/car.mp4 with 5199 points
⏱️ Total execution: 27.56s

✅ Done! Check outputs/car.mp4
```

## Step 7: View Your Video

Open the generated video:

```bash
# macOS
open outputs/car.mp4

# Linux
xdg-open outputs/car.mp4

# Windows
start outputs/car.mp4
```

## Configuration Tips

### Adjust Route Size
```rust
let route_scale = 0.3;  // Increase for larger route
```

### Center the Route
```rust
let offset_x_percent = 0.25;  // Move right
let offset_y_percent = 0.25;  // Move down
```

### Different Positions
```rust
// Top-left corner
(0.1, 0.1)

// Top-right corner  
(0.6, 0.1)

// Bottom-left corner
(0.1, 0.6)

// Bottom-right corner
(0.6, 0.6)

// Centered
(0.25, 0.25)
```

## Next Steps

### Generate Static Image Instead

```rust
use runarium::generators::running_route_image::generate_running_route_image;

fn main() -> Result<()> {
    generate_running_route_image(0.2, 0.1, 0.1)?;
    Ok(())
}
```

### Run Examples

```bash
# Generate video
cargo run --example generate_video

# Generate static image
cargo run --example generate_image

# Try custom scaling
cargo run --example custom_scaling
```

### Read the Full Documentation

- [README.md](README.md) - Complete API reference
- [PUBLISHING.md](PUBLISHING.md) - If you want to contribute

## Troubleshooting

### "Could not find OpenCV"

Make sure OpenCV is installed and environment variables are set:
```bash
# Check OpenCV installation
pkg-config --modversion opencv4

# Check LIBCLANG_PATH
echo $LIBCLANG_PATH
```

### "FIT file not found"

Make sure your FIT file is at `source/car.fit`:
```bash
ls -la source/
```

### "Map image not found"

Make sure your map is at `source/map.png`:
```bash
file source/map.png
```

### Build is slow

Use release mode for much faster execution:
```bash
cargo build --release
cargo run --release
```

## Need Help?

- Check the [README](README.md) for detailed documentation
- See [PUBLISHING.md](PUBLISHING.md) for advanced topics
- Open an issue on [GitHub](https://github.com/JFKongphop/runarium/issues)

## Example Output

Your generated video will show:
- ✅ Animated route being drawn progressively
- ✅ Current position marker (green dot)
- ✅ Real-time pace and distance info
- ✅ Lap statistics panel (pace, heart rate, stride)
- ✅ 30 FPS smooth animation

Enjoy your route visualization! 🎉
