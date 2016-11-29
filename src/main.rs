use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::ffi::OsStr;

fn push_path_to(p: PathBuf, v: &mut Vec<PathBuf>) {
    v.push(p);
}

fn main() {
    //Files with .vm extension
    let mut vm_file_paths: Vec<PathBuf> = vec![];
    if let Some(arg) = env::args().nth(1) {
        let arg_path = PathBuf::from(arg);
        if arg_path.is_dir() {
            push_entries_with_ext(&arg_path, "vm", &mut vm_file_paths, &push_path_to).unwrap();
        } else {
            vm_file_paths.push(arg_path)
        }
    } else {
        panic!("Please provide a File or Directory as your first argument")
    }
    println!("Translating the following files into {:?}, Hack machine Language", vm_file_paths)
}

fn ext_from_path<'a>(path: &'a Path) -> Option<&'a OsStr> {
    match path.extension() {
        Some(ext) => Some(ext),
        None => None
    }
}

fn ext_match(ext: &str, other: &OsStr) -> bool {
    match other.to_str() {
        Some(s) => ext == s,
        None => false
    }
}

fn push_entries_with_ext(dir: &Path, ext: &str,  buf: &mut Vec<PathBuf>, cb: &Fn(PathBuf, &mut Vec<PathBuf>)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                try!(push_entries_with_ext(&path, ext, buf, cb));
            } else {
                if let Some(e) = ext_from_path(&path) {
                    if ext_match(ext, &e) {
                        //cb(path, buf);
                        cb(entry.path(), buf);
                    }
                }

            }
        }
    }
    Ok(())
}
