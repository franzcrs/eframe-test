use egui::Color32;
use egui::FontData;
use egui::FontDefinitions;
use egui::FontFamily;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "untitled folder".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //
        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        let mut system_font_data =
            FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro.ttf"));
        system_font_data.index = 4294967295;
        fonts.font_data.insert(
            "system_font".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                // system_font_data
                FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro.ttf")),
            ),
        );

        // Put my font first (highest priority):
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "system_font".to_owned());

        // Put my font as last fallback for monospace:
        // fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        //     .push("system_font".to_owned());

        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:

        //     egui::menu::bar(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }

        //         egui::widgets::global_theme_preference_buttons(ui);
        //     });
        // });

        // Reference: https://github.com/emilk/egui/discussions/1286
        let my_frame = egui::containers::Frame {
            inner_margin: egui::epaint::Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            outer_margin: egui::epaint::Margin {
                left: 22.,
                right: 22.,
                top: 22.,
                bottom: 22.,
            },
            rounding: egui::Rounding {
                nw: 1.0,
                ne: 1.0,
                sw: 1.0,
                se: 1.0,
            },
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(1.0, Color32::LIGHT_GRAY),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(my_frame)
            .show(ctx, |ui| {
                // egui::CentralPanel::default().show(ctx, |ui| {

                // Reference: ui.rs
                ui.label("New Folder");
                // ui.label(egui::RichText::new("Heavy Text").strong());
                ui.label("Name of new folder inside the current folder");
                ui.add(
                    egui::TextEdit::singleline(&mut self.label)
                        .hint_text("Type something...") // Placeholder text
                        .desired_width(f32::INFINITY) // Make it take full width
                        .font(egui::TextStyle::Monospace), // Use monospace font
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    // ui.label("world!");
                    // ui.label("Hello");
                    // Primary button (Create) - Blue with white text
                    let create_button = egui::Button::new(
                        egui::RichText::new("Create").color(egui::Color32::WHITE),
                    )
                    .fill(egui::Color32::from_rgb(0, 122, 255)) // macOS blue
                    .rounding(egui::Rounding::same(6.0)); // macOS rounded corners

                    if ui.add(create_button).clicked() {
                        // Handle create button click
                        println!("Create clicked with value: {}", self.label);
                    }

                    ui.add_space(8.0); // Space between buttons

                    // Secondary button (Cancel) - Light gray with default text
                    let cancel_button =
                        egui::Button::new("Cancel").rounding(egui::Rounding::same(6.0));

                    if ui.add(cancel_button).clicked() {
                        // Handle cancel button click
                        println!("Cancel clicked");
                        // Optional: reset the input field
                        self.label = "untitled folder".to_owned();
                    }
                });
            });
        //     // The central panel the region left after adding TopPanel's and SidePanel's
        //     ui.heading("eframe template");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(&mut self.label);
        //     });

        //     ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         self.value += 1.0;
        //     }

        //     ui.separator();

        //     ui.add(egui::github_link_file!(
        //         "https://github.com/emilk/eframe_template/blob/main/",
        //         "Source code."
        //     ));

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         powered_by_egui_and_eframe(ui);
        //         egui::warn_if_debug_build(ui);
        //     });
        // });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // _visuals.window_fill().to_normalized_gamma_f32()
        // egui::Color32::from_gray(27).to_normalized_gamma_f32()
        egui::Color32::from_rgb(34, 32, 29).to_normalized_gamma_f32()
    }
}

// fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//     ui.horizontal(|ui| {
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
