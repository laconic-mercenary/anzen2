# Camera Matrix Setup Guide

This guide shows how to set up the camera matrix interface with MediaMTX for monitoring IP camera feeds.

## Prerequisites

- MediaMTX installed and running
- FFmpeg installed (for testing)
- Rust server running (`cargo run`)

## MediaMTX Configuration

### 1. Configure mediamtx.yml

Edit your `mediamtx.yml` file to enable HLS and configure paths:

```yaml
# HLS server configuration
hls: true
hlsAddress: :8888
hlsEncryption: false

# Global path defaults (allows dynamic path creation)
pathDefaults:
  publishUser: ""
  publishPass: ""

# Or configure specific paths
paths:
  test1:
    publishUser: ""
    publishPass: ""
    
  test2:
    publishUser: ""
    publishPass: ""
    
  test3:
    publishUser: ""
    publishPass: ""
```

### 2. Start MediaMTX

```bash
./mediamtx
```

## Testing with FFmpeg

Create test video streams using FFmpeg:

### Test Pattern 1 (Color Bars)
```bash
ffmpeg -f lavfi -i testsrc=size=1280x720:rate=30 -c:v libx264 -preset ultrafast -pix_fmt yuv420p -f flv rtmp://localhost:1935/test1
```

### Test Pattern 2 (Moving Gradient)
```bash
ffmpeg -f lavfi -i testsrc2=size=1280x720:rate=30 -c:v libx264 -preset ultrafast -pix_fmt yuv420p -f flv rtmp://localhost:1935/test2
```

### Test Pattern 3 (RGB Color Cycling)
```bash
ffmpeg -f lavfi -i rgbtestsrc=size=1280x720:rate=30 -c:v libx264 -preset ultrafast -pix_fmt yuv420p -f flv rtmp://localhost:1935/test3
```

## Using the Camera Matrix Interface

### 1. Access the Interface

Navigate to: `http://localhost:8080/mtx-monitor`

### 2. Add Camera Feeds

Click "Add Camera" and enter the following examples:

#### Test Pattern 1
- **Camera Name:** `Test Pattern 1`
- **Stream URL:** `http://localhost:8888/test1/index.m3u8`

#### Test Pattern 2
- **Camera Name:** `Test Pattern 2`
- **Stream URL:** `http://localhost:8888/test2/index.m3u8`

#### Test Pattern 3
- **Camera Name:** `Test Pattern 3`
- **Stream URL:** `http://localhost:8888/test3/index.m3u8`

### 3. Real IP Camera Examples

For actual IP cameras, use these URL formats:

#### Basic RTSP Camera
- **Camera Name:** `Front Door`
- **Stream URL:** `http://localhost:8888/front_door/index.m3u8`

Configure in mediamtx.yml:
```yaml
paths:
  front_door:
    source: rtsp://192.168.1.100:554/stream1
```

#### Authenticated RTSP Camera
- **Camera Name:** `Garage Camera`
- **Stream URL:** `http://localhost:8888/garage/index.m3u8`

Configure in mediamtx.yml:
```yaml
paths:
  garage:
    source: rtsp://admin:password123@192.168.1.101:554/live/main
```

## Troubleshooting

### Common Issues

1. **"path not configured" error**: Make sure the path is defined in mediamtx.yml
2. **HLS 500 error**: Ensure MediaMTX HLS is enabled and FFmpeg stream is running
3. **Video won't play**: Check browser console for errors, ensure HLS.js is loaded

### Verify Setup

- Check MediaMTX web interface: `http://localhost:8888/`
- Test HLS endpoint directly: `curl -I http://localhost:8888/test1/index.m3u8`
- View MediaMTX logs for connection status

## Features

- **Responsive Grid Layout**: Automatically adjusts to screen size
- **Dynamic Addition**: Add/remove cameras without page reload
- **HLS Support**: Works with MediaMTX HLS streams
- **Error Handling**: Shows alerts for failed stream connections
- **Multiple Protocols**: Supports any camera type that MediaMTX can ingest

## Camera Matrix Controls

- **Add Camera**: Click to add new camera feed
- **Remove Camera**: Click × button on any camera feed
- **Responsive Layout**: Grid automatically adjusts as cameras are added
- **Keyboard Shortcuts**: ESC to close add camera dialog