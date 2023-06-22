#[path = "value.rs"]
pub mod value;


#[derive(Default, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    #[default]
    OpReturn = 1,
    OpConstant(value::Value),
}


#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>, 
    pub count: usize,
    pub constants: value::ValueArray,
    pub lines: Vec<i32>,
}


impl Chunk {

    pub fn init_chunk() -> Self {

        let code_store: Vec<OpCode> = Vec::with_capacity(8);
        Self {
            code: code_store,
            count: 0,
            constants: value::ValueArray::init_value_array(),
            lines: Vec::<i32>::with_capacity(8),
        }
    }

    pub fn add_constant(&mut self, value: value::Value) -> OpCode {
        self.constants.write_value_array(value);
        OpCode::OpConstant(value)
    }

    pub fn write_chunk(&mut self, inst: OpCode, line: i32) {
        self.code.reserve_exact(2 * self.code.capacity());
        self.lines.reserve_exact(2 * self.lines.capacity());

        // if self.lines.is_empty() {
        self.lines.push(line);
        // }
        // else {
        // self.code[self.count] = inst;
        // }

        // if self.code.is_empty() {
        self.code.push(inst);
        // }
        // else {
        // self.lines[self.count] = line;
        // }
        self.count += 1;
    }
}

