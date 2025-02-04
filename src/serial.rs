use std::time::Duration;

use log::{debug, error, info, trace, warn, LevelFilter};
use serialport::{SerialPortInfo, SerialPortType};

pub struct SERIAL {
    pub port_name: String,
    pub baud_rate: u32,
    pub buf: [u8; 1],
    pub port: Option<Box<dyn serialport::SerialPort>>,
}

impl SERIAL {
    pub fn new() -> SERIAL {
        SERIAL {
            port_name: String::new(),
            baud_rate: 0,
            buf: [0; 1],
            port: None,
        }
    }

    pub fn init(&mut self, port_name: &String, baud_rate: u32) {
        let mut serial_port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open port");

        self.port_name = port_name.clone();
        self.baud_rate = baud_rate;
        self.buf = [0; 1];
        self.port = Some(serial_port);
    }

    pub fn print_port_detail(&mut self, port_name: &String, usb_port: &serialport::UsbPortInfo) {
        debug!("Port name : {}", port_name);

        if let Some(manufacturer) = &usb_port.manufacturer {
            debug!("MFR : {}", manufacturer);
        }
        if let Some(product) = &usb_port.product {
            debug!("Product : {}", product);
        }
        if let Some(serial_number) = &usb_port.serial_number {
            debug!("Serial : {}", serial_number);
        }
    }

    pub fn scan_ports(&mut self) -> Vec<String> {
        let mut port_name_list = Vec::new();

        match serialport::available_ports() {
            Ok(ports) => {
                if ports.is_empty() {
                    debug!("No ports exists");
                } else {
                    for (idx, port) in ports.iter().enumerate() {
                        match port.clone().port_type {
                            SerialPortType::UsbPort(usb_port_info) => {
                                port_name_list.push(port.port_name.clone());
                                debug!("Port num : {}", idx);
                                self.print_port_detail(&port.port_name, &usb_port_info);
                                debug!("")
                            }
                            SerialPortType::PciPort => {}
                            SerialPortType::BluetoothPort => {}
                            SerialPortType::Unknown => {}
                        }
                    }
                }
            }
            Err(e) => {
                print!("Error when scanning ports {e}");
            }
        }

        return port_name_list;
    }

    pub fn read(&mut self) -> u8 {
        if let Some(ref mut port) = self.port {
            port.read(&mut self.buf).unwrap();
            return self.buf[0];
        } else {
            panic!("Serial port not initialized");
        }

        /* OLD CODE
        match serial_port.read(&mut serial_buf) {
            Ok(t) => {
                for byte in &serial_buf[..t] {
                    trace!("Serial receive : {:02X} ", byte);
                    let (rslt, p) = packet.parse(*byte);
                    if rslt {
                        // trace!("Valid PACKET\r\n{}", p.to_string());
                        debug!("Packet Received\r\n{}", p.to_string());
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
         */
    }

    // pub fn write(&mut self, data: u8) {
    //     self.port.write(&[data]).unwrap();
    // }
}
