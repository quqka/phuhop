
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::ffi::CString;
use std::path::PathBuf;

pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

type PluginInitFunc = unsafe extern "Rust" fn() -> *mut dyn Plugin;

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    loaded_libraries: HashMap<String, Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
            loaded_libraries: HashMap::new(),
        }
    }

    pub fn run_plugins(&mut self) {
        for plugin in self.plugins.values_mut() {
            plugin.execute();
        }
    }
    

    pub fn load_plugin(&mut self, path: &str) -> Result<(), String> {
        let lib_path = PathBuf::from(path);
        if !lib_path.exists() || !lib_path.is_file() {
            return Err(format!("{} does not exist or is not a file.", path));
        }

        let lib = unsafe { Library::new(path) };
        match lib {
            Ok(library) => {
                unsafe {
                    let init_func: Symbol<PluginInitFunc> = library.get(b"register").unwrap();
                    let plugin_ptr = init_func();
                    if !plugin_ptr.is_null() {
                        let plugin_box: Box<dyn Plugin> = Box::from_raw(plugin_ptr);
                        let plugin_name = CString::from_vec_unchecked(plugin_box.name().into())
                            .into_string()
                            .unwrap();
                        self.plugins.insert(plugin_name, plugin_box);
                        self.loaded_libraries.insert(path.to_owned(), library);
                        Ok(())
                    } else {
                        Err("Failed to create plugin instance.".to_string())
                    }
                }
            },
            Err(e) => Err(format!("Failed to load library {}: {}", path, e)),
        }
    }

}

