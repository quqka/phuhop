use egui_virtual_list::VirtualList;
use phuhop_core::egui;

pub struct PluginWindow {
    items: Vec<String>,
    virtual_list: VirtualList,
    search_string: String,
}

impl PluginWindow {
    pub fn new() -> Self {
        Self { items: vec!["plugina".to_string()], virtual_list: VirtualList::new(), search_string: "".to_string()}
    }
    pub fn show(&mut self, ctx: &egui::Context, show_popup: &mut bool)
    {
        // if !*show_popup { return; }
        egui::Window::new("Plugin Manager")
            .open(show_popup)
            .show(ctx, |ui| {
                let response = ui.add(egui::TextEdit::singleline(&mut self.search_string));
                if response.changed() {
                    println!("{:?}", self.search_string);
                }
                self.virtual_list.ui_custom_layout(ui, self.items.len(), |ui, start_index| {
                    let item = &self.items[start_index];
                    ui.horizontal(|ui| {
                        ui.label(item);
                        ui.checkbox(&mut true, "Loaded");
                    });
                    1
                });
                
        });
    }
}

