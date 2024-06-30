use phuhop_core::plugin::*;
struct PluginA;

impl Plugin for PluginA {
    fn run(&self) {
        println!("Hello from PluginA!")
    }
    fn name(&self) -> &'static str {
        "PluginA"
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginA));
}