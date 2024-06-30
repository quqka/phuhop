use libloading::Library;

pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

pub trait Plugin {
    fn run(&self);
    fn name(&self) -> &'static str;
}

pub struct Registrar {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistrar for Registrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
}

pub fn load_plugin(lib_name: String, registrar: &mut Registrar) -> Result<&str, std::io::Error> {
    let lib = Box::leak(Box::new(unsafe { Library::new(lib_name).unwrap() }));
    unsafe {        
        let func: libloading::Symbol<unsafe extern "Rust" fn(&mut dyn PluginRegistrar) -> ()> =
            lib.get(b"plugin_entry").unwrap();
        func(registrar);
    }
    Ok("success")
}

