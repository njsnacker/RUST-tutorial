#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::thread;
use std::time::Duration;

fn main() -> eframe::Result {
    thread::spawn(|| {
        let ports = serialport::available_ports().expect("No ports found!");

        let mut port = serialport::new("/dev/ttyUSB0", 9_600)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open port");

        for p in ports {
            println!("{}", p.port_name)
        }

        loop {
            let mut serial_buf: Vec<u8> = vec![0; 32];
            port.read(serial_buf.as_mut_slice())
                .expect("Found no data!");
            println!(
                "Buf {}",
                serial_buf
                    .iter()
                    .map(|b| format!("{:02x}", b).to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            serial_buf.clear();
        }
        for i in 1..10 {
            println!("num {i} form spawned thread!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Serial Tool",
        native_options,
        Box::new(|cc| Ok(Box::new(RUST_tutorial::SerialApp::new(cc)))),
    )
}
