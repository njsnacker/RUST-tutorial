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

// fn main() {
//     let s1 = gv_ownship();

//     let s2 = String::from("da?");
//     let s3 = tk_and_gv_back(s2);

//     println!("{s1}");
//     // println!("{s2}");
//     println!("{s3}");

//     let mut s = String::from("hello");

//     let r1 = &s; // 문제없음
//     let r2 = &s; // 문제없음
//     println!("{} and {}", r1, r2);
//     // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

//     let r3 = &mut s; // 문제없음
//     println!("{}", r3);
// }

// fn gv_ownship() -> String {
//     let str = String::from("hi?");

//     str
// }

// fn tk_and_gv_back(a_string: String) -> String {
//     a_string
// }
