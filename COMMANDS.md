# Runarium - Command Reference

Quick reference for all commands related to developing and using Runarium.

## 📦 For Library Users

### Installation
```bash
# Add to existing project
cargo add runarium

# Or manually add to Cargo.toml
echo 'runarium = "0.1.0"' >> Cargo.toml
```

### Running Examples
```bash
# Generate video
cargo run --example generate_video --release

# Generate static image
cargo run --example generate_image --release

# Custom scaling demo
cargo run --example custom_scaling --release
```

### Building Your Project
```bash
# Development build (slow, with debug info)
cargo build

# Release build (fast, optimized)
cargo build --release

# Run your code
cargo run --release
```

## 🛠️ For Library Developers

### Development
```bash
# Check code compiles
cargo check

# Run tests
cargo test

# Run with warnings
cargo clippy

# Format code
cargo fmt

# Build documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

### Examples Development
```bash
# Run specific example
cargo run --example generate_video

# List all examples
ls examples/

# Build all examples
cargo build --examples
```

### Testing Before Publish
```bash
# Dry run publish
cargo publish --dry-run

# List files that will be published
cargo package --list

# Build the package
cargo package

# Test package locally
cd target/package/runarium-0.1.0
cargo build
```

### Publishing
```bash
# Login to crates.io (once)
cargo login <your-api-token>

# Publish to crates.io
cargo publish

# Yank a version if needed
cargo yank --vers 0.1.0

# Un-yank
cargo yank --vers 0.1.0 --undo
```

### Version Management
```bash
# Update version in Cargo.toml, then:
git add Cargo.toml
git commit -m "Bump version to 0.1.1"
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin main
git push origin v0.1.1
cargo publish
```

## 🔧 Environment Setup

### macOS - Initial Setup
```bash
# Install dependencies
brew install opencv llvm

# Configure environment (add to ~/.zshrc)
cat >> ~/.zshrc << 'EOF'
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export SDKROOT="$(xcrun --show-sdk-path)"
export CPATH="$SDKROOT/usr/include"
export CPLUS_INCLUDE_PATH="$SDKROOT/usr/include/c++/v1:$SDKROOT/usr/include"
EOF

# Reload configuration
source ~/.zshrc
```

### Linux - Initial Setup
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install libopencv-dev clang libclang-dev

# Configure environment (add to ~/.bashrc)
echo 'export LIBCLANG_PATH=/usr/lib/llvm-14/lib' >> ~/.bashrc
source ~/.bashrc
```

### Verify Setup
```bash
# Check OpenCV
pkg-config --modversion opencv4

# Check LLVM
llvm-config --version

# Check environment
echo $LIBCLANG_PATH
echo $SDKROOT
```

## 📊 File Management

### Required Project Structure
```bash
# Create directories
mkdir -p source outputs

# Expected files
# source/car.fit    - Your GPS data
# source/map.png   - Background map
# outputs/         - Generated files (auto-created)
```

### File Operations
```bash
# List source files
ls -lh source/

# Check FIT file
file source/car.fit

# View map image info
file source/map.png

# Check outputs
ls -lh outputs/

# Play generated video (macOS)
open outputs/car.mp4

# Play generated video (Linux)
xdg-open outputs/car.mp4
```

## 🧪 Testing Commands

### Unit Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Integration Tests
```bash
# Run examples as integration tests
cargo run --example generate_video
cargo run --example generate_image
cargo run --example custom_scaling
```

### Performance Testing
```bash
# Time execution
time cargo run --release

# With detailed metrics
cargo run --release 2>&1 | grep "execution"

# Memory usage (macOS)
/usr/bin/time -l cargo run --release

# Memory usage (Linux)
/usr/bin/time -v cargo run --release
```

## 🐛 Debugging

### Show Debug Output
```bash
# Set log level
RUST_LOG=debug cargo run

# Verbose build
cargo build -vv

# Show build script output
cargo clean && cargo build -vv 2>&1 | grep -A 20 "opencv"
```

### Common Fixes
```bash
# Fix formatting issues
cargo fmt

# Fix clippy warnings
cargo clippy --fix

# Update dependencies
cargo update

# Clean and rebuild
cargo clean && cargo build --release
```

## 📝 Documentation

### Generate Docs
```bash
# Build and open docs
cargo doc --open

# Build docs without dependencies
cargo doc --no-deps

# Build docs for all features
cargo doc --all-features
```

### README Examples
```bash
# Test README code examples
cargo readme > README_test.md
diff README.md README_test.md
```

## 🔍 Search and Inspect

### Dependency Info
```bash
# List dependencies
cargo tree

# Check for updates
cargo outdated

# Audit for security issues
cargo audit
```

### Code Statistics
```bash
# Count lines of code
find src -name "*.rs" | xargs wc -l

# Find TODO comments
grep -r "TODO" src/

# Find FIXME comments
grep -r "FIXME" src/
```

## 🚀 Performance Optimization

### Profiling
```bash
# Build with profiling
cargo build --release

# Run with profiling (requires instruments on macOS)
instruments -t "Time Profiler" target/release/runarium

# Benchmark (if divan is configured)
cargo bench
```

### Build Optimization
```bash
# Parallel build
cargo build --release -j 8

# LTO (Link Time Optimization)
cargo rustc --release -- -C lto

# Target CPU
cargo rustc --release -- -C target-cpu=native
```

## 🌐 Git Operations

### Repository Management
```bash
# Clone repository
git clone https://github.com/JFKongphop/runarium.git
cd runarium

# Create feature branch
git checkout -b feature/new-feature

# Commit changes
git add .
git commit -m "Add new feature"

# Push to remote
git push origin feature/new-feature
```

### Release Process
```bash
# Create release
git checkout main
git pull
# Update version in Cargo.toml
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin main
git push origin v0.2.0
```

## 💾 Backup and Restore

### Backup Project
```bash
# Backup (excluding build artifacts)
tar -czf runarium-backup.tar.gz \
  --exclude='target' \
  --exclude='outputs' \
  runarium/
```

### Restore Project
```bash
# Restore from backup
tar -xzf runarium-backup.tar.gz
cd runarium
cargo build --release
```

## 🎯 Quick Actions

```bash
# New user getting started
brew install opencv llvm
cargo new my-project && cd my-project
cargo add runarium
cargo run --release

# Developer making changes
cargo check && cargo clippy && cargo test
git commit -am "Your changes"
git push

# Publishing new version
# 1. Update Cargo.toml version
# 2. Update CHANGELOG.md
cargo publish --dry-run
cargo publish
git tag -a v0.x.x -m "Release v0.x.x"
git push origin main --tags
```

## 📞 Getting Help

```bash
# Cargo help
cargo --help
cargo build --help
cargo publish --help

# Check versions
cargo --version
rustc --version
opencv_version  # if installed

# Community
echo "Visit https://github.com/JFKongphop/runarium/issues"
```
