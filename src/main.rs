// mod chunk;
mod debug;
mod chunk;
mod vm;

fn main() {

    let mut chunk = chunk::Chunk::init_chunk();
    let mut vm_instance = vm::VM::init_vm(&chunk);
    let ret = chunk.add_constant(24.2);
    chunk.write_chunk(ret, 123);
    chunk.write_chunk(chunk::OpCode::OpReturn, 123);
    let output = vm::interpret(&mut vm_instance, &chunk);
    debug::disassemble_chunk(&chunk, "test chunk");
    debug::debug_stack_trace(&vm_instance);
    println!("Rust Debug mode");

    println!("{:?}", chunk);
}
