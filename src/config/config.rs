use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub device: Device,
    pub resolution: Resolution,
    pub picture: Picture,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub vid: u16,
    pub pid: u16,
}

#[derive(Debug, Deserialize)]
pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Picture {
    SolidColor {
        color: u16,
    },
    Image {
        file: String,
    },
    Video {
        file: String,
        fps: u32,
    },
}

pub fn load_config(path: &str) -> Result<Config> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read config from {}", path))?;
    let cfg: Config = toml::from_str(&text)
        .context("failed to parse TOML config:")?;

    println!("Config file: {}", fs::canonicalize(path).unwrap().display());
    println!("Device: {:04x}:{:04x}", cfg.device.vid, cfg.device.pid);
    println!("Resolution: {}x{}", cfg.resolution.width, cfg.resolution.height);
 
    match &cfg.picture {
        Picture::SolidColor { color } => {
            println!("Media type: solid color");
            println!("RGB565 color: 0x{:04x}", color);
        },
        Picture::Image { file } => {
            println!("Media type: image");
            println!("File path: {}", file);
        },
        Picture::Video { file, fps } => {
            println!("Media type: video");
            println!("File path: {}", file);
            println!("FPS: {}", fps);
        },
    }

    Ok(cfg)
}

