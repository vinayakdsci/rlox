// this file is necessary because of the representation of OpCodes as bytes
use crate::chunk::value;
use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::vm;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("====={}=====", name);
    //for all instructions in the chunk, disassemble them
    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("---{}    ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        //check if the last and the current line are same
        print!{"    |   "};
    } else {
        //new inst line
        print!("    {}   ", chunk.lines[offset]);
    }
    let inst = chunk.code[offset];
    match inst {
        OpCode::OpConstant(x) => constant_instruction("OpConstant", x, offset),
        OpCode::OpReturn => simple_instruction("OpReturn", offset),
        OpCode::OpNegate => simple_instruction("OpNegate", offset),
        OpCode::OpAdd => simple_instruction("OpAdd", offset),
        OpCode::OpSubtract => simple_instruction("OpSubtract", offset),
        OpCode::OpDivide => simple_instruction("OpDivide", offset),
        OpCode::OpMultiply => simple_instruction("OpMultiply", offset),
    }
}

fn constant_instruction(name: &str, value: value::Value, offset: usize) -> usize {
    print!("{}   ---   {}", name, value);
    println!();
    offset + 1
}


pub fn debug_stack_trace(vm: &vm::VM) {
    if std::env::args().any(|x| &x == "debug_build") {
        for (_, value) in vm.stack.iter().enumerate() {
            println!(" -- STACK TRACE -- ");
            print!("[ ");
            print_value(*value);
            print!(" ]");
        }
        println!("--- STACK TRACE ENDS ---");
    }
}

pub fn print_value(value: value::Value) {
    print!("{}", value);
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
