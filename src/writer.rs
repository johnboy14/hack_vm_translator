
use parser;

const  STACK_POINTER: &'static str = "@SP";

pub fn vm_commands_to_hack(vm_commands: &Vec<parser::VMCommand>) -> Vec<String> {
    let mut hack_instructions: Vec<String> = vec![];
    for vm_command in vm_commands {
        let insts = match vm_command.command_type {
            parser::CommandType::C_ARITHMETIC => arithmetic_instruction(&vm_command.arg1),
            _ => vec![]
        };
        for inst in insts {
            hack_instructions.push(inst);
        }
    }
    hack_instructions
}

fn arithmetic_instruction(command: &String) -> Vec<String> {
    let mut insts: Vec<String> = vec![];
    insts.push(STACK_POINTER.to_string());
    insts.push("AM=M-1".to_string());
    insts.push("D=M".to_string());
    insts.push("@SP".to_string());
    insts.push("A=M-1".to_string());
    if command.eq(&String::from("add")) {
        insts.push("M=M+D".to_string());
    } else {
        insts.push("M=M-D".to_string());
    }
    insts
}