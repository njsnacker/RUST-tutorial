use crate::protocol::PACKET;
use crate::serial::BaudRate;
use crate::serial::ComPort;
use eframe::Frame;
use egui::emath::align;
use egui::frame;
use egui::vec2;
use egui::Widget;
use egui::{Align, Button, Color32, InnerResponse, Layout};
use lipsum::lipsum;
use strum::IntoEnumIterator;

pub const WIDNOW_X_MIN: f32 = 800.0;
pub const WIDNOW_Y_MIN: f32 = 600.0;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SerialApp {
    // Example stuff:
    baud_rate: BaudRate,
    com_port: ComPort,

    id_filter: String,
    cmd_filter: String,

    send_delay: u32,
    send_count: u32,

    #[serde(skip)]
    packet: PACKET,
}

impl Default for SerialApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            baud_rate: BaudRate::B9600,
            com_port: ComPort::COM1,
            id_filter: String::new(),
            cmd_filter: String::new(),
            send_delay: 100,
            send_count: 1,
            packet: PACKET::new(),
        }
    }
}

impl SerialApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    // COM Port 연결 설정 섹션
    fn section_comport_select(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Select COM Port :");
                egui::ComboBox::from_id_salt("Select COM Port : ")
                    .selected_text(format!("{:?}", self.com_port))
                    .show_ui(ui, |ui| {
                        for com_port in ComPort::iter() {
                            ui.selectable_value(
                                &mut self.com_port,
                                com_port,
                                format!("{:?}", com_port),
                            );
                        }
                    });

                ui.label("Baud rate :");
                egui::ComboBox::from_id_salt("Baud rate : ")
                    .selected_text(self.baud_rate.to_string())
                    .show_ui(ui, |ui| {
                        for baud_rate in BaudRate::iter() {
                            ui.selectable_value(
                                &mut self.baud_rate,
                                baud_rate,
                                baud_rate.to_string(),
                            );
                        }
                    });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.button("Connect");
                });
            });
        });
    }

    // ID 및 CMD 필터 설정 섹션
    fn section_filter_config(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            // let width = ui.available_width(); // 사용 가능한 전체 너비 가져오기
            // ui.set_min_width(width); // Frame의 최소 너비를 설정

            egui::CollapsingHeader::new("Filter Configuration")
                .default_open(false)
                .show(ui, |ui| {
                    ui.vertical(|ui: &mut egui::Ui| {
                        ui.horizontal(|ui| {
                            ui.label("ID      : ");
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(&mut self.id_filter),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("CMD : ");
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(&mut self.cmd_filter),
                            );
                        });
                    });
                });
        });
    }

    fn unit_1(
        &self,
        ui: &mut egui::Ui,
        label: &str,
        value: &mut String,
        align_center: bool,
        size: f32,
    ) -> InnerResponse<()> {
        return ui.allocate_ui(vec2(size, 50.0), |ui| {
            ui.with_layout(
                egui::Layout::from_main_dir_and_cross_align(
                    egui::Direction::TopDown,
                    if align_center {
                        Align::Center
                    } else {
                        Align::Min
                    },
                )
                .with_cross_justify(true)
                .with_main_wrap(true),
                |ui| {
                    // ui.painter().rect_filled(
                    //     ui.available_rect_before_wrap(),
                    //     0.0,
                    //     Color32::from_rgba_unmultiplied(255, 0, 0, 128), // Example of a faded red color
                    // );
                    ui.label(label);
                    ui.add(
                        egui::TextEdit::singleline(value).horizontal_align(if align_center {
                            Align::Center
                        } else {
                            Align::Min
                        }),
                    );
                },
            );
        });
    }

    // 패킷 전송 섹션
    fn section_send_packet(&self, ui: &mut egui::Ui) {
        let mut data = String::new();
        let mut delay = String::new();
        let mut count = String::new();

        egui::Frame::group(ui.style()).show(ui, |ui| {
            egui::CollapsingHeader::new("Packet Send")
                .default_open(false)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // ui.painter().rect_filled(
                        //     ui.available_rect_before_wrap(),
                        //     0.0,
                        //     Color32::from_rgba_unmultiplied(0, 255, 0, 128), // Example of a faded red color
                        // );
                        ui.horizontal(|ui| {
                            self.unit_1(
                                ui,
                                "STX",
                                &mut format!("{:02X}", self.packet.header.stx),
                                true,
                                40.0,
                            );
                            self.unit_1(
                                ui,
                                "ID",
                                &mut format!("{:02X}", self.packet.header.id),
                                true,
                                40.0,
                            );
                            self.unit_1(
                                ui,
                                "LEN",
                                &mut format!("{:02X}", self.packet.header.length),
                                true,
                                40.0,
                            );
                            self.unit_1(
                                ui,
                                "CMD",
                                &mut &mut format!("{:02X}", self.packet.header.command),
                                true,
                                40.0,
                            );
                            self.unit_1(
                                ui,
                                "SEQ",
                                &mut &mut format!("{:02X}", self.packet.header.sequence),
                                true,
                                40.0,
                            );

                            self.unit_1(
                                ui,
                                "DATA",
                                &mut data,
                                false,
                                ui.available_width() - (50.0),
                            );

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                self.unit_1(
                                    ui,
                                    "CS",
                                    &mut &mut format!("{:02X}", self.packet.checksum),
                                    true,
                                    40.0,
                                );
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Delay :");
                            ui.add_sized(
                                [80.0, 20.0],
                                egui::TextEdit::singleline(&mut delay)
                                    .horizontal_align(Align::Center),
                            );
                            ui.label("Count :");
                            ui.add_sized(
                                [80.0, 20.0],
                                egui::TextEdit::singleline(&mut count)
                                    .horizontal_align(Align::Center),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.add_sized([40.0, 20.0], Button::new("Send"));
                                },
                            );
                        });
                    });
                });
        });
    }

    // 로그 출력 섹션
    fn log(&self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            let width: f32 = ui.available_width(); // 사용 가능한 전체 너비 가져오기
            ui.set_min_width(width); // Frame의 최소 너비를 설정
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::default())
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                        |ui| {
                            ui.label(lipsum(1000));
                        },
                    );
                });
        });
    }
}

impl eframe::App for SerialApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // 패딩 설정
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0); // 위젯 사이의 간격
        ctx.set_style(style);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Save log as").clicked() {}
                });
                ui.menu_button("View", |ui| if ui.button("Clear log").clicked() {});
                ui.menu_button("Option", |ui| if ui.button("Bit checker").clicked() {});
                ui.menu_button("Help", |ui| if ui.button("About").clicked() {});
                // egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::Frame::default()
                .inner_margin(egui::vec2(2.0, 2.0))
                .show(ui, |ui| {
                    self.section_comport_select(ui);
                    self.section_filter_config(ui);
                    self.section_send_packet(ui);
                    self.log(ui);
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

// fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//     ui.horizontal(|ui: &mut egui::Ui| {
//         ui.spacing_mut().item_spacing.x = 0.0;
//         ui.label("Powered by ");
//         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//         ui.label(" and ");
//         ui.hyperlink_to(
//             "eframe",
//             "https://github.com/emilk/egui/tree/master/crates/eframe",
//         );
//         ui.label(".");
//     });
// }
