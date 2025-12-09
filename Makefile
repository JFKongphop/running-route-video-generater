# Runarium Makefile

.PHONY: help build run clean test examples video image check fmt fmt-check clippy docker-build docker-up docker-down api-test-video api-test-video-config api-test-image api-test-image-config api-health

help:
	@echo "Available commands:"
	@echo "  make build        - Build the project"
	@echo "  make run          - Run the main application"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make test         - Run tests"
	@echo "  make examples     - Build all examples"
	@echo "  make video        - Run video generation example"
	@echo "  make image        - Run image generation example"
	@echo "  make server       - Run HTTP server example"
	@echo "  make check        - Run cargo check"
	@echo "  make fmt          - Format code (requires nightly)"
	@echo "  make fmt-check    - Check code formatting"
	@echo "  make clippy       - Run clippy linter"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-up    - Start Docker container"
	@echo "  make docker-down  - Stop Docker container"
	@echo "  make api-test-video - Test video generation with default config"
	@echo "  make api-test-video-config - Test video generation with full config"
	@echo "  make api-test-image - Test image generation with default config"
	@echo "  make api-test-image-config - Test image generation with full config"
	@echo "  make api-health   - Check API health"

build:
	cargo build

run:
	cargo run

clean:
	cargo clean

test:
	cargo test

examples:
	cargo build --examples

video:
	cargo run --example video_config

image:
	cargo run --example image_config

server:
	cargo run --example server

check:
	cargo check --all-features --lib --bins

fmt:
	cargo +nightly fmt

fmt-check:
	cargo +nightly fmt --all -- --check

clippy:
	cargo clippy --all-features --lib --bins -- -D warnings

build:
	docker compose build

up:
	docker compose up -d

down:
	docker compose down

# Test API endpoints
api-test-video:
	curl -X POST http://localhost:3000/generate-video \
	  -F "fit_file=@source/example.fit" \
	  -F "background=@source/example.jpg" | jq

api-test-video-config:
	curl -X POST http://localhost:3000/generate-video \
	  -F "fit_file=@source/example.fit" \
	  -F "background=@source/example.jpg" \
	  -F 'config={"scale":0.2,"offset_x_percent":0.1,"offset_y_percent":0.1,"route_line_color":[0.0,0.0,255.0,0.0],"current_position_color":[0.0,255.0,0.0,0.0],"text_color":[255.0,255.0,255.0,0.0],"lap_bars_color":[0.0,165.0,255.0,0.0],"pace_font_scale":0.6,"pace_thickness":2,"lap_position_x":0.5,"lap_position_y":0.09,"lap_font_scale":0.5,"lap_thickness":1,"show_bottom_bar":true,"show_route":true,"show_lap_data":true,"show_pace":true,"show_distance":true,"show_heart_rate":true,"show_stride_length":true,"show_pace_bars":true}' | jq

api-test-image:
	curl -X POST http://localhost:3000/generate-image \
	  -F "fit_file=@source/example.fit" \
	  -F "background=@source/example.jpg" | jq

api-test-image-config:
	curl -X POST http://localhost:3000/generate-image \
	  -F "fit_file=@source/example.fit" \
	  -F "background=@source/example.jpg" \
	  -F 'config={"scale":0.2,"offset_x_percent":0.1,"offset_y_percent":0.1,"route_line_color":[0.0,0.0,255.0,0.0],"current_position_color":[0.0,255.0,0.0,0.0],"text_color":[255.0,255.0,255.0,0.0],"lap_bars_color":[0.0,165.0,255.0,0.0],"pace_font_scale":0.6,"pace_thickness":2,"lap_position_x":0.5,"lap_position_y":0.09,"lap_font_scale":0.5,"lap_thickness":1,"show_bottom_bar":true,"show_route":true,"show_lap_data":true,"show_pace":true,"show_distance":true,"show_heart_rate":true,"show_stride_length":true,"show_pace_bars":true}' | jq

api-health:
	curl http://localhost:3000/health