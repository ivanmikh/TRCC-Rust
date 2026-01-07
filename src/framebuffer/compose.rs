const WIDTH: usize = 320;
const HEIGHT: usize = 320;
const FRAME_SIZE: usize = WIDTH * HEIGHT * 2;
const HEADER_SIZE: usize = 64;

use crate::framebuffer::ffmpeg::ffmpeg_convert_image_to_rg565ble;

fn solid_color_rgb565(color: u16) -> Vec<u8> {
    let lo = (color & 0x00FF) as u8;
    let hi = ((color & 0xFF00) >> 8) as u8;

    let mut buf = vec![0u8; FRAME_SIZE];
    for px in buf.chunks_exact_mut(2) {
        px[0] = lo;
        px[1] = hi;
    }

    buf
}

fn frame_header() -> [u8; HEADER_SIZE] {
    let mut h = [0u8; HEADER_SIZE];

    // magic
    h[0..4].copy_from_slice(&0x7856_3412u32.to_le_bytes());

    // cmd = 3 (write frame)
    h[4..8].copy_from_slice(&3u32.to_le_bytes());

    // width / height
    h[8..12].copy_from_slice(&(WIDTH as u32).to_le_bytes());
    h[12..16].copy_from_slice(&(HEIGHT as u32).to_le_bytes());

    // bytes per pixel
    h[0x38..0x3C].copy_from_slice(&2u32.to_le_bytes());

    // payload length
    h[0x3C..0x40].copy_from_slice(&(FRAME_SIZE as u32).to_le_bytes());

    h
}

pub fn solid_color_frame(color: u16) -> Vec<u8> {
    let header = frame_header();
    let pixels = solid_color_rgb565(color);

    let mut frame = Vec::with_capacity(header.len() + pixels.len());
    frame.extend_from_slice(&header);
    frame.extend_from_slice(&pixels);

    frame
}

pub fn picture_frame(path: &str) -> Vec<u8> {
    let header = frame_header();
    let pixels = ffmpeg_convert_image_to_rg565ble(path, WIDTH, HEIGHT).unwrap();

    let mut frame = Vec::with_capacity(header.len() + pixels.len());
    frame.extend_from_slice(&header);
    frame.extend_from_slice(&pixels);

    frame
}
