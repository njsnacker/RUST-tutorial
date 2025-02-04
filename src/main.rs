use log::{debug, error, info, trace, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};
use std::thread;

mod protocol;
mod serial;

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

fn main() {
    let log_handle = init_logger();
    // change_log_level(&log_handle, LevelFilter::Trace);
    change_log_level(&log_handle, LevelFilter::Debug);

    let mut packet: protocol::PACKET = protocol::PACKET::new();
    let protocol_dummy: [u8; 8] = [0x02, 0xC1, 0x08, 0x12, 0x00, 0x04, 0x78, 0x9F];

    let mut serial: serial::SERIAL = serial::SERIAL::new();
    serial.scan_ports();
    serial.init(&String::from("COM3"), 9_600);

    loop {
        // for v in protocol_dummy {
        //     port0.write(&[v]).expect("Failed to write to port");
        // }
        // port0.write(b"a").expect("Failed to write to port");
        // thread::sleep(Duration::from_millis(1000));

        let d = serial.read();
        trace!("Serial receive : {:02X} ", d);
        let (rslt, p) = packet.parse(d);
        if rslt {
            // trace!("Valid PACKET\r\n{}", p.to_string());
            debug!("Packet Received\r\n{}", p.to_string());
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
