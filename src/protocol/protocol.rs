// Packet fields and offsets
const MAGIC: u32 = 0x7856_3412;
const MAGIC_OFFSET_START: usize = 0x00;
const MAGIC_OFFSET_END: usize = 0x04;

const PROBE_FLAG: u32 = 1;
const PROBE_FLAG_OFFSET_START: usize = 0x38;
const PROBE_FLAG_OFFSET_END: usize = 0x3C;

const WRITE_FRAME_CMD: u32 = 3;
const CMD_OFFSET_START: usize = 0x04;
const CMD_OFFSET_END: usize = 0x08;

const WIDTH_OFFSET_START: usize = 0x08;
const WIDTH_OFFSET_END: usize = 0x0c;

const HEIGHT_OFFSET_START: usize = 0xc;
const HEIGHT_OFFSET_END: usize = 0x10;

const BYTES_PER_PIXEL_OFFSET_START: usize = 0x38;
const BYTES_PER_PIXEL_OFFSET_END: usize = 0x3C;

const PAYLOAD_LENGTH_OFFSET_START: usize = 0x3C;
const PAYLOAD_LENGTH_OFFSET_END: usize = 0x40;

const HEADER_SIZE: usize = 64;

pub fn compose_probe_request() -> Vec<u8> {
    let mut pkt = vec![0u8; HEADER_SIZE];

    pkt[MAGIC_OFFSET_START..MAGIC_OFFSET_END].copy_from_slice(&MAGIC.to_le_bytes());
    pkt[PROBE_FLAG_OFFSET_START..PROBE_FLAG_OFFSET_END].copy_from_slice(&PROBE_FLAG.to_le_bytes());

    pkt
}

pub fn compose_frame_pkt(
    width: u16,
    height: u16,
    bytes_per_pixel: u16,
    frame_bytes: Vec<u8>
) -> Vec<u8> {
    let mut pkt = vec![0u8; HEADER_SIZE];

    let w = width as u32;
    let h = height as u32;
    let bpp = bytes_per_pixel as u32;
    let payload_len = w * h * bpp;

    pkt[MAGIC_OFFSET_START..MAGIC_OFFSET_END].copy_from_slice(&MAGIC.to_le_bytes());
    pkt[CMD_OFFSET_START..CMD_OFFSET_END].copy_from_slice(&WRITE_FRAME_CMD.to_le_bytes());
    pkt[WIDTH_OFFSET_START..WIDTH_OFFSET_END].copy_from_slice(&w.to_le_bytes());
    pkt[HEIGHT_OFFSET_START..HEIGHT_OFFSET_END].copy_from_slice(&h.to_le_bytes());
    pkt[BYTES_PER_PIXEL_OFFSET_START..BYTES_PER_PIXEL_OFFSET_END].copy_from_slice(&bpp.to_le_bytes());
    pkt[PAYLOAD_LENGTH_OFFSET_START..PAYLOAD_LENGTH_OFFSET_END].copy_from_slice(&payload_len.to_le_bytes());

    pkt.extend_from_slice(&frame_bytes);

    pkt
}
