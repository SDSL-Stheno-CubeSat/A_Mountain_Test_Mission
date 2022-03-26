extern crate libusb;

use std::slice;
use std::time::Duration;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8
}

// Use "lsusb" in terminal for usb info

fn main() {
    // vid: 32e4 (13028), pid: 9520 (38176)
    let vid: u16 = 13028;
    let pid: u16 = 38176;

    let context = libusb::Context::new().unwrap();

    for mut device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            println!("Open device ID {:04x}:{:04x}", vid,pid);
            match device.open() {
                Ok(mut handle) => read_device(&mut device, &device_desc, &mut handle).unwrap(),
                Err(err) => println!("{}", err),
            }
        }
    }
}


fn read_device(device: &mut libusb::Device, device_desc: &libusb::DeviceDescriptor, handle: &mut libusb::DeviceHandle) -> libusb::Result<()> {
    handle.reset().unwrap();

    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout).unwrap();

    println!("Active configuration: {}", handle.active_configuration().unwrap());
    println!("Languages: {:?}", languages);

    if languages.len() > 0 {
        let language = languages[0];

        println!("Manufacturer: {:?}", handle.read_manufacturer_string(language, device_desc, timeout).ok());
        println!("Product: {:?}", handle.read_product_string(language, device_desc, timeout).ok());
        println!("Serial Number: {:?}", handle.read_serial_number_string(language, device_desc, timeout).ok());
    }

    match find_readable_endpoint(device, device_desc, libusb::TransferType::Interrupt) {
        Some(endpoint) => read_endpoint(handle, endpoint, libusb::TransferType::Interrupt),
        None => println!("No readable interrupt endpoint")
    }

    match find_readable_endpoint(device, device_desc, libusb::TransferType::Bulk) {
        Some(endpoint) => read_endpoint(handle, endpoint, libusb::TransferType::Bulk),
        None => println!("No readable bulk endpoint")
    }

    Ok(())
}

fn find_readable_endpoint(device: &mut libusb::Device, device_desc: &libusb::DeviceDescriptor, transfer_type: libusb::TransferType) -> Option<Endpoint> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue
        };

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == libusb::Direction::In && endpoint_desc.transfer_type() == transfer_type {
                        return Some(Endpoint {
                            config: config_desc.number(),
                            iface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                            address: endpoint_desc.address()
                        });
                    }
                }
            }
        }
    }

    None
}

fn read_endpoint(handle: &mut libusb::DeviceHandle, endpoint: Endpoint, transfer_type: libusb::TransferType) {
    println!("Reading from endpoint: {:?}", endpoint);

    let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(endpoint.iface).ok();
            true
        },
        _ => false
    };

    println!(" - kernel driver? {}", has_kernel_driver);

    match configure_endpoint(handle, &endpoint) {
        Ok(_) => {
            let mut vec = Vec::<u8>::with_capacity(256);
            let buf = unsafe { slice::from_raw_parts_mut((&mut vec[..]).as_mut_ptr(), vec.capacity()) };

            let timeout = Duration::from_secs(1);

            match transfer_type {
                libusb::TransferType::Interrupt => {
                    match handle.read_interrupt(endpoint.address, buf, timeout) {
                        Ok(len) => {
                            unsafe { vec.set_len(len) };
                            println!(" - read: {:?}", vec);
                        },
                        Err(err) => println!("could not read from endpoint: {}", err)
                    }
                },
                libusb::TransferType::Bulk => {
                    match handle.read_bulk(endpoint.address, buf, timeout) {
                        Ok(len) => {
                            unsafe { vec.set_len(len) };
                            println!(" - read: {:?}", vec);
                        },
                        Err(err) => println!("could not read from endpoint: {}", err)
                    }
                },
                _ => ()
            }
        },
        Err(err) => println!("could not configure endpoint: {}", err)
    }

    if has_kernel_driver {
        handle.attach_kernel_driver(endpoint.iface).ok();
    }
}

fn configure_endpoint<'a>(handle: &'a mut libusb::DeviceHandle, endpoint: &Endpoint) -> libusb::Result<()> {
    handle.set_active_configuration(endpoint.config).unwrap();
    handle.claim_interface(endpoint.iface).unwrap();
    handle.set_alternate_setting(endpoint.iface, endpoint.setting).unwrap();
    Ok(())
}