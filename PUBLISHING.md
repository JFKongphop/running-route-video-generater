# Publishing Runarium to crates.io

This guide walks you through publishing the `runarium` library to crates.io so others can install it with `cargo add runarium`.

## Prerequisites

1. **Create a crates.io account**
   - Go to https://crates.io/
   - Sign in with GitHub

2. **Get your API token**
   - Visit https://crates.io/me
   - Create a new token
   - Save it securely

3. **Configure cargo**
   ```bash
   cargo login <your-api-token>
   ```

## Pre-Publishing Checklist

### 1. Update Cargo.toml

Make sure your `Cargo.toml` has all required fields:

```toml
[package]
name = "runarium"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]  # ← Update this!
description = "Generate animated videos from GPS running/cycling data"
repository = "https://github.com/JFKongphop/runarium"
homepage = "https://github.com/JFKongphop/runarium"
documentation = "https://docs.rs/runarium"
readme = "README.md"
license = "MIT OR Apache-2.0"  # ← Choose your license
keywords = ["gps", "fitness", "video", "visualization", "running"]
categories = ["multimedia::video", "visualization", "command-line-utilities"]
```

### 2. Choose and Add a License

**Option A: MIT License**
```bash
curl https://opensource.org/licenses/MIT > LICENSE-MIT
```

**Option B: Apache 2.0 License**
```bash
curl https://www.apache.org/licenses/LICENSE-2.0.txt > LICENSE-APACHE
```

**Option C: Dual License (Recommended)**
Add both licenses and specify in Cargo.toml:
```toml
license = "MIT OR Apache-2.0"
```

Then update README.md:
```markdown
## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
```

### 3. Update Code to Use New Package Name

Update `src/main.rs` to use the new crate name:

```rust
use anyhow::Result;
use runarium::generators::route_video::progressive_route_with_config;
use runarium::configs::video_config::RouteVideoConfig;
use runarium::utils::performance::measure;

fn main() -> Result<()> {
    let config = RouteVideoConfig::default();

    measure("Total execution", || {
        progressive_route_with_config(config)
    })?;

    Ok(())
}
```

### 4. Add System Requirements Documentation

Create `SYSTEM_REQUIREMENTS.md`:

```markdown
# System Requirements for Runarium

## OpenCV Installation

Runarium requires OpenCV to be installed on your system.

### macOS
\`\`\`bash
brew install opencv llvm
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export SDKROOT="$(xcrun --show-sdk-path)"
export CPATH="$SDKROOT/usr/include"
export CPLUS_INCLUDE_PATH="$SDKROOT/usr/include/c++/v1:$SDKROOT/usr/include"
\`\`\`

### Linux
\`\`\`bash
sudo apt-get install libopencv-dev clang libclang-dev
export LIBCLANG_PATH=/usr/lib/llvm-14/lib
\`\`\`

### Windows
\`\`\`bash
vcpkg install opencv4
\`\`\`
```

### 5. Verify the Package

```bash
# Check for issues
cargo check

# Run tests
cargo test

# Build documentation locally
cargo doc --open

# Package the crate (doesn't publish)
cargo package

# List files that will be included
cargo package --list
```

### 6. Create .gitignore (if not exists)

```bash
cat > .gitignore << 'EOF'
/target
/outputs
*.mp4
*.png
Cargo.lock
.DS_Store
EOF
```

### 7. Exclude Unnecessary Files from Package

Add to `Cargo.toml`:

```toml
[package]
# ... other fields ...
exclude = [
    "source/*",
    "outputs/*",
    "*.mp4",
    "*.png",
    ".github/*",
]
```

## Publishing Steps

### Step 1: Dry Run

```bash
# This checks everything but doesn't actually publish
cargo publish --dry-run
```

Fix any warnings or errors that appear.

### Step 2: Build and Test

```bash
# Clean build
cargo clean
cargo build --release

# Run tests
cargo test

# Check documentation
cargo doc --no-deps
```

### Step 3: Publish to crates.io

```bash
# Publish for real!
cargo publish
```

### Step 4: Verify Publication

1. Visit https://crates.io/crates/runarium
2. Check that documentation is building at https://docs.rs/runarium
3. Test installation in a new project:

```bash
cargo new test-runarium
cd test-runarium
cargo add runarium
```

## After Publishing

### 1. Create a Git Tag

```bash
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

### 2. Create GitHub Release

1. Go to https://github.com/JFKongphop/runarium/releases
2. Click "Draft a new release"
3. Choose the tag `v0.1.0`
4. Add release notes
5. Publish release

### 3. Monitor docs.rs

Check that documentation builds successfully:
- Visit https://docs.rs/runarium
- If build fails, check the logs

## Updating the Package

When you want to release a new version:

### 1. Update Version

In `Cargo.toml`:
```toml
version = "0.1.1"  # or 0.2.0 for breaking changes
```

Follow [Semantic Versioning](https://semver.org/):
- `0.1.1` - Patch (bug fixes)
- `0.2.0` - Minor (new features, backwards compatible)
- `1.0.0` - Major (breaking changes)

### 2. Update CHANGELOG

Create `CHANGELOG.md`:
```markdown
# Changelog

## [0.1.1] - 2025-12-03
### Fixed
- Bug fixes

### Added
- New features

## [0.1.0] - 2025-12-03
- Initial release
```

### 3. Publish Update

```bash
cargo publish
git tag -a v0.1.1 -m "Release version 0.1.1"
git push origin v0.1.1
```

## Common Issues

### Issue: "crate name already exists"

**Solution:** Choose a different name in `Cargo.toml`. Check availability at https://crates.io/

### Issue: "failed to verify package"

**Solution:** Make sure all local paths in dependencies are removed or properly configured.

### Issue: "documentation failed to build"

**Solution:** 
- Ensure all examples compile
- Fix any broken documentation links
- Test with `cargo doc --no-deps`

### Issue: OpenCV dependency problems

**Solution:** Add a build.rs file to detect OpenCV:

```rust
// build.rs
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Verify OpenCV is available
    if let Err(e) = pkg_config::probe_library("opencv4") {
        eprintln!("Warning: Could not find opencv4: {}", e);
        eprintln!("Please install OpenCV. See SYSTEM_REQUIREMENTS.md");
    }
}
```

Add to `Cargo.toml`:
```toml
[build-dependencies]
pkg-config = "0.3"
```

## Resources

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Policies](https://crates.io/policies)
- [Semantic Versioning](https://semver.org/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Quick Reference

```bash
# Login to crates.io
cargo login <token>

# Check package
cargo check
cargo test
cargo package --list

# Dry run
cargo publish --dry-run

# Publish
cargo publish

# Yank a version (if needed)
cargo yank --vers 0.1.0
```

## Need Help?

- Check the [Cargo Book](https://doc.rust-lang.org/cargo/)
- Ask on [Rust Users Forum](https://users.rust-lang.org/)
- Visit [Rust Discord](https://discord.gg/rust-lang)
