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
