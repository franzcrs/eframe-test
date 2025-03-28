use egui::Color32;
use egui::FontData;
use egui::FontDefinitions;
use egui::{FontFamily, FontId};
use std::collections::BTreeMap;

use egui::TextStyle::*;
use egui::epaint::Margin;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    stroke_color: Color32,
    is_focused: bool,
    selection_color: Color32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "untitled folder".to_owned(),
            value: 2.7,
            stroke_color: Color32::TRANSPARENT,
            is_focused: false,
            selection_color: Color32::from_rgb(71,98,135),
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

        // System Text Regular
        // 1st option
        fonts.font_data.insert(
            "system-text-regular-1".to_owned(),
            std::sync::Arc::new(
                // 1st option
                FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro-Text-Medium.otf")),
            ),
        );
        // 2nd option

        // Create a new font family
        // Reference: https://github.com/emilk/egui/discussions/4449 | https://stackoverflow.com/questions/78069584/how-to-set-a-new-fontfamily-to-my-egui-app

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

        // System Text Regular
        newfam = BTreeMap::new();
        newfam.insert(
            FontFamily::Name("System-Text-Regular".into()),
            vec!["system-text-regular-1".to_owned()],
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

        // Redefine text_styles adding new text styles
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
            (
                Name("DialogBody".into()),
                FontId::new(11.0, FontFamily::Name("System-Text-Regular".into())),
            ),
            (
                Name("TextInputBody".into()),
                FontId::new(13.0, FontFamily::Name("System-Text-Regular".into())),
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
        let dialog_frame = egui::containers::Frame {
            inner_margin: Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            outer_margin: Margin {
                // left: 20.5-3.5,
                // right: 20.5-3.5,
                // top: 0.,
                // bottom: 0.,
                left: 20.5-3.5,
                right: 20.5-3.5,
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
        let text_box_frame = egui::containers::Frame {
            inner_margin: Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            outer_margin: Margin {
                left: 0.2,
                right: 0.2,
                top: 0.2,
                bottom: 0.2,
            },
            rounding: egui::Rounding {
                nw: 0.0,
                ne: 0.0,
                sw: 0.0,
                se: 0.0,
            },
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::from_rgb(44, 43, 40),
            stroke: egui::Stroke::new(0.15, Color32::from_rgb(83, 82,82)),
            ..Default::default()
        };
        let text_box_stroke = egui::containers::Frame {
            inner_margin: Margin {
                left: 3.5,
                right: 3.5,
                top: 3.5,
                bottom: 3.5,
            },
            outer_margin: Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            rounding: egui::Rounding {
                nw: 3.,
                ne: 3.,
                sw: 3.,
                se: 3.,
            },
            shadow: eframe::epaint::Shadow::NONE,
            fill: self.stroke_color, // fill: Color32::from_rgb(56, 100, 138),
            stroke: egui::Stroke::new(0.0, Color32::WHITE),
            ..Default::default()
        };
        // Store a reference to our text edit output for later use
        let mut text_edit_output = None;
        egui::CentralPanel::default()
            .frame(dialog_frame)
            .show(ctx, |ui| {
                // egui::CentralPanel::default().show(ctx, |ui| {

                ui.horizontal(|ui| {
                    ui.add_space(3.5);
                    // Reference: https://github.com/emilk/egui/discussions/3933 | https://docs.rs/egui/0.30.0/egui/struct.Ui.html#method.set_row_height | https://docs.rs/egui/0.30.0/egui/struct.Ui.html#method.horizontal
                    // ui.set_row_height(30.0);
                    // ui.style_mut().spacing.interact_size.y = 30.0; // horizontal layout and the spacing.interact_size.y modifies the height of the first row of layout and button widgets so NOT USEFUL
                    // ui.horizontal(|ui| {

                    // Reference: egui-0.30.0/src/ui.rs
                    ui.allocate_ui_with_layout([f32::INFINITY, 16.].into(), egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                        ui.label(
                            egui::RichText::new("New Folder")
                                .color(Color32::from_rgb(221, 221, 221))
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
                ui.horizontal(|ui| {
                    ui.add_space(3.5);
                    ui.allocate_ui_with_layout([f32::INFINITY, 35.].into(), egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        ui.label(
                            egui::RichText::new("“current folder”:")
                                .color(Color32::from_rgb(221, 221, 221))
                                .text_style(Name("DialogBody".into()))
                                .font(FontId {
                                  size: 11.2, 
                                  family: FontFamily::Name("System-Text-Regular".into())
                                })
                                ,
                        );
                        ui.label(
                            egui::RichText::new("Name of new folder inside")
                                .color(Color32::from_rgb(221, 221, 221))
                                .text_style(Name("DialogBody".into()))
                                .line_height(Some(11.)) // Create space below the text
                                ,
                        );
                    });
                });
                ui.add_space(6.0);
                // ui.add(
                //     egui::TextEdit::singleline(&mut self.label)
                //         // .hint_text("Type something...") // Placeholder text
                //         .desired_width(f32::INFINITY) // Make it take full width
                //         .font(Name("TextInputBody".into()))
                //         .margin(Margin::symmetric(4.0, 0.5))
                //         .background_color(Color32::from_rgb(44, 43, 40))
                //         .text_color(Color32::from_rgb(221, 221, 221))
                //         .frame(false)
                //         //TODO: Create a frame or a window area to change the stroke style of the textedit widget
                //         ,
                // );
                text_box_stroke.show(ui, |ui| {
                    text_box_frame.show(ui, |ui| {
                        let visuals = ui.visuals_mut();
                        // visuals.selection.stroke = egui::Stroke::new(2.0, Color32::RED); // Change stroke color
                        visuals.selection.bg_fill = self.selection_color; // Change background fill color
                        // ui.add(
                        //     egui::TextEdit::singleline(&mut self.label)
                        //         // .hint_text("Type something...") // Placeholder text
                        //         .desired_width(f32::INFINITY) // Make it take full width
                        //         .font(Name("TextInputBody".into()))
                        //         .margin(Margin::symmetric(4.0, 1.))
                        //         .background_color(Color32::from_rgb(44, 43, 40))
                        //         .text_color(Color32::from_rgb(221, 221, 221))
                        //         .frame(false)
                        //         .lock_focus(true)
                        //         ,
                        // );
                        // Use show() instead of ui.add() to get the output state
                        let output = egui::TextEdit::singleline(&mut self.label)
                            .desired_width(f32::INFINITY)
                            .font(Name("TextInputBody".into()))
                            .margin(Margin::symmetric(4.0, 1.))
                            .background_color(Color32::from_rgb(44, 43, 40))
                            .text_color(Color32::from_rgb(221, 221, 221))
                            .frame(false)
                            .lock_focus(true)
                            .show(ui);
                    
                        // Select all text by setting cursor range from 0 to end of text, on focus
                        // Reference: https://stackoverflow.com/questions/74324236/select-the-text-of-a-textedit-object-in-egui
                        // use egui::{text::CCursor, text::CCursorRange};
                        // if output.response.gained_focus() { // If the widget is focused
                        // // if self.is_focused {
                        //     output.state.cursor.set_char_range(Some(
                        //         CCursorRange::two(
                        //             CCursor::new(0),
                        //             CCursor::new(self.label.len())
                        //         )
                        //     ));
                        //     // Apply the changes
                        //     output.state.store(ui.ctx(), output.response.id);
                        // }

                        // Store output for later use outside of this UI closure
                        text_edit_output = Some(output);
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    ui.add_space(3.5);
                    // Primary button (Create) - Blue with white text
                    let create_button = egui::Button::new(
                        egui::RichText::new("Create").color(egui::Color32::WHITE),
                    )
                    .fill(Color32::from_rgb(0, 122, 255)) // macOS blue
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
        // Check if the egui window is focused.
        // Reference: https://docs.rs/egui/0.30.0/egui/struct.Context.html#method.input
        self.is_focused = ctx.input(|i| i.focused);
        if self.is_focused {
            // Update the stroke color based on focus
            self.stroke_color = Color32::from_rgb(56, 100, 138);
            // Update the selection color based on focus
            self.selection_color = Color32::from_rgb(71,98,135);
            // Select all text in TextEdit widget on a specific condition
            // Reference: https://stackoverflow.com/questions/74324236/select-the-text-of-a-textedit-object-in-egui
            if let Some(mut output) = text_edit_output {
                use egui::{text::CCursor, text::CCursorRange};
                // Defining the selection of text if the widget is focused
                if output.response.gained_focus() {
                    output.state.cursor.set_char_range(Some(
                        CCursorRange::two(
                            CCursor::new(0),
                            CCursor::new(self.label.len())
                        )
                        //TODO: track the cursor position and set the cursor range to the current cursor position
                    ));
                    // Apply the changes
                    output.state.store(ctx, output.response.id);
                }
                // Request focus on the TextEdit widget
                output.response.request_focus();
            }
        }
        else {
            // Update the stroke color based on focus
            self.stroke_color = Color32::TRANSPARENT;
            // Update the selection color based on focus
            self.selection_color = Color32::from_rgb(70,70,70);
        }
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
        Color32::from_rgb(33, 32, 29).to_normalized_gamma_f32()
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
