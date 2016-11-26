use std::env;
use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    //Files with .vm extension
    let mut vm_file_paths: Vec<PathBuf> = vec![];
    if let Some(arg) = env::args().nth(1) {
        let arg_path = PathBuf::from(arg);
        if arg_path.is_dir() {
            find_with_ext(&arg_path, "vm", &mut vm_file_paths)
        } else {
            vm_file_paths.push(arg_path)
        }
    } else {
        panic!("Please provide a File or Directory as your first argument")
    }
    println!("Translating the following files into {:?}, Hack machine Language", vm_file_paths)
}

fn find_with_ext(dir: &Path, ext: &str, vec: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        //Retrieve File metadata
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(e) = entry.path().extension() {
                    if let Some(exstr) = e.to_str() {
                        if ext == exstr {
                            vec.push(entry.path())
                        }
                    }
                }
            }
        }
    }
}
