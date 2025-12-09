# HTTP Server Usage

## Running the Server

### Local Development
```bash
# Build and run the server
cargo run --example server

# Or with release mode for better performance
cargo run --example server --release

# Using Makefile
make server
```

### Docker Deployment
```bash
# Build Docker image
make docker-build

# Start container
make docker-up

# Stop container
make docker-down
```

The server will start on `http://localhost:3000`

## API Endpoints

### 1. Health Check
```bash
GET /health
```

### 2. Generate Video
```bash
POST /generate-video
Content-Type: multipart/form-data
```

**Request Body:**
- `fit_file`: Your GPS data file (.fit format)
- `background`: Background map image (.jpg format)
- `config` (optional): JSON configuration object

**Response:**
```json
{
  "success": true,
  "message": "Video generated successfully",
  "download_url": "/download-video/550e8400-e29b-41d4-a716-446655440000",
  "video_id": "550e8400-e29b-41d4-a716-446655440000",
  "generation_time_ms": 15000
}
```

### 3. Generate Image
```bash
POST /generate-image
Content-Type: multipart/form-data
```

**Request Body:**
- `fit_file`: Your GPS data file (.fit format)
- `background`: Background map image (.jpg format)
- `config` (optional): JSON configuration object

**Response:**
```json
{
  "success": true,
  "message": "Image generated successfully",
  "download_url": "/download-image/550e8400-e29b-41d4-a716-446655440000",
  "image_id": "550e8400-e29b-41d4-a716-446655440000",
  "generation_time_ms": 1200
}
```

### 4. Download Video
```bash
GET /download-video/:video_id
```

Returns the generated MP4 video file. **One-time download only** - file is deleted after download.

### 5. Download Image
```bash
GET /download-image/:image_id
```

Returns the generated PNG image file. **One-time download only** - file is deleted after download.

## Example Usage

### Using Makefile Commands

```bash
# Test video generation with default config
make api-test-video

# Test video generation with full config
make api-test-video-config

# Test image generation with default config
make api-test-image

# Test image generation with full config
make api-test-image-config

# Check API health
make api-health
```

### Using cURL

#### Generate Video (Default Config)
```bash
curl -X POST http://localhost:3000/generate-video \
  -F "fit_file=@source/example.fit" \
  -F "background=@source/example.jpg"
```

#### Generate Video (Custom Config)
```bash
curl -X POST http://localhost:3000/generate-video \
  -F "fit_file=@source/example.fit" \
  -F "background=@source/example.jpg" \
  -F 'config={"scale":0.3,"offset_x_percent":0.15,"offset_y_percent":0.15,"route_line_color":[255.0,0.0,0.0,0.0],"show_lap_data":true}'
```

#### Generate Image (Default Config)
```bash
curl -X POST http://localhost:3000/generate-image \
  -F "fit_file=@source/example.fit" \
  -F "background=@source/example.jpg"
```

#### Generate Image (Custom Config)
```bash
curl -X POST http://localhost:3000/generate-image \
  -F "fit_file=@source/example.fit" \
  -F "background=@source/example.jpg" \
  -F 'config={"scale":0.25,"route_line_color":[0.0,255.0,0.0,0.0]}'
```

#### Download Video
```bash
# Replace VIDEO_ID with the id from response
curl -o output.mp4 http://localhost:3000/download-video/VIDEO_ID
```

#### Download Image
```bash
# Replace IMAGE_ID with the id from response
curl -o output.png http://localhost:3000/download-image/IMAGE_ID
```

### Using JavaScript/Fetch

#### Generate Video
```javascript
const formData = new FormData();
formData.append('fit_file', fitFileBlob);
formData.append('background', imageFileBlob);

// Optional: Add custom config
const config = {
  scale: 0.3,
  route_line_color: [255.0, 0.0, 0.0, 0.0],
  show_lap_data: true
};
formData.append('config', JSON.stringify(config));

const response = await fetch('http://localhost:3000/generate-video', {
  method: 'POST',
  body: formData
});

const data = await response.json();
console.log('Download URL:', data.download_url);
console.log('Generation time:', data.generation_time_ms, 'ms');

// Download video
window.location.href = `http://localhost:3000${data.download_url}`;
```

#### Generate Image
```javascript
const formData = new FormData();
formData.append('fit_file', fitFileBlob);
formData.append('background', imageFileBlob);

const response = await fetch('http://localhost:3000/generate-image', {
  method: 'POST',
  body: formData
});

const data = await response.json();
console.log('Download URL:', data.download_url);

// Download image
window.location.href = `http://localhost:3000${data.download_url}`;
```

### Using Python

#### Generate Video
```python
import requests

# Upload and generate with default config
files = {
    'fit_file': open('source/example.fit', 'rb'),
    'background': open('source/example.jpg', 'rb')
}

