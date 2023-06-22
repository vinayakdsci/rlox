// this file is necessary because of the representation of OpCodes as bytes
use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::chunk::value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("====={}=====", name);
    //for all instructions in the chunk, disassemble them
    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{
    print!("---{}    ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        //check if the last and the current line are same
        print!{"    |   "};
    }
    else {
        //new inst line
        print!("    {}   ", chunk.lines[offset]);
    }
    let inst = chunk.code[offset];
    match inst {
        OpCode::OpConstant(x) => {
            constant_instruction("OpConstant", x, offset)
        } 
        OpCode::OpReturn => {
            simple_instruction("OpReturn", offset)
        }
    }
}

fn constant_instruction(name: &str, value: value::Value, offset: usize) -> usize {
    print!("{}   ---   {}", name, value);
    println!();
    offset + 1

}

fn print_value(value: value::Value) {
    println!("{}", value);
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
