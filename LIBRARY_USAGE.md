# Runarium Library Usage Summary

## 📦 Installation for Users

When you publish `runarium` to crates.io, users can install it easily:

### 1. Install System Dependencies (One-time setup)

#### macOS
```bash
brew install opencv llvm
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export SDKROOT="$(xcrun --show-sdk-path)"
export CPATH="$SDKROOT/usr/include"
export CPLUS_INCLUDE_PATH="$SDKROOT/usr/include/c++/v1:$SDKROOT/usr/include"
```

#### Linux
```bash
sudo apt-get install libopencv-dev clang libclang-dev
export LIBCLANG_PATH=/usr/lib/llvm-14/lib
```

### 2. Add to Project

```bash
cargo add runarium
```

Or in `Cargo.toml`:
```toml
[dependencies]
runarium = "0.1.0"
```

### 3. Use in Code

```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route;

fn main() -> Result<()> {
    generate_progressive_route(0.2, 0.1, 0.1)?;
    Ok(())
}
```

## 🔧 Key Differences from Installing with OpenCV Paths

### ❌ OLD WAY (Manual OpenCV Setup)
Users had to:
1. Find OpenCV installation paths manually
2. Set multiple environment variables
3. Configure build scripts
4. Handle platform-specific paths

Example:
```bash
export OPENCV_INCLUDE_PATHS=/usr/local/include/opencv4
export OPENCV_LINK_PATHS=/usr/local/lib
export OPENCV_LINK_LIBS=opencv_core,opencv_imgproc,...
```

### ✅ NEW WAY (Published Library)
Users only need:
1. Install OpenCV via package manager (brew/apt/vcpkg)
2. Set LIBCLANG_PATH
3. `cargo add runarium`

Much simpler! The library handles OpenCV detection automatically.

## 📋 What's Included in the Published Package

When you run `cargo publish`, users get:

### Code Structure
```
runarium/
├── src/
│   ├── lib.rs              (Library entry point)
│   ├── generators/         (Video & image generation)
│   ├── types/             (Data structures)
│   └── utils/             (Helper functions)
├── examples/              (Usage examples)
├── README.md             (Documentation)
└── Cargo.toml           (Dependencies & metadata)
```

### Excluded from Package
```
source/        (Your FIT files - not published)
outputs/       (Generated videos - not published)
target/        (Build artifacts - not published)
.git/          (Git history - not published)
```

## 🚀 Publishing Checklist

Before running `cargo publish`:

- [ ] Update `Cargo.toml` with your info
  ```toml
  authors = ["Your Name <email@example.com>"]
  license = "MIT OR Apache-2.0"
  repository = "https://github.com/JFKongphop/runarium"
  ```

- [ ] Add license files
  ```bash
  # Choose one or both
  curl https://opensource.org/licenses/MIT > LICENSE-MIT
  curl https://www.apache.org/licenses/LICENSE-2.0.txt > LICENSE-APACHE
  ```

- [ ] Test the package
  ```bash
  cargo check
  cargo test
  cargo doc --open
  ```

- [ ] Package and verify
  ```bash
  cargo package --list
  cargo publish --dry-run
  ```

- [ ] Publish!
  ```bash
  cargo login <your-crates-io-token>
  cargo publish
  ```

## 📚 Documentation Files Created

1. **README.md** - Main documentation with:
   - Installation instructions
   - API reference
   - Usage examples
   - Troubleshooting

2. **QUICKSTART.md** - 5-minute getting started guide

3. **PUBLISHING.md** - Complete guide to publish the crate

4. **examples/** - Working code examples:
   - `generate_video.rs` - Create animated video
   - `generate_image.rs` - Create static image
   - `custom_scaling.rs` - Custom positioning

## 🎯 User Experience

### Before Publishing (Current)
```bash
# User needs to:
git clone https://github.com/JFKongphop/runarium
cd runarium
# Set up OpenCV paths manually
cargo build
```

### After Publishing (Goal)
```bash
# User can simply:
cargo new my-project
cd my-project
cargo add runarium
# Write code using runarium
cargo run
```

## 💡 Benefits of Publishing

1. **Easy Installation**: Users just `cargo add runarium`
2. **Version Management**: Semantic versioning (0.1.0, 0.2.0, etc.)
3. **Documentation**: Auto-generated at docs.rs
4. **Discoverability**: Listed on crates.io
5. **Trust**: Official Rust package registry
6. **Updates**: Users can upgrade with `cargo update`

## 🔗 Important Links After Publishing

- **Crate Page**: https://crates.io/crates/runarium
- **Documentation**: https://docs.rs/runarium
- **Repository**: https://github.com/JFKongphop/runarium
- **Issues**: https://github.com/JFKongphop/runarium/issues

## 📊 Example Usage After Publishing

### User's Cargo.toml
```toml
[package]
name = "my-running-videos"
version = "0.1.0"
edition = "2021"

[dependencies]
runarium = "0.1.0"
anyhow = "1.0"
```

### User's main.rs
```rust
use anyhow::Result;
use runarium::generators::route_video::generate_progressive_route;

fn main() -> Result<()> {
    // That's it! No complex setup needed
    generate_progressive_route(0.2, 0.1, 0.1)?;
    Ok(())
}
```

### User's Terminal
```bash
$ cargo run --release
    Updating crates.io index
   Compiling runarium v0.1.0
   Compiling my-running-videos v0.1.0
    Finished release [optimized] target(s) in 2m 13s
     Running `target/release/my-running-videos`
Processed 5199/5199 points
✅ Video created: outputs/car.mp4
⏱️ Total execution: 27.56s
```

## 🎉 Summary

Once published to crates.io, **runarium** becomes:
- ✅ Easy to install (`cargo add runarium`)
- ✅ Well documented (README + docs.rs)
- ✅ Version controlled (SemVer)
- ✅ Discoverable (crates.io search)
- ✅ Professional (Official Rust ecosystem)

Users no longer need to:
- ❌ Clone the repo
- ❌ Set complex OpenCV paths
- ❌ Understand your build system
- ❌ Manage dependencies manually

They just install OpenCV via their system package manager, add your crate, and start coding!
