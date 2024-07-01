use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let project_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dll_source_dir = Path::new(&project_root).join("target/debug/");
    let target_dir = Path::new(&project_root).join("plugins");
    fs::create_dir_all(&target_dir).expect("Failed to create target directory");
    
    if let Ok(entries) = fs::read_dir(dll_source_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "dll" || extension == "so" || extension == "dylib" {
                        let target_path = target_dir.join(path.file_name().unwrap());
                        fs::copy(&path, &target_path).expect("Failed to copy DLL");
                        println!("Copied {} to {:?}", path.display(), target_path);
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read source directory");
    }
}