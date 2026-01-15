use std::env;
use std::fs;
use std::io::Read;
use std::process::{Child, Command, Stdio};

use anyhow::{Context, Result};

/// Reads video frames from FFmpeg subprocess as raw RGB565 pixels.
pub struct VideoFrameReader {
    child: Child,
    frame_size: usize,
}

impl VideoFrameReader {
    /// Spawns FFmpeg to decode a video file and stream RGB565 frames.
    /// The video loops infinitely by default.
    /// Automatically center-crops to match display aspect ratio before scaling.
    pub fn new(filepath: &str, width: usize, height: usize, fps: u32) -> Result<Self> {
        // Center crop to target aspect ratio, then scale
        // If source is wider than target: crop width, keep height
        // If source is taller than target: crop height, keep width
        let crop = format!(
            "crop='if(gt(iw/ih,{w}/{h}),ih*{w}/{h},iw)':'if(gt(iw/ih,{w}/{h}),ih,iw*{h}/{w})'",
            w = width,
            h = height
        );
        let filter = format!("{},scale={}:{}:flags=lanczos,fps={}", crop, width, height, fps);

        let args = [
            "-stream_loop", "-1",  // Loop video infinitely
            "-i", filepath,
            "-vf", &filter,
            "-pix_fmt", "rgb565be",
            "-f", "rawvideo",
            "-",  // Output to stdout
        ];

        println!("ffmpeg {}", args.join(" "));

        let child = Command::new("ffmpeg")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to spawn ffmpeg")?;

        Ok(Self {
            child,
            frame_size: width * height * 2,
        })
    }

    /// Reads the next frame from the video stream.
    /// Returns None if the stream has ended.
    pub fn read_frame(&mut self) -> Result<Option<Vec<u8>>> {
        let mut buf = vec![0u8; self.frame_size];
        let stdout = self.child.stdout.as_mut().context("ffmpeg stdout not available")?;

        match stdout.read_exact(&mut buf) {
            Ok(()) => Ok(Some(buf)),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => Err(e).context("failed to read frame from ffmpeg"),
        }
    }
}

impl Drop for VideoFrameReader {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

pub fn ffmpeg_convert_image_to_rg565ble(
    filepath: &str,
    width: usize,
    height: usize,
) -> Result<Vec<u8>> {
    let output = env::temp_dir().join("trcc_frame.rgb565");
    let output_str = output.to_string_lossy();

    // Center crop to target aspect ratio, then scale
    let crop = format!(
        "crop='if(gt(iw/ih,{w}/{h}),ih*{w}/{h},iw)':'if(gt(iw/ih,{w}/{h}),ih,iw*{h}/{w})'",
        w = width,
        h = height
    );
    let scale = format!("{},scale={}:{}:flags=lanczos", crop, width, height);
    let args = [
        "-i", filepath,
        "-vf", &scale,
        "-frames:v", "1",
        "-pix_fmt", "rgb565be",
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
