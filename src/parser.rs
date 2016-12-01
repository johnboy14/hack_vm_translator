use std::path::{PathBuf};
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

#[derive(PartialEq)]
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
    pub command_type: CommandType,
    pub arg1:         String,
    pub arg2:         Option<String>
}

pub fn parse_file(vm_file: &PathBuf, com_vec: &mut Vec<VMCommand>) -> io::Result<()> {
    let mut file = try!(File::open(vm_file));
    let file_buffer = BufReader::new(file);

    //read each line
    for line in file_buffer.lines() {
        let l = line.unwrap();
        if !is_comment(&l) && !is_blank(&l) {
            com_vec.push(new_vm_command(&l));
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
    let command = spli.next().unwrap();
    let command_type = match command {
        "push" => CommandType::C_PUSH,
        "pop"  => CommandType::C_POP,
        _      => CommandType::C_ARITHMETIC
    };

    let arg1 = match command_type {
        CommandType::C_ARITHMETIC => command.to_string(),
        _                         => spli.next().unwrap().to_string()
    };

    let arg2 = match spli.next() {
        Some(a) => Some(a.to_string()),
        None    => None
    };

    return VMCommand{command_type: command_type, arg1: arg1, arg2: arg2};
}
