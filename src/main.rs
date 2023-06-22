// mod chunk;
mod debug;
mod chunk;

fn main() {
    let mut chunk = chunk::Chunk::init_chunk();
    let ret = chunk.add_constant(24.2);
    chunk.write_chunk(ret, 123);
    chunk.write_chunk(chunk::OpCode::OpReturn, 123);
    debug::disassemble_chunk(&chunk, "test chunk");

    println!("Rust Debug mode");

    println!("{:?}", chunk);
}
