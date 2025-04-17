#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::io::{self, Write};
use std::sync::{Arc, Mutex};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).


    // Ask for current folder name via CLI
    print!("Enter current folder name: ");
    io::stdout().flush().unwrap();
    
    let mut folder_name = String::new();
    io::stdin().read_line(&mut folder_name).expect("Failed to read line");
    folder_name = folder_name.trim().to_string();
    
    println!("Opening dialog for folder: {}", folder_name);
    
    // Create a shared result container
    let result = Arc::new(Mutex::new(String::new()));
    let result_clone = result.clone();


    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([321.0, 165.0]) //[321.0, 165.0]
            // .with_min_inner_size([300.0, 220.0])
            // .with_icon(
            //     // NOTE: Adding an icon is optional
            //     eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
            //         .expect("Failed to load icon"),
            // )
            .with_title("")
            .with_resizable(true)
            // .with_mouse_passthrough(true)
            .with_active(true)
            .with_transparent(false)
            .with_decorations(true)
            // .with_always_on_top()
            .with_close_button(true)
            .with_maximize_button(false)
            .with_minimize_button(false)
            // Window modifiers exclusive to macOS
            .with_titlebar_buttons_shown(false)
            .with_titlebar_shown(false)
            .with_fullsize_content_view(true)
            // Window modifiers exclusive to Windows
            .with_taskbar(false)
            ,
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(
          |cc| 
          // TODO: Pass variables in and out of the app without wrapper
          // {
          //   // Pass current_folder value to the app
          //   let app = eframe_test::TemplateApp::new(cc).with_current_folder(folder_name);
          //   // Wrap the app to capture the result on exit
          //   Ok(Box::new(AppWrapper {
          //       app,
          //       result: result_clone,
          //   }))
          // }
          // cc.raw_window_handle = windowHandler.raw_window_handle().unwrap();
          Ok(Box::new(eframe_test::TemplateApp::new(cc).with_current_folder(folder_name).with_result(result_clone)))
          )
    )?;

    // Get the result after the app closes
    let app_result = result.lock().unwrap();
    println!("New folder name: {}", app_result);

    Ok(())
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(eframe_test::TemplateApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
