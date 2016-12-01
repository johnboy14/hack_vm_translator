use std::env;
use std::path::{PathBuf};

mod file;
mod parser;
mod writer;

fn main() {
    //Files with .vm extension
    let mut vm_file_paths: Vec<PathBuf> = vec![];
    if let Some(arg) = env::args().nth(1) {
        let arg_path = PathBuf::from(arg);
        if arg_path.is_dir() {
            file::push_entries_with_ext(&arg_path, "vm", &mut vm_file_paths, &file::push_path_to).unwrap();
        } else {
            vm_file_paths.push(arg_path)
        }
    } else {
        panic!("Please provide a File or Directory as your first argument")
    }
    let mut hack_insts: Vec<String> = vec![];
    for vm_file_path in vm_file_paths {
        println!("Translating the following file into {:?}, Hack machine Language", vm_file_path);
        let mut vm_commands: Vec<parser::VMCommand> = vec![];
        parser::parse_file(&vm_file_path, &mut vm_commands);
        for inst in writer::vm_commands_to_hack(&vm_commands) {
            hack_insts.push(inst);
        }
    }

    for hack_inst in hack_insts {
        println!("{:?}", hack_inst)
    }
}
