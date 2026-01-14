use std::time::Duration;
use anyhow::anyhow;

use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, Result, TransferType, UsbContext,
};

#[derive(Debug)]
pub struct Endpoint {
    address: u8,
    config: u8,
    iface: u8,
    setting: u8,
}

impl Endpoint {
    pub fn get_address(&self) -> u8 {
        self.address
    }
}

#[derive(Debug)]
pub struct BulkEndpoints {
    pub in_ep: Endpoint,
    pub out_ep: Endpoint,
}

pub fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(e) => panic!("Device found but failed to open: {}", e),
            }
        }
    }

    None
}

pub fn read_device_info<T: UsbContext>(
    device_desc: &DeviceDescriptor,
    handle: &mut DeviceHandle<T>,
) -> Result<()> {
    handle.reset()?;

    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);
    println!("Number of configurations: {}", device_desc.num_configurations());

    if !languages.is_empty() {
        let language = languages[0];

        println!(
            "Manufacturer: {:?}",
            handle
                .read_manufacturer_string(language, device_desc, timeout)
                .ok()
                .unwrap()
        );
        println!(
            "Product: {:?}",
            handle
                .read_product_string(language, device_desc, timeout)
                .ok()
                .unwrap()
        );
        println!(
            "Serial Number: {:?}",
            handle
                .read_serial_number_string(language, device_desc, timeout)
                .ok()
                .unwrap()
        );
    }

    Ok(())
}

pub fn find_bulk_endpoints<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: &DeviceDescriptor,
) -> Option<BulkEndpoints> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                let mut bulk_in: Option<Endpoint> = None;
                let mut bulk_out: Option<Endpoint> = None;

                for ep in interface_desc.endpoint_descriptors() {
                    if ep.transfer_type() != TransferType::Bulk {
                        continue;
                    }
                    let endpoint = Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: ep.address(),
                    };

                    match ep.direction() {
                        Direction::In => bulk_in = Some(endpoint),
                        Direction::Out => bulk_out = Some(endpoint),
                    }
                }

                // Check if both endpoints are filled
                if let (Some(in_ep), Some(out_ep)) = (bulk_in, bulk_out) {
                    return Some(BulkEndpoints { in_ep, out_ep });
                }
            }
        }
    }

    None
}
