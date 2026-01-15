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
use crate::framebuffer::compose::{picture_frame, solid_color_frame};

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
    let frame = match config.picture {
        Picture::SolidColor { color } => {
            solid_color_frame(config.resolution.width, config.resolution.height, color)
        }
        Picture::Image { file } => {
            picture_frame(config.resolution.width, config.resolution.height, &file)
        }
        Picture::Video { file, fps } => {
            todo!("Video support not yet implemented: {}, {}", file, fps)
        }
    };

    let fps = 1;
    let period = Duration::from_millis(1000 / fps);

    let mut counter = 0u64;
    loop {
        print!("\rFrames sent = {}", counter);
        io::stdout().flush().unwrap();

        match send_bulk_out(&handle, &frame, bulk_eps.out_ep.get_address()) {
            Ok(_) => counter += 1,
            Err(e) => {
                println!("Failed to send a frame: {}", e);
                break;
            }
        }
        std::thread::sleep(period);
    }

    Ok(())
}
