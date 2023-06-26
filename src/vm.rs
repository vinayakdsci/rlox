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
    pub inst_pointer: usize, // Rust might not allow pointers to the middle of the array, so use an index insead
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

    fn pop(&mut self) -> Value {
        match self.stack.last() {
            Some(x) => {
                return self.stack.pop().unwrap();
            }
            None => {
               eprintln!("No values in the stack, expression required");
               64f64
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
}


pub fn interpret(source: &str) -> InterpretResult {
    //pass the chunk to the compiler (remember borrow),
    //fill it with bytecode, and then execute it on the VM
    let mut chunk = chunk::Chunk::init_chunk();
    let mut vm = VM::init_vm(&chunk);
    let mut scanner = compiler::scanner::Scanner::init_scanner();
    let mut parser = compiler::Parser::init_parser();

    if !compiler::compile(source, &mut chunk, &mut parser, &mut scanner) {
        return InterpretResult::InterpretCompileError;
    }

    vm.chunk = chunk;
    vm.inst_pointer = 0;
    let result: InterpretResult = run(&mut vm);

    result
        // InterpretResult::InterpretOK
}

fn binary_solver(vm: &mut VM, operator: char) {
    let a = vm.pop();
    let b = vm.pop();
    match operator {
        '+' => vm.push(a + b),
        '-' => vm.push(b - a),
        '*' => vm.push(a * b),
        '/' => {
            if a == 0f64 {
                println!("Error! cannot divide by 0");
                vm.push(b);
                vm.push(a);
            }
            else {
                vm.push(b / a);
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
