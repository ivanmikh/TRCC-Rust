use std::time::Duration;

use anyhow::{
    Context, Result
};

use rusb::{
    DeviceHandle, UsbContext
};

#[derive(Debug, Clone, Copy)]
struct ProbeRequest {
    magic: u32,
    probe_flag: u32,
}

impl ProbeRequest {
    fn new() -> Self {
        Self {
            magic: 0x7856_3412,
            probe_flag: 1,
        }
    }

    fn to_bytes(self) -> [u8; 64] {
        let mut buf = [0u8; 64];
        buf[0x00..0x04].copy_from_slice(&self.magic.to_le_bytes());
        buf[0x38..0x3C].copy_from_slice(&self.probe_flag.to_le_bytes());
        buf
    }
}

pub fn probe<T: UsbContext>(
    handle: &DeviceHandle<T>,
    ep_in: u8,
    ep_out: u8,
) -> Result<[u8; 64]> {
    let timeout = Duration::from_secs(1);

    handle
        .write_bulk(ep_out, &ProbeRequest::new().to_bytes(), timeout)
        .context("write_buld probe request failed")?;

    let mut resp = [0u8; 64];

    handle
        .read_bulk(ep_in, &mut resp, timeout)
        .context("read_bulk probe response failed")?;

    Ok(resp)
}
