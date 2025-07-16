# vid2gif

A fast and efficient CLI tool for converting videos to GIFs using ffmpeg.

## Features

- Convert any video format supported by ffmpeg to GIF
- Adjustable playback speed (0.5x for slow motion, 2x for fast forward)
- Optional looping control
- Optimized GIF output with palette generation for better quality
- Automatic scaling to 640px width for reasonable file sizes
- Progress feedback and file size comparison

## Prerequisites

- **ffmpeg** must be installed and available in your PATH
- Rust (for building from source)

### Installing ffmpeg

**Ubuntu/Debian:**
```bash
sudo apt update && sudo apt install ffmpeg
```

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**
Download from [ffmpeg.org](https://ffmpeg.org/download.html) or use chocolatey:
```bash
choco install ffmpeg
```

## Installation

### From Source
```bash
git clone <this-repo>
cd vid2gif
cargo build --release
sudo cp target/release/vid2gif /usr/local/bin/
```

### Using Cargo
```bash
cargo install --path .
```

## Usage

```bash
vid2gif --input input.webm --output output.gif [OPTIONS]
```

### Options

- `--input, -i <FILE>`: Input video file (required)
- `--output, -o <FILE>`: Output GIF file (required)  
- `--speed, -s <FLOAT>`: Speed multiplier (default: 1.0)
  - `0.5` = half speed (slow motion)
  - `1.0` = normal speed
  - `2.0` = double speed (fast forward)
- `--loop`: Enable infinite looping for the GIF

### Examples

**Basic conversion:**
```bash
vid2gif --input screencast.webm --output screencast.gif
```

**Slow motion with looping:**
```bash
vid2gif --input action.mp4 --output action.gif --speed 0.5 --loop
```

**Fast forward:**
```bash
vid2gif --input tutorial.webm --output tutorial.gif --speed 2.0
```

**Full example:**
```bash
vid2gif --loop --speed 1.5 --input "Screencast From 2025-07-15 14-38-16.webm" --output "screencast.gif"
```

## Technical Details

The tool uses ffmpeg with optimized settings for GIF conversion:

- **Frame rate**: Dynamically calculated based on speed (5-30 fps range)
- **Scaling**: Resized to 640px width maintaining aspect ratio
- **Quality**: Uses Lanczos scaling and palette generation for optimal quality
- **Compression**: Generates optimized 255-color palette

## Supported Input Formats

- WebM (.webm)
- MP4 (.mp4)
- AVI (.avi)
- MOV (.mov)
- MKV (.mkv)
- FLV (.flv)
- WMV (.wmv)
- And any other format supported by your ffmpeg installation

## Error Handling

The tool provides clear error messages for common issues:
- Missing input files
- Invalid speed values
- ffmpeg not found in PATH
- Permission issues with output directory

## Performance

Conversion speed depends on:
- Input video length and resolution
- Output speed setting
- System performance
- ffmpeg version and configuration

Typical conversion times:
- 10-second 1080p screencast: ~2-5 seconds
- 30-second 4K video: ~10-20 seconds

## License

MIT License - see LICENSE file for details.