response = requests.post('http://localhost:3000/generate-video', files=files)
result = response.json()

print(f"Video ID: {result['video_id']}")
print(f"Download URL: {result['download_url']}")
print(f"Generation time: {result['generation_time_ms']}ms")

# Download video
video_response = requests.get(f"http://localhost:3000{result['download_url']}")
with open('output.mp4', 'wb') as f:
    f.write(video_response.content)
```

#### Generate Video with Custom Config
```python
import requests
import json

files = {
    'fit_file': open('source/example.fit', 'rb'),
    'background': open('source/example.jpg', 'rb')
}

config = {
    'scale': 0.3,
    'route_line_color': [255.0, 0.0, 0.0, 0.0],
    'show_lap_data': True,
    'show_pace': True
}

data = {
    'config': json.dumps(config)
}

response = requests.post('http://localhost:3000/generate-video', 
                        files=files, data=data)
result = response.json()
```

#### Generate Image
```python
import requests

files = {
    'fit_file': open('source/example.fit', 'rb'),
    'background': open('source/example.jpg', 'rb')
}

response = requests.post('http://localhost:3000/generate-image', files=files)
result = response.json()

# Download image
image_response = requests.get(f"http://localhost:3000{result['download_url']}")
with open('output.png', 'wb') as f:
    f.write(image_response.content)
```

### Using Postman

#### Generate Video
1. Create a new POST request to `http://localhost:3000/generate-video`
2. Go to "Body" tab → Select "form-data"
3. Add fields:
   - Key: `fit_file`, Type: File, Value: Select your .fit file
   - Key: `background`, Type: File, Value: Select your .jpg file
   - Key: `config` (optional), Type: Text, Value: JSON config string
4. Click "Send"
5. Copy the `download_url` from response
6. Create a new GET request to `http://localhost:3000{download_url}`
7. Click "Send and Download"

#### Generate Image
1. Create a new POST request to `http://localhost:3000/generate-image`
2. Follow same steps as video generation
3. Download from the returned `download_url`

## Configuration Options

All configuration parameters are optional. If not provided, default values will be used.

### Available Config Parameters

```json
{
  "scale": 0.2,                    // Route scale (default: 0.2)
  "offset_x_percent": 0.1,         // X offset percentage (default: 0.1)
  "offset_y_percent": 0.1,         // Y offset percentage (default: 0.1)
  
  "route_line_color": [0.0, 0.0, 255.0, 0.0],           // Red (BGRA format)
  "current_position_color": [0.0, 255.0, 0.0, 0.0],     // Green
  "text_color": [255.0, 255.0, 255.0, 0.0],             // White
  "lap_bars_color": [0.0, 165.0, 255.0, 0.0],           // Orange
  
  "pace_font_scale": 0.6,          // Pace text size (default: 0.6)
  "pace_thickness": 2,             // Pace text thickness (default: 2)
  
  "lap_position_x": 0.5,           // Lap data X position (default: 0.5)
  "lap_position_y": 0.09,          // Lap data Y position (default: 0.09)
  "lap_font_scale": 0.5,           // Lap text size (default: 0.5)
  "lap_thickness": 1,              // Lap text thickness (default: 1)
  
  "show_bottom_bar": true,         // Show bottom bar (default: true)
  "show_route": true,              // Show route line (default: true)
  "show_lap_data": true,           // Show lap information (default: true)
  "show_pace": true,               // Show pace (default: true)
  "show_distance": true,           // Show distance (default: true)
  "show_heart_rate": true,         // Show heart rate (default: true)
  "show_stride_length": true,      // Show stride length (default: true)
  "show_pace_bars": true           // Show pace bars (default: true)
}
```

### Color Format (BGRA)
Colors are specified in BGRA format (Blue, Green, Red, Alpha):
- Red: `[0.0, 0.0, 255.0, 0.0]`
- Green: `[0.0, 255.0, 0.0, 0.0]`
- Blue: `[255.0, 0.0, 0.0, 0.0]`
- White: `[255.0, 255.0, 255.0, 0.0]`
- Orange: `[0.0, 165.0, 255.0, 0.0]`

## File Storage

- **In-Memory Only**: Videos and images are stored in memory (not saved to disk)
- **One-Time Download**: Files are automatically deleted after download
- **Temporary Processing**: Files are temporarily written during generation, then cleaned up
- **System Temp Directory**: Uses OS temp directory for cross-platform compatibility

## Limits

- Maximum file size: 100MB (configurable via `DefaultBodyLimit`)
- Concurrent requests: Handled by tokio async runtime
- Memory usage: Videos/images stored in RAM until downloaded

## Docker Configuration

The Docker image is optimized for production:
- Multi-stage build (~600MB final size)
- Based on `debian:bookworm-slim`
- Includes OpenCV runtime libraries
- Exposes port 3000
- Memory limits: 2GB max, 512MB reserved
