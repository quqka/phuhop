use phuhop_core::plugin::*;
struct PluginA;

impl Plugin for PluginA {
    fn execute(&self) {
        println!("Hello from PluginA!")
    }
    fn name(&self) -> &'static str {
        "PluginA"
    }
}

#[no_mangle]
pub fn register() -> *mut dyn Plugin {
    Box::into_raw(Box::new(PluginA))
}