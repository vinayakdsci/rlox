// The Virtual Machine!
use crate::chunk;
use crate::chunk::value::Value;
use crate::compiler;



const STACK_MAX: usize = 256;

#[repr(u8)]
#[derive(PartialEq)]
pub enum InterpretResult {
    InterpretOK = 1,
    InterpretCompileError,
    InterpretRuntimeError,
}


pub struct VM {
    pub chunk: chunk::Chunk,
    pub inst_pointer: usize, // Rust might not allow pointers to the middle of the array, sp use an index insead
    pub stack: Vec<Value>,
}

impl VM {

    pub fn init_vm(chunk: &chunk::Chunk) -> Self {
        Self {
            chunk: chunk.to_owned(),
            inst_pointer: 0,
            stack: Vec::<Value>::new(),
        }
    }

    // fn reset_stack(&mut self) {
    //     // self.stack_top = self.stack.len();
    // }
    pub fn pop(&mut self) -> Value {
        // assert_eq!(self.stack.capacity(), self.stack.len());
        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
}


pub fn interpret(source: &str) -> InterpretResult {
    compiler::compile(source);
    InterpretResult::InterpretOK
}

fn binary_solver(vm: &mut VM, operator: char) {
    let a = vm.pop();
    let b = vm.pop();
    match operator {
        '+' => vm.push(a + b),
        '-' => vm.push(a - b),
        '*' => vm.push(a * b),
        '/' => {
            if b == 0f32 {
                println!("Error! cannot divide by 0");
                vm.push(b);
                vm.push(a);
            }
            else {
                vm.push(a / b);
            }
        },
        _ => {
            println!("This operator is not recognized");
        }
    }
}

fn run(vm: &mut VM) -> InterpretResult {
    for x in 0..vm.chunk.code.len() {
        let op_code = vm.chunk.code[x];
        vm.inst_pointer += 1;
        match op_code {
            chunk::OpCode::OpReturn => {
                println!("{}", vm.pop());
                return InterpretResult::InterpretOK
            },
            chunk::OpCode::OpNegate => {
                let neg = vm.pop();
                vm.push(-neg);
            },
            chunk::OpCode::OpAdd => binary_solver(vm, '+'),
            chunk::OpCode::OpSubtract => binary_solver(vm, '-'),
            chunk::OpCode::OpMultiply => binary_solver(vm, '*'),
            chunk::OpCode::OpDivide => binary_solver(vm, '/'),
            chunk::OpCode::OpConstant(x) => vm.push(x),
            // _ => {
            //     panic!("Unknown Operation Instruction encountered");
            // }
        }
    }
    InterpretResult::InterpretCompileError
}
