use crate::framebuffer::ffmpeg::ffmpeg_convert_image_to_rg565ble;
use crate::protocol::protocol::compose_frame_pkt;

fn solid_color_rgb565(
    color: u16,
    frame_size: usize
) -> Vec<u8> {
    let lo = (color & 0x00FF) as u8;
    let hi = ((color & 0xFF00) >> 8) as u8;

    let mut buf = vec![0u8; frame_size];
    for px in buf.chunks_exact_mut(2) {
        px[0] = lo;
        px[1] = hi;
    }

    buf
}

pub fn solid_color_frame(
    color: u16,
    width: u16,
    height: u16,
) -> Vec<u8> {
    let frame_size = (width * height) as usize;

    let pixels = solid_color_rgb565(color, frame_size);
    let bytes_per_pixel: u16 = 2;

    compose_frame_pkt(width, height, bytes_per_pixel, pixels)
}

pub fn picture_frame(
    width: u16,
    height: u16,
    path: &str
) -> Vec<u8> {
    let pixels = ffmpeg_convert_image_to_rg565ble(path, width as usize, height as usize).unwrap();
    let bytes_per_pixel: u16 = 2;

    compose_frame_pkt(width, height, bytes_per_pixel, pixels)
}
