use std::time::Duration;

use serde::ser::SerializeStructVariant;
use serialport::{SerialPortInfo, SerialPortType};

fn scan_serial_ports() -> Vec<String> {
    let debug_ports = true;
    let mut ports_list = Vec::new();

    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                print!("No ports exists");
            } else {
                for port in ports {
                    match port.port_type {
                        SerialPortType::UsbPort(usb_port_info) => {
                            ports_list.push(port.port_name.clone());

                            if debug_ports {
                                println!("포트 이름 : {}", port.port_name);
                                if let Some(manufacturer) = usb_port_info.manufacturer {
                                    println!("제조사: {manufacturer}");
                                }
                                if let Some(product) = usb_port_info.product {
                                    println!("제품명: {product}");
                                }
                                if let Some(serial_number) = usb_port_info.serial_number {
                                    println!("시리얼 번호: {serial_number}\n");
                                }
                            }
                        }
                        SerialPortType::PciPort => {}
                        SerialPortType::BluetoothPort => {}
                        SerialPortType::Unknown => {}
                    }
                }
            }
        }
        Err(e) => {
            // ports_info.push(format!("포트 검색 중 에러 발생: {}", e));
            print!("Error when scanning ports {e}");
        }
    }

    return ports_list;
}

fn main() {
    let mut serial_buf: Vec<u8> = vec![0; 32];

    scan_serial_ports();
}
