use std::env;
use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    if let Some(arg) = env::args().nth(1) {
        let arg_path = Path::new(&arg);
        print_vm_file_paths(&arg_path)
    } else {
        panic!("Please provide a File or Directory as your first argument")
    }
}

fn print_vm_file_paths(arg_path: &Path) {
    if arg_path.is_dir() {
        read_dirs(arg_path)
    } else {
        println!("Is Not Dir")
    }
}

fn read_dirs(dir: &Path) {
    let mut vm_file_paths: Vec<PathBuf> = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        //Retrieve File metadata
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(e) = entry.path().extension() {
                    if let Some(exstr) = e.to_str() {
                        match exstr {
                            "vm" => vm_file_paths.push(entry.path()),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", vm_file_paths);
}
