use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::ffi::OsStr;

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

pub fn push_path_to(p: PathBuf, v: &mut Vec<PathBuf>) {
    v.push(p);
}

pub fn push_entries_with_ext(dir: &Path, ext: &str, buf: &mut Vec<PathBuf>,
                             push_fn: &Fn(PathBuf, &mut Vec<PathBuf>)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                try!(push_entries_with_ext(&path, ext, buf, push_fn));
            } else {
                if let Some(e) = ext_from_path(&path) {
                    if ext_match(ext, &e) {
                        push_fn(entry.path(), buf);
                    }
                }

            }
        }
    }
    Ok(())
}