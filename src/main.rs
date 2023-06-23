// mod chunk;
use std::io::Write;
use std::io::BufRead;
mod debug;
mod chunk;
mod vm;
mod compiler;

fn main() {

    let mut chunk = chunk::Chunk::init_chunk();
    let mut vm_instance = vm::VM::init_vm(&chunk);
    let mut args = std::env::args();
    
    if args.len() == 1 {
        repl();
    }
    else if args.len() == 2 {
        runfile(std::path::PathBuf::from(args.nth(1).unwrap()));
    }
    else {
        println!("Usage: cargo run [file_path]");
    }
}


fn repl() {
    loop{
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let line = std::io::stdin().lock().lines().next().unwrap().unwrap();

        if line == "exit" {
            break;
        }
        else {
            let s = vm::interpret(line);
        }
    }
    println!("Exited.");
}


fn runfile(file_path: std::path::PathBuf) {
    let mut source = std::fs::read_to_string(file_path).expect("invalid file path.");
    let result = vm::interpret(source);
}

// let ret = chunk.add_constant(24.2);
// chunk.write_chunk(ret, 123);
// chunk.write_chunk(chunk::OpCode::OpReturn, 123);
// let output = vm::interpret(&mut vm_instance, &chunk);
// debug::disassemble_chunk(&chunk, "test chunk");
// debug::debug_stack_trace(&vm_instance);
// println!("Rust Debug mode");

// println!("{:?}", chunk);
