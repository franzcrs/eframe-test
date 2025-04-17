use egui::Color32;
use egui::FontData;
use egui::FontDefinitions;
use egui::Stroke;
use egui::Vec2;
use egui::{FontFamily, FontId};
use std::collections::BTreeMap;

use egui::TextStyle::*;
use egui::epaint::Margin;
use std::sync::{Arc, Mutex};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    folder_name: String,
    current_folder: String,
    #[serde(skip)] // This is how you opt-out of serialization of a field
    result: Arc<Mutex<String>>, // Output result
    #[serde(skip)] // This is how you opt-out of serialization of a field
    stroke_color: Color32,
    is_focused: bool,
    #[serde(skip)] // This is how you opt-out of serialization of a field
    selection_color: Color32,
    #[serde(skip)] // This is how you opt-out of serialization of a field
    ok_button_color: Color32,

    #[serde(skip)] // This is how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            folder_name: String::from("untitled folder"),
            current_folder: String::from("current folder name"),
            result: Arc::new(Mutex::new(String::new())), // Output result
            value: 2.7,
            stroke_color: Color32::TRANSPARENT,
            is_focused: false,
            selection_color: Color32::from_rgb(71,98,135), // macOS text selection color
            ok_button_color: Color32::from_rgb(48, 98, 212), // macOS blue button color
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

        // System Text Medium
        // 1st option
        fonts.font_data.insert(
            "system-text-medium-1".to_owned(),
            std::sync::Arc::new(
                FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro-Text-Medium.otf")),
            ),
        );
        // 2nd option

        // System Display Medium
        // 1st option
        fonts.font_data.insert(
            "system-display-medium-1".to_owned(),
            std::sync::Arc::new(
                FontData::from_static(include_bytes!("/Library/Fonts/SF-Pro-Display-Medium.otf")),
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

        // System Text Medium
        newfam = BTreeMap::new();
        newfam.insert(
            FontFamily::Name("System-Text-Medium".into()),
            vec!["system-text-medium-1".to_owned()],
        );
        fonts.families.append(&mut newfam);

        // System Display Medium
        newfam = BTreeMap::new();
        newfam.insert(
            FontFamily::Name("System-Display-Medium".into()),
            vec!["system-display-medium-1".to_owned()],
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
                FontId::new(11.0, FontFamily::Name("System-Text-Medium".into())),
            ),
            (
              Name("DialogFolderName".into()),
              FontId::new(11.5, FontFamily::Name("System-Text-Medium".into())),
          ),
            (
                Name("TextInputBody".into()),
                FontId::new(13.0, FontFamily::Name("System-Text-Medium".into())),
            ),
            (
                Name("ButtonBody".into()),
                FontId::new(13.5, FontFamily::Name("System-Display-Medium".into())),
            )
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

    /// Setter for current_folder value
    pub fn with_current_folder(mut self, name: String) -> Self {
        self.current_folder = name;
        self
    }

    /// Setter for the result variable
    pub fn with_result(mut self, result: Arc<Mutex<String>>) -> Self {
        self.result = result;
        self
    }

    /// Getter for folder_name value
    pub fn get_result(&self) -> String {
        self.folder_name.clone()
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
                left: 20.5-3.5,
                right: 20.5-3.5,
                top: 19.5,
                bottom: 20.,
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
        // let mut text_edit_output = None;
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
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("New Folder".to_owned())
                                    .color(Color32::from_rgb(221, 221, 221))
                                    .text_style(Name("DialogHeading".into()))
                                    ,)
                                .selectable(false)
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
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("“".to_owned() + &self.current_folder + "”:")
                                    .color(Color32::from_rgb(221, 221, 221))
                                    .text_style(Name("DialogFolderName".into()))
                                    ,)
                                .selectable(false)
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Name of new folder inside")
                                    .color(Color32::from_rgb(221, 221, 221))
                                    .text_style(Name("DialogBody".into()))
                                    .line_height(Some(11.)) // Create space below the text
                                    ,)
                                .selectable(false)
                        );
                    });
                });
                ui.add_space(4.5);
                // ui.add(
                //     egui::TextEdit::singleline(&mut self.folder_name)
                //         // .hint_text("Type something...") // Placeholder text
                //         .desired_width(f32::INFINITY) // Make it take full width
                //         .font(Name("TextInputBody".into()))
                //         .margin(Margin::symmetric(4.0, 0.5))
                //         .background_color(Color32::from_rgb(44, 43, 40))
                //         .text_color(Color32::from_rgb(221, 221, 221))
                //         .frame(false)
                //         ,
                // );
                text_box_stroke.show(ui, |ui| {
                    text_box_frame.show(ui, |ui| {
                        let visuals = ui.visuals_mut();
                        // visuals.selection.stroke = egui::Stroke::new(2.0, Color32::RED); // Change stroke color
                        visuals.selection.bg_fill = self.selection_color; // Change background fill color
                        // ui.add(
                        //     egui::TextEdit::singleline(&mut self.folder_name)
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
                        let mut output = egui::TextEdit::singleline(&mut self.folder_name)
                            .desired_width(f32::INFINITY)
                            .font(Name("TextInputBody".into()))
                            .margin(Margin::symmetric(3.0, 1.))
                            .background_color(Color32::from_rgb(44, 43, 40))
                            .text_color(Color32::from_rgb(221, 221, 221))
                            .frame(false)
                            .lock_focus(true)
                            .show(ui);

                        // Request focus on the TextEdit widget, once on first rendering
                        output.response.request_focus();

                        // Select all text by setting cursor range from 0 to end of text, on focus
                        // Reference: https://stackoverflow.com/questions/74324236/select-the-text-of-a-textedit-object-in-egui
                        use egui::{text::CCursor, text::CCursorRange};
                        if output.response.gained_focus() { // If the widget is focused
                        // if self.is_focused {
                            output.state.cursor.set_char_range(Some(
                                CCursorRange::two(
                                    CCursor::new(0),
                                    CCursor::new(self.folder_name.len())
                                )
                            ));
                            // Apply the changes
                            output.state.store(ui.ctx(), output.response.id);
                        }

                        // Store output for later use outside of this UI closure
                        // text_edit_output = Some(output);
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    ui.add_space(3.);

                    // Custom button styles
                    // Reference: button.rs & style.rs
                    let styles = ui.style_mut();
                    styles.spacing.button_padding = egui::vec2(10.0, 2.0);
                    styles.visuals.widgets.hovered.expansion = 0.0;
                    styles.visuals.widgets.active.expansion = 0.0;

                    // Primary button (Create) - Blue with white text
                    let create_button = egui::Button::new(
                        egui::RichText::new("Create")
                        .color(egui::Color32::from_rgb(221, 221, 221))
                        .text_style(Name("ButtonBody".into()))
                        .extra_letter_spacing(0.2),
                    )
                    .fill(self.ok_button_color) // macOS blue
                    .rounding(egui::Rounding::same(5.0)) // macOS rounded corners
                    .frame(true)
                    .min_size(Vec2::new(61., 20.))
                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                    ;

                    if ui.add(create_button).clicked() {
                        // Handle create button click
                        println!("Create clicked. New folder name: {}", self.folder_name);
                        // Reference: egui-0.30.0/src/ui.rs | https://github.com/emilk/egui/discussions/5340
                        let style_height = ui.text_style_height(&Name("DialogHeading".into()));
                        println!("Text Style Height = {}", style_height);
                        // Window size
                        let window_size = ctx.screen_rect().size();
                        println!("Window size = {:?}", window_size);
                        // Window position
                        let window_pos = ctx.screen_rect().min;
                        println!("Window position = {:?}", window_pos);
                        // Window center
                        let window_center = ctx.screen_rect().center();
                        println!("Window center = {:?}", window_center);
                        // Close the window
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    ui.add_space(0.1); // Space between buttons

                    // Secondary button (Cancel) - Light gray with default text
                    let cancel_button = egui::Button::new(
                        egui::RichText::new("Cancel")
                        .color(egui::Color32::from_rgb(221, 221, 221))
                        .text_style(Name("ButtonBody".into()))
                        .extra_letter_spacing(0.2),
                    )
                    .fill(Color32::from_rgb(89, 88, 86)) // macOS gray button color
                    .rounding(egui::Rounding::same(5.0)) // macOS rounded corners
                    .frame(true)
                    // .min_size(Vec2::new(61., 20.))
                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                    ;


                    if ui.add(cancel_button).clicked() {
                        // Handle cancel button click
                        println!("Cancel clicked");
                        // Optional: reset the input field
                        // self.folder_name = "untitled folder".to_owned();
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
            self.selection_color = Color32::from_rgb(71,98,135); // macOS text selection color
            // Select all text in TextEdit widget on a specific condition
            // Reference: https://stackoverflow.com/questions/74324236/select-the-text-of-a-textedit-object-in-egui
            // if let Some(mut output) = text_edit_output {
            //     use egui::{text::CCursor, text::CCursorRange};
            //     // Defining the selection of text if the widget is focused
            //     if output.response.gained_focus() {
            //         output.state.cursor.set_char_range(Some(
            //             CCursorRange::two(
            //                 CCursor::new(0),
            //                 CCursor::new(self.folder_name.len())
            //             )
            //         ));
            //         // Apply the changes
            //         output.state.store(ctx, output.response.id);
            //     }
            //     // Request focus on the TextEdit widget
            //     output.response.request_focus();
            // }
            // Update the color of the OK button based on focus
            self.ok_button_color = Color32::from_rgb(48, 98, 212); // macOS blue button color
        }
        else {
            // Update the stroke color based on focus
            self.stroke_color = Color32::TRANSPARENT;
            // Update the selection color based on focus
            self.selection_color = Color32::from_rgb(70,70,70); // macOS unfocused text selection color
            // Update the color of the OK button based on focus
            self.ok_button_color = Color32::from_rgb(89, 88, 86); // macOS gray button color
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // _visuals.window_fill().to_normalized_gamma_f32()
        // egui::Color32::from_gray(27).to_normalized_gamma_f32()
        Color32::from_rgb(33, 32, 29).to_normalized_gamma_f32()
    }

    // Reference: https://qiita.com/8bitTD/items/7d745bbf90a82aaffd7f
    fn on_exit(&mut self, _:Option<&eframe::glow::Context>) {
        // Save the result when the app is closing
        let mut result = self.result.lock().unwrap();
        *result = self.get_result();
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
