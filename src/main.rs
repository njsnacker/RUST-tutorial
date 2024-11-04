#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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

fn main() {
    let s1 = gv_ownship();

    let s2 = String::from("da?");
    let s3 = tk_and_gv_back(s2);

    println!("{s1}");
    // println!("{s2}");
    println!("{s3}");

    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    let r3 = &mut s; // 문제없음
    println!("{}", r3);
}

fn gv_ownship() -> String {
    let str = String::from("hi?");

    str
}

fn tk_and_gv_back(a_string: String) -> String {
    a_string
}
