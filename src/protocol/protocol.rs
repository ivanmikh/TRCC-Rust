use bytemuck::{Pod, Zeroable};

const MAGIC: u32 = 0x7856_3412;
const PROBE: u32 = 1;
const WRITE: u32 = 3;

/// Packet header for the ThermalRight display protocol.
/// 64 bytes total, little-endian fields.
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C, packed)]
pub struct PacketHeader {
    pub magic: u32,           // 0x00: Magic number (0x12345678)
    pub command: u32,         // 0x04: Command (1=probe, 3=write_frame)
    pub width: u32,           // 0x08: Display width
    pub height: u32,          // 0x0C: Display height
    pub _reserved1: [u8; 32], // 0x10-0x2F: Reserved/padding
    pub _reserved2: [u8; 8],  // 0x30-0x37: Reserved/padding
    pub bytes_per_pixel: u32, // 0x38: Bytes per pixel or probe flag
    pub payload_length: u32,  // 0x3C: Payload length in bytes
}

// Compile-time assertion that header is exactly 64 bytes
const _: () = assert!(std::mem::size_of::<PacketHeader>() == 64);

impl PacketHeader {
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

pub fn compose_probe_request() -> Vec<u8> {
    let header = PacketHeader {
        magic: MAGIC.to_le(),
        command: 0,
        width: 0,
        height: 0,
        _reserved1: [0u8; 32],
        _reserved2: [0u8; 8],
        bytes_per_pixel: PROBE.to_le(),
        payload_length: 0,
    };
    header.as_bytes().to_vec()
}

pub fn compose_frame_pkt(
    width: u16,
    height: u16,
    bytes_per_pixel: u16,
    frame_bytes: Vec<u8>,
) -> Vec<u8> {
    let w = width as u32;
    let h = height as u32;
    let bpp = bytes_per_pixel as u32;
    let payload_len = w * h * bpp;

    let header = PacketHeader {
        magic: MAGIC.to_le(),
        command: WRITE.to_le(),
        width: w.to_le(),
        height: h.to_le(),
        _reserved1: [0u8; 32],
        _reserved2: [0u8; 8],
        bytes_per_pixel: bpp.to_le(),
        payload_length: payload_len.to_le(),
    };

    let mut pkt = header.as_bytes().to_vec();
    pkt.extend_from_slice(&frame_bytes);
    pkt
}
