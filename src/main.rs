use std::fs;
use phuhop_core::{eframe, egui};
use phuhop_core::plugin::PluginManager;

fn init_plugins(manager: &mut PluginManager) {
    let extension = if cfg!(target_os = "windows") {
        "dll" 
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        "so"
    } else{
        "unknown"
    };
    for entry in fs::read_dir(&"plugins").unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() { continue; }
        if let Some(file_extension) = path.extension() {
            let file_extension = file_extension.to_str().unwrap(); // Convert OsString to &str
            if file_extension != extension { continue; }
        }
        match manager.load_plugin(path.to_str().unwrap()) {
            Ok(_) => { println!("Loaded plugin: {:?}", path.display())},
            Err(e) => {
                println!("Failed to load plugin: {:?}", e);
            }
        }
    };

}

// #![warn(clippy::all, rust_2018_idioms)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() -> eframe::Result<()> {
    env_logger::init();
    let mut manager = PluginManager::new();
    init_plugins(&mut manager);
    manager.run_plugins();
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "phuhop",
        native_options,
        Box::new(|cc: &eframe::CreationContext| Box::new(phuhop::MyApp::new(cc))),
    )
}

