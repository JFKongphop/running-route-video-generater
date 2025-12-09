# Build stage
FROM rustlang/rust:nightly-bookworm as builder

# Install OpenCV dependencies
RUN apt-get update && apt-get install -y \
  libopencv-dev \
  clang \
  libclang-dev \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests and source code
COPY Cargo.toml Cargo.lock ./
COPY rustfmt.toml ./
COPY src ./src
COPY examples ./examples
COPY source ./source

# Build the server example and strip symbols
RUN cargo build --release --example server && strip target/release/examples/server

# Runtime stage - minimal Debian slim
FROM debian:bookworm-slim

# Install only runtime OpenCV libraries
RUN apt-get update && apt-get install -y --no-install-recommends \
  libopencv-core4.6 \
  libopencv-imgproc4.6 \
  libopencv-highgui4.6 \
  libopencv-imgcodecs4.6 \
  libopencv-videoio4.6 \
  libopencv-video4.6 \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the stripped binary
COPY --from=builder /app/target/release/examples/server /app/server

# Expose port
EXPOSE 3000

# Run the server
CMD ["/app/server"]
