use serde::ser::SerializeStructVariant;
use serialport::{SerialPortInfo, SerialPortType};
use std::thread;
use std::time::Duration;

mod protocol;

fn print_usb_serial_port(port_name: &String, usb_port: &serialport::UsbPortInfo) {
    println!("포트 이름: {}", port_name);

    if let Some(manufacturer) = &usb_port.manufacturer {
        println!("제조사: {}", manufacturer);
    }
    if let Some(product) = &usb_port.product {
        println!("제품명: {}", product);
    }
    if let Some(serial_number) = &usb_port.serial_number {
        println!("시리얼 번호: {}", serial_number);
    }
}

fn scan_serial_ports() -> Vec<String> {
    let debug_ports: bool = true;
    let mut ports_name_list = Vec::new();

    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                print!("No ports exists");
            } else {
                for (idx, port) in ports.iter().enumerate() {
                    match port.clone().port_type {
                        SerialPortType::UsbPort(usb_port_info) => {
                            ports_name_list.push(port.port_name.clone());
                            if debug_ports {
                                println!("포트 번호: {}", idx);
                                print_usb_serial_port(&port.port_name, &usb_port_info);
                                println!()
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
    let mut packet: protocol::PACKET = protocol::PACKET::new();

    let mut serial_buf: [u8; 1] = [0; 1];
    let protocol_dummy: [u8; 8] = [0x02, 0xC1, 0x08, 0x12, 0x00, 0x04, 0x78, 0x9F];
    let mut target_port_name = String::from("COM3");

    let port_names = scan_serial_ports();

    println!("Port names : {:?}", port_names);

    // FOR DEBUG
    let mut port0 = serialport::new(&target_port_name, 9_600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    loop {
        // for v in protocol_dummy {
        //     port0.write(&[v]).expect("Failed to write to port");
        // }
        // port0.write(b"a").expect("Failed to write to port");
        // thread::sleep(Duration::from_millis(1000));
        match port0.read(&mut serial_buf) {
            Ok(t) => {
                // println!("READ : {:?}", &serial_buf[..t]);
                for byte in &serial_buf[..t] {
                    println!("{:02X} ", byte);
                }
                let d: u8 = serial_buf[0];
                packet.parse(d);
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
