use std::fs;
use anyhow::{Result, Context};
use std::process::Command;

pub fn ffmpeg_convert_image_to_rg565ble(
    filepath: &str,
    width: usize,
    height: usize
) -> Result<Vec<u8>> {
    let w = width.to_string();
    let h = height.to_string();
    let scale = format!("scale={}:{}:flags=lanczos", w, h);
    let output = "img.rgb565";
    let args = [
        "-i", filepath,
        "-vf", &scale,
        "-frames:v", "1",
        "-pix_fmt", "rgb565le",
        "-f", "rawvideo",
        "-y",
        &output
    ];

    println!("ffmpeg {}", args.join(" "));
    let status = Command::new("ffmpeg").args(&args).status()?;
    if !status.success() {
        anyhow::bail!("ffmpeg failed");
    }

    let pixels = fs::read(&output).with_context(|| format!("failed to read {:?}", &output))?;

    if pixels.len() != width * height * 2 {
        anyhow::bail!(
            "{} file has wrong size: {}, expected {}",
            output,
            pixels.len(),
            width * height * 2
        );
    }

    Ok(pixels)
}
