use std::path::{PathBuf};
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

pub enum CommandType {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL
}

pub struct VMCommand {
    command_type: CommandType,
    arg1:         String,
    arg2:         String
}

pub fn parse_file(vm_file: &PathBuf, com_vec: &mut Vec<VMCommand>) -> io::Result<()> {
    let mut file = try!(File::open(vm_file));
    let file_buffer = BufReader::new(file);

    //read each line
    for line in file_buffer.lines() {
        let l = line.unwrap();
        if is_comment(&l) == false && is_blank(&l) == false {
            com_vec.push(new_vm_command(&l));
            println!("{:?}", l);
        }
    }

    Ok(())
}

fn is_comment(line: &str) -> bool {
    line.starts_with('/')
}

fn is_blank(line: &str) -> bool {
    line.is_empty()
}

fn new_vm_command(line: &str) -> VMCommand {
    let spli = line.split_whitespace();

    return VMCommand{command_type: CommandType::C_ARITHMETIC, arg1: "arg1".to_string(), arg2: "arg2".to_string()};
}