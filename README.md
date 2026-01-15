# TRCC-Rust

A Rust CLI tool that drives ThermalRight Cooler's USB LCD Display.

> Only tested to work with 87ad:70db so far.

## Features

- Send solid color frames to USB displays
- Display images (via FFmpeg conversion)
- Stream video files with configurable FPS (via FFmpeg)
- Automatic center-crop to match display aspect ratio
- Systemd user service support

## Requirements

- FFmpeg (must be in PATH)
- USB permissions (see udev setup below)

## Udev Setup

To access the USB device without root, create a udev rule:

```bash
sudo tee /etc/udev/rules.d/99-thermalright.rules <<EOF
SUBSYSTEM=="usb", ATTR{idVendor}=="87ad", ATTR{idProduct}=="70db", MODE="0666"
EOF
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## Installation

```bash
./install.sh
```

This installs the binary via `cargo install`, copies the config to `~/.config/trcc/`, and sets up a systemd user service.

Or manually:

```bash
cargo install --path .
```

## Usage

```bash
trcc -c config.toml
```

Or via systemd:

```bash
systemctl --user start trcc
systemctl --user status trcc
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

# Solid color
[picture]
type = "solid_color"
color = 0xffc0  # RGB565

# Or an image
[picture]
type = "image"
file = "image.jpg"

# Or a video (loops infinitely)
[picture]
type = "video"
file = "video.mp4"
fps = 30
```

## License

MIT
