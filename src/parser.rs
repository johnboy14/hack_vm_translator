use std::path::{PathBuf};
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct VMCommand {
    command_type: CommandType,
    arg1:         Option<String>,
    arg2:         Option<String>
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
    let mut spli = line.split_whitespace();

    let command = match spli.next() {
        Some("push") => CommandType::C_PUSH,
        Some("pop")  => CommandType::C_POP,
        _            => CommandType::C_ARITHMETIC
    };
    
    let arg1 = match spli.next() {
        Some(a) => Some(a.to_string()),
        None    => None
    };

    let arg2 = match spli.next() {
        Some(a) => Some(a.to_string()),
        None    => None
    };

    return VMCommand{command_type: command, arg1: arg1, arg2: arg2};
}
