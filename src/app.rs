use egui::InnerResponse;
use lipsum::lipsum;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SerialApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for SerialApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
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

            comport_select(ui);
            filter_config(ui);
            send_packet(ui);
            log(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn comport_select(ui: &mut egui::Ui) {
    let mut selected = 0;
    ui.vertical(|ui| {
        ui.heading("COM Port Settings"); // 제목 추가
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Select COM Port :");
                egui::ComboBox::from_id_salt("Select COM Port : ")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, 0, "First");
                        ui.selectable_value(&mut selected, 1, "Second");
                        ui.selectable_value(&mut selected, 2, "Third");
                    });

                ui.label("Baud rate :");
                egui::ComboBox::from_id_salt("Baud rate : ")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, 0, "9600");
                        ui.selectable_value(&mut selected, 1, "115200");
                        ui.selectable_value(&mut selected, 2, "Third");
                    });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.button("Connect");
                });
            });
        });
    });
}

fn filter_config(ui: &mut egui::Ui) {
    let mut id_filter = String::new();
    let mut cmd_filter = String::new();

    ui.vertical(|ui| {
        ui.heading("Filter Configuration"); // 제목 추가
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.vertical(|ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    ui.label("ID :");
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut id_filter),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("CMD :");
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut cmd_filter),
                    );
                });
            });
        });
    });
}

fn unit_1(ui: &mut egui::Ui, label: &str, value: &mut String, expand: bool) -> InnerResponse<()> {
    return ui.vertical(|ui| {
        ui.label(label);
        if expand {
            ui.add_sized(ui.available_size(), egui::TextEdit::singleline(value));
        } else {
            ui.add_sized([30.0, 20.0], egui::TextEdit::singleline(value));
        }
    });
}

fn send_packet(ui: &mut egui::Ui) {
    let mut id = String::new();
    let mut len = String::new();
    let mut cmd = String::new();
    let mut seq = String::new();
    let mut data = String::new();
    let mut cs = String::new();

    let mut delay = String::new();
    let mut count = String::new();

    ui.heading("Send Packet");
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                unit_1(ui, "ID", &mut id, false);
                unit_1(ui, "LEN", &mut len, false);
                unit_1(ui, "CMD", &mut cmd, false);
                unit_1(ui, "SEQ", &mut seq, false);
                // DATA를 확장 가능한 영역으로 설정
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    unit_1(ui, "DATA", &mut data, false);
                    // ui.label("DATA");
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    unit_1(ui, "CS", &mut cs, false);
                });
            });
            ui.horizontal(|ui| {
                ui.label("Delay :");
                ui.text_edit_singleline(&mut delay);
                ui.label("Count :");
                ui.text_edit_singleline(&mut count);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.button("Send");
                });
            });
        });
    });
}

fn log(ui: &mut egui::Ui) {
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
