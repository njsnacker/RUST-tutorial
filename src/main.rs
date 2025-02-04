use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};
use serde::ser::SerializeStructVariant;
use serialport::{SerialPortInfo, SerialPortType};
use std::thread;
use std::time::Duration;

mod protocol;

// const LOG_PATTERN: &str = "[{d} {l}] {m}{n}";
const LOG_PATTERN: &str = "[{d(%Y-%m-%d %H:%M:%S%.3f)} {l}] {m}{n}";
const LOG_FILE: &str = "log.txt";

fn init_logger() -> Handle {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build();

    let file: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
        .build(LOG_FILE)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appenders(["stdout", "file"])
                .build(LevelFilter::Info),
        )
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();
    handle
}

fn change_log_level(handle: &Handle, new_level: LevelFilter) {
    let new_config = Config::builder()
        .appender(
            Appender::builder().build(
                "stdout",
                Box::new(
                    ConsoleAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
                        .build(),
                ),
            ),
        )
        .appender(
            Appender::builder().build(
                "file",
                Box::new(
                    FileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
                        .build(LOG_FILE)
                        .unwrap(),
                ),
            ),
        )
        .build(
            Root::builder()
                .appenders(["stdout", "file"])
                .build(new_level),
        )
        .unwrap();

    handle.set_config(new_config);
}

fn print_usb_serial_port(port_name: &String, usb_port: &serialport::UsbPortInfo) {
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

fn scan_serial_ports() -> Vec<String> {
    let debug_ports: bool = true;
    let mut ports_name_list = Vec::new();

    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                debug!("No ports exists");
            } else {
                for (idx, port) in ports.iter().enumerate() {
                    match port.clone().port_type {
                        SerialPortType::UsbPort(usb_port_info) => {
                            ports_name_list.push(port.port_name.clone());
                            if debug_ports {
                                debug!("Port num : {}", idx);
                                print_usb_serial_port(&port.port_name, &usb_port_info);
                                debug!("")
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

    return ports_name_list;
}

fn main() {
    let log_handle = init_logger();
    change_log_level(&log_handle, LevelFilter::Trace);
    change_log_level(&log_handle, LevelFilter::Debug);

    let mut packet: protocol::PACKET = protocol::PACKET::new();

    let mut serial_buf: [u8; 1] = [0; 1];
    let protocol_dummy: [u8; 8] = [0x02, 0xC1, 0x08, 0x12, 0x00, 0x04, 0x78, 0x9F];
    let mut target_port_name = String::from("COM3");

    let port_names = scan_serial_ports();

    debug!("Port names : {:?}", port_names);

    // FOR DEBUG
    let mut serial_port = serialport::new(&target_port_name, 9_600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    loop {
        // for v in protocol_dummy {
        //     port0.write(&[v]).expect("Failed to write to port");
        // }
        // port0.write(b"a").expect("Failed to write to port");
        // thread::sleep(Duration::from_millis(1000));

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
    }
}

// fn main() -> eframe::Result {
//     env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

//     let native_options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default()
//             .with_inner_size([400.0, 300.0])
//             .with_min_inner_size([300.0, 220.0])
//             .with_icon(
//                 // NOTE: Adding an icon is optional
//                 eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
//                     .expect("Failed to load icon"),
//             ),
//         ..Default::default()
//     };
//     eframe::run_native(
//         "Serial Tool",
//         native_options,
//         Box::new(|cc| Ok(Box::new(RUST_tutorial::SerialApp::new(cc)))),
//     )
// }
