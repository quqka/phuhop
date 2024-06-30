use phuhop_core::{eframe, egui};
use crate::windows::PluginManager;

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("fonts".to_owned(),
    egui::FontData::from_static(include_bytes!("../assets/Yozai-Bold.ttf")));
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "fonts".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .push("fonts".to_owned());
    ctx.set_fonts(fonts);
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct MyApp {
    view: bool,
    show_preferences: bool,
    show_plugin_manager: bool,
    #[serde(skip)]
    plugin_manager: PluginManager,
    // value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            view: true,
            show_preferences: false,
            show_plugin_manager: false,
            plugin_manager: PluginManager::new(),
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_fonts(&cc.egui_ctx);
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // ui.menu_button("File", |ui| {
                //     if ui.button("New Graph").clicked() {
                //     }
                //     if ui.button("Open").clicked() {
                //     }
                //     if ui.button("Exit").clicked() {
                //         ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                //     }
                // });
                // ui.add_space(8.0);
                // ui.menu_button("Edit", |ui| {
                //     if ui.button("Undo").clicked() {
                //     }
                //     if ui.button("Redo").clicked() {
                //     }
                //     ui.add_space(8.0);
                //     if ui.button("Preferences").clicked() {
                //         self.show_preferences = true;
                //         ui.ctx().request_repaint();
                //     }
                // });
                // ui.add_space(8.0);
                ui.menu_button("Windows", |ui| {
                    // if ui.checkbox(&mut self.view, "3D View").changed() { }
                    ui.add_space(8.0);
                    if ui.button("Plugin Manager").clicked() {
                        self.show_plugin_manager = true;
                        ui.ctx().request_repaint();
                    }
                });
                
            });
        });
        self.plugin_manager.show(ctx, &mut self.show_plugin_manager);
        // if self.show_preferences {
        //     egui::Window::new("Preferences")
        //         .open(&mut self.show_preferences)
        //         .show(ctx, |ui| {
        //             ui.label("This is a popup window!");
        //     });
        // }
        // egui::CentralPanel::default().show(ctx, |ui| {
        // });
    }
}

