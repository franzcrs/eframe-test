use egui::Color32;
use egui::FontData;
use egui::FontDefinitions;
use egui::Vec2;
use egui::{FontFamily, FontId};
use std::collections::BTreeMap;

use egui::TextStyle::*;

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
        // Customize the fonts used by egui
        // Reference: https://docs.rs/egui/latest/egui/struct.FontDefinitions.html

        // Call the default Font Definitions
        let mut fonts = FontDefinitions::default();

        // Install my own fonts

        // System Text Heavy
        // 1st option
        fonts.font_data.insert(
            "system-text-heavy-1".to_owned(),
            std::sync::Arc::new(
                // system font data (.ttf and .otf supported)
                // FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro-Display-Bold.otf")),
                // FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro-Display-Heavy.otf")),
                // FontData::from_static(include_bytes!("../assets/HelveticaNeueLTPro85Heavy.otf")),
                // 1st option
                FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro-Text-Heavy.otf")),
            ),
        );
        // 2nd option
        fonts.font_data.insert(
            "system-text-heavy-2".to_owned(),
            std::sync::Arc::new(
                // 2nd option
                FontData::from_static(include_bytes!("../assets/HelveticaNeueHeavy.otf")),
            ),
        );

        // Create a new font family
        // Reference: https://stackoverflow.com/questions/78069584/how-to-set-a-new-fontfamily-to-my-egui-app

        // System Text Heavy
        let mut newfam = BTreeMap::new();
        newfam.insert(
            FontFamily::Name("System-Text-Heavy".into()),
            vec![
                "system-text-heavy-1".to_owned(),
                "system-text-heavy-2".to_owned(),
            ],
        );
        fonts.families.append(&mut newfam);

        // Put my font first (highest priority):
        // fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        //     .insert(0, "system_font".to_owned());

        // Put my font as last fallback for monospace:
        // fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        //     .push("system_font".to_owned());

        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Reference: https://github.com/emilk/eframe_template/blob/main/src/app.rs | eframe-0.30.0/src/lib.rs
        cc.egui_ctx.set_fonts(fonts);

        // Redefine text_styles
        // Reference: https://docs.rs/egui/latest/egui/style/struct.Style.html#structfield.text_styles
        let text_styles: BTreeMap<_, _> = [
            (Heading, FontId::new(30.0, FontFamily::Proportional)),
            (Body, FontId::new(12.0, FontFamily::Proportional)),
            (Monospace, FontId::new(14.0, FontFamily::Proportional)),
            (Button, FontId::new(14.0, FontFamily::Proportional)),
            (Small, FontId::new(10.0, FontFamily::Proportional)),
            (
                Name("DialogHeading".into()),
                FontId::new(13.0, FontFamily::Name("System-Text-Heavy".into())),
            ),
        ]
        .into();

        // Mutate global styles with new text styles
        cc.egui_ctx
            .all_styles_mut(move |style| style.text_styles = text_styles.clone());

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

        // Change CentralPanel default style with custom frame 
        // Reference: https://github.com/emilk/egui/discussions/1286
        let my_frame = egui::containers::Frame {
            inner_margin: egui::epaint::Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            outer_margin: egui::epaint::Margin {
                // left: 0.,
                // right: 0.,
                // top: 20.5,
                // bottom: 20.5,
                left: 20.5,
                right: 20.5,
                top: 20.5,
                bottom: 20.5,
            },
            rounding: egui::Rounding {
                nw: 1.0,
                ne: 1.0,
                sw: 1.0,
                se: 1.0,
            },
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::LIGHT_GRAY),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(my_frame)
            .show(ctx, |ui| {
                // egui::CentralPanel::default().show(ctx, |ui| {

                // Reference: https://github.com/emilk/egui/discussions/3933 | https://docs.rs/egui/0.30.0/egui/struct.Ui.html#method.set_row_height | https://docs.rs/egui/0.30.0/egui/struct.Ui.html#method.horizontal
                // ui.set_row_height(30.0);
                // ui.style_mut().spacing.interact_size.y = 30.0; // horizontal layout and the spacing.interact_size.y modifies the height of the first row of layout and button widgets so NOT USEFUL
                // ui.horizontal(|ui| {

                // Reference: egui-0.30.0/src/ui.rs
                ui.allocate_ui_with_layout( Vec2::new(120., 16.), egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                    ui.label(
                        egui::RichText::new("New Folder")
                            .color(egui::Color32::from_rgb(221, 221, 221))
                            .text_style(Name("DialogHeading".into()))
                            // .font(FontId {
                            //     // size: 13.2,
                            //     // size: 13.5,
                            //     size: 12.5,
                            //     family: FontFamily::Name("System-Text-Heavy".into()),
                            // })
                            // .line_height(Some(40.0)) // Create space below the text
                            // .extra_letter_spacing(0.1)
                            // .strong()
                            ,
                    );
                });

                // ui.label(
                //     egui::RichText::new("I am Proportional")
                //         .color(egui::Color32::YELLOW)
                //         .font(FontId {
                //             size: 12.0,
                //             family: FontFamily::Proportional,
                //         }),
                // );
                // ui.label(
                //     egui::RichText::new("I am Monospace")
                //         .color(egui::Color32::GREEN)
                //         .font(FontId {
                //             size: 12.0,
                //             family: FontFamily::Monospace,
                //         }),
                // );

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
                        // Reference: egui-0.30.0/src/ui.rs | https://github.com/emilk/egui/discussions/5340
                        let style_height = ui.text_style_height(&Name("DialogHeading".into()));
                        println!("Text Style Height = {}", style_height)
                        // When FontData is SF-Pro-Text-Heavy.otf
                        // Text Style Height = 14.916992
                        // When FontData is HelveticaNeueHeavy.otf
                        // Text Style Height = 12.35
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
        egui::Color32::from_rgb(33, 32, 29).to_normalized_gamma_f32()
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
