// mod chunk;
#[path = "scanner.rs"]
pub mod scanner;
// use crate::compiler::chunk::*;
#[path = "compiler.rs"]
pub mod compiler;

#[path = "debug.rs"]
pub mod debug;
// use crate::compiler::;

#[path = "chunk.rs"]
pub mod chunk;

use std::io::BufRead;
use std::io::Write;
mod vm;

fn main() {
    let mut args = std::env::args();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        runfile(std::path::PathBuf::from(args.nth(1).unwrap()));
    } else {
        println!("Usage: cargo run [file_path]");
    }
}

fn repl() {
    // the compiler is single pass, so init the parser here?
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let line = std::io::stdin().lock().lines().next().unwrap().unwrap();

        if line == "exit" {
            break;
        } else {
            let _s = vm::interpret(&line);
        }
    }
    println!("Exited.");
}

fn runfile(file_path: std::path::PathBuf) {
    let source = std::fs::read_to_string(file_path).expect("invalid file path.");
    println!("{:?}", source);
    let _result = vm::interpret(&source);
}

// let ret = chunk.add_constant(24.2);
// chunk.write_chunk(ret, 123);
// chunk.write_chunk(chunk::OpCode::OpReturn, 123);
// let output = vm::interpret(&mut vm_instance, &chunk);
// debug::disassemble_chunk(&chunk, "test chunk");
// debug::debug_stack_trace(&vm_instance);
// println!("Rust Debug mode");

// println!("{:?}", chunk);
