use std::time::Duration;

use anyhow::{
    Context, Result
};

use rusb::{
    DeviceHandle, UsbContext
};

pub fn send_bulk_out<T: UsbContext>(
    handle: &DeviceHandle<T>,
    buf: &Vec<u8>,
    ep_out: u8,
) -> Result<()> {
    let timeout = Duration::from_millis(500);
    let written = handle.write_bulk(ep_out, &buf, timeout)?;

    if written != buf.len() {
        anyhow::bail!("short USB write: {} / {}", written, buf.len());
    }

    Ok(())
}

pub fn read_bulk_in<T: UsbContext>(
    handle: &DeviceHandle<T>,
    ep_in: u8,
) -> Result<[u8; 64]> {
    let timeout = Duration::from_secs(1);

    let mut resp = [0u8; 64];

    handle
        .read_bulk(ep_in, &mut resp, timeout)
        .context("read_bulk probe response failed")?;

    Ok(resp)
}
