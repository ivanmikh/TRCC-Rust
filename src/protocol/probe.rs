use anyhow::Result;

use rusb::{
    DeviceHandle, UsbContext
};

use crate::protocol::protocol::compose_probe_request;
use crate::usb::bulk::{send_bulk_out, read_bulk_in};

pub fn probe<T: UsbContext>(
    handle: &DeviceHandle<T>,
    ep_in: u8,
    ep_out: u8,
) -> Result<[u8; 64]> {
    let probe_req = compose_probe_request();

    send_bulk_out(&handle, &probe_req, ep_out)?;

    read_bulk_in(&handle, ep_in)
}
