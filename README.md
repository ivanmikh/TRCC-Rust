# TRCC-Rust

A Rust CLI tool that drives ThermalRight Cooler's USB LCD Display.

> Only tested to work with 87ad:70db so far.

## Features

- Send solid color frames to USB displays
- Stream images (via FFmpeg conversion to RGB565)
- Configurable display resolution and FPS

## Installation

```bash
cargo build --release
```

## Usage

```bash
trcc-rust -c config.toml
```

## Configuration

Create a TOML config file:

```toml
[device]
vid = 0x87ad
pid = 0x70db

[resolution]
width = 320
height = 320

[picture]
type = "solid"
color = 0xffc0 # RGB565

# Or for an image:
# type = "image"
# path = "image.jpg"
```

## License

MIT
