use std::env;
use std::fs;
use std::process::Command;

use anyhow::{Context, Result};

pub fn ffmpeg_convert_image_to_rg565ble(
    filepath: &str,
    width: usize,
    height: usize,
) -> Result<Vec<u8>> {
    let output = env::temp_dir().join("trcc_frame.rgb565");
    let output_str = output.to_string_lossy();

    let scale = format!("scale={}:{}:flags=lanczos", width, height);
    let args = [
        "-i", filepath,
        "-vf", &scale,
        "-frames:v", "1",
        "-pix_fmt", "rgb565le",
        "-f", "rawvideo",
        "-y",
        &output_str,
    ];

    println!("ffmpeg {}", args.join(" "));
    let status = Command::new("ffmpeg").args(&args).status()?;
    if !status.success() {
        anyhow::bail!("ffmpeg failed");
    }

    let pixels = fs::read(&output).with_context(|| format!("failed to read {:?}", output))?;

    // Clean up temp file
    let _ = fs::remove_file(&output);

    if pixels.len() != width * height * 2 {
        anyhow::bail!(
            "rgb565 file has wrong size: {}, expected {}",
            pixels.len(),
            width * height * 2
        );
    }

    Ok(pixels)
}
