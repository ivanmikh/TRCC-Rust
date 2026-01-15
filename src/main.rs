use anyhow::Result;
use clap::Parser;
use hexdump::hexdump;
use rusb::Context;
use std::time::Duration;
use std::io::{self, Write};

mod usb;
use crate::usb::open::{open_device, read_device_info, find_bulk_endpoints};
use crate::usb::bulk::send_bulk_out;

mod framebuffer;
use crate::framebuffer::compose::{picture_frame, solid_color_frame, video_frame};
use crate::framebuffer::ffmpeg::VideoFrameReader;

mod config;
use crate::config::{load_config, Picture};

mod protocol;
use crate::protocol::probe::probe;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help="Path to the config file")]
    config: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
 
    println!("# TRCC : Starting");
    println!("# Config :");
    let config = load_config(&args.config)?;

    let mut context = Context::new()?;
    let (mut device, device_desc, mut handle) =
        open_device(&mut context, config.device.vid, config.device.pid)?;

    println!("# Device opened successfully. Desc info:");
    read_device_info(&device_desc, &mut handle)?;

    println!("# Looking for bulk endpoints.");
    let bulk_eps = find_bulk_endpoints(&mut device, &device_desc)?;
    println!("BULK_IN: {:?}", bulk_eps.in_ep);
    println!("BULK_OUT: {:?}", bulk_eps.out_ep);

    println!("# Probe the display");
    match probe(&handle, bulk_eps.in_ep.get_address(), bulk_eps.out_ep.get_address()) {
        Ok(resp) => {
            println!("Response:");
            hexdump(&resp);
        }
        Err(e) => println!("Probe failed: {}", e),
    }

    println!("# Let's send some frames");

    match config.picture {
        Picture::SolidColor { color } => {
            let frame = solid_color_frame(config.resolution.width, config.resolution.height, color);
            send_static_frame(&handle, &frame, bulk_eps.out_ep.get_address(), 1)?;
        }
        Picture::Image { file } => {
            let frame = picture_frame(config.resolution.width, config.resolution.height, &file);
            send_static_frame(&handle, &frame, bulk_eps.out_ep.get_address(), 1)?;
        }
        Picture::Video { file, fps } => {
            stream_video(
                &handle,
                bulk_eps.out_ep.get_address(),
                &file,
                config.resolution.width,
                config.resolution.height,
                fps,
            )?;
        }
    };

    Ok(())
}

fn send_static_frame(
    handle: &rusb::DeviceHandle<rusb::Context>,
    frame: &[u8],
    endpoint: u8,
    fps: u64,
) -> Result<()> {
    let period = Duration::from_millis(1000 / fps);
    let mut counter = 0u64;

    loop {
        print!("\rFrames sent = {}", counter);
        io::stdout().flush().unwrap();

        match send_bulk_out(handle, frame, endpoint) {
            Ok(_) => counter += 1,
            Err(e) => {
                println!("\nFailed to send a frame: {}", e);
                break;
            }
        }
        std::thread::sleep(period);
    }

    Ok(())
}

fn stream_video(
    handle: &rusb::DeviceHandle<rusb::Context>,
    endpoint: u8,
    file: &str,
    width: u16,
    height: u16,
    fps: u32,
) -> Result<()> {
    let mut reader = VideoFrameReader::new(file, width as usize, height as usize, fps)?;
    let period = Duration::from_millis(1000 / fps as u64);
    let mut counter = 0u64;

    loop {
        let pixels = match reader.read_frame()? {
            Some(p) => p,
            None => {
                println!("\nVideo stream ended");
                break;
            }
        };

        let frame = video_frame(width, height, pixels);

        print!("\rFrames sent = {}", counter);
        io::stdout().flush().unwrap();

        match send_bulk_out(handle, &frame, endpoint) {
            Ok(_) => counter += 1,
            Err(e) => {
                println!("\nFailed to send a frame: {}", e);
                break;
            }
        }

        std::thread::sleep(period);
    }

    Ok(())
}
