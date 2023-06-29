use std::str::FromStr;

#[path = "scanner.rs"]
pub mod scanner;

use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::debug::*;

const PREC_NONE: u8 = 1;
const PREC_ASSIGNMENT: u8 = 2; // =
const PREC_OR: u8 = 3; // or
const PREC_AND: u8 = 4; // and
const PREC_EQUALITY: u8 = 5; // == !=
const PREC_COMPARISON: u8 = 6; // < > <= >=
const PREC_TERM: u8 = 7; // + -
const PREC_FACTOR: u8 = 8; // * /
const PREC_UNARY: u8 = 9; // ! -
const PREC_CALL: u8 = 10; // . ()
const PREC_PRIMARY: u8 = 11;

//Define the Parser
#[derive(Debug)]
pub struct Parser {
    previous_token: Option<scanner::Token>,
    current_token: Option<scanner::Token>,
    had_error: bool,
    // set the panic mode to supress other errors
    panic_mode: bool,
}

impl Parser {
    // remember, parser.current is a token!
    // so parser.current.start is a valid statement

    pub fn init_parser() -> Parser {
        Parser {
            previous_token: None,
            current_token: None,
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self, source: &str, scanner: &mut scanner::Scanner, chunk: &mut Chunk) {
        self.previous_token = match &self.current_token {
            Some(x) => Some(x.to_owned()),
            None => None,
        };

        loop {
            self.current_token = Some(scanner.scan_token(source));
            match &self.current_token {
                Some(x) => match x.kind {
                    scanner::TokenKind::TokenError => {
                        self.error_at_current("Error encountered passing through to advance");
                    }
                    _ => break,
                },
                None => {
                    // Do nothing
                }
            }
        }
    }

    fn consume(
        &mut self,
        source: &str,
        token_kind: scanner::TokenKind,
        msg: &str,
        scanner: &mut scanner::Scanner,
        chunk: &mut Chunk,
    ) {
        match &self.current_token {
            Some(x) => match &x.kind {
                token_kind => {
                    self.advance(source, scanner, chunk);
                    return;
                }
            },
            None => {
                eprintln!("None in consume")
            }
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at("current", message);
    }

    fn error_at_prev(&mut self, message: &str) {
        self.error_at("previous", message);
    }

    fn error_at(&mut self, tok: &str, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprintln!(
            "{}",
            format!("[line {}] Error", self.current_token.as_ref().unwrap().line)
        );
        match tok {
            "current" => match &self.current_token {
                Some(x) => match x.kind {
                    scanner::TokenKind::TokenEof => {
                        eprintln!(" at the end of the source code.");
                    }
                    scanner::TokenKind::TokenError => {}
                    _ => {
                        eprintln!(
                            "{}",
                            format!(" at col {} to {}", x.start, x.start + x.length)
                        );
                    }
                },
                None => {}
            },
            "previous" => match &self.current_token {
                Some(x) => match &x.kind {
                    scanner::TokenKind::TokenEof => {
                        eprintln!(" at the end of the source code.");
                    }
                    scanner::TokenKind::TokenError => {}
                    _ => {
                        eprintln!(
                            "{}",
                            format!(" at col {} to {}", x.start, x.start + x.length)
                        );
                    }
                },
                None => {}
            },
            _ => {
                eprintln!("unreachable state in error_At")
            }
        }
        eprintln!("{}", format!(" :{}", message));
        self.had_error = true;
    }

    fn number(&self, source: &str, chunk: &mut Chunk) {
        match self.previous_token.to_owned() {
            Some(x) => {
                let value = source.get(x.start..=x.start + x.length - 1).unwrap();
                let cons = f64::from_str(value);
                match cons {
                    Ok(y) => {
                        let mut byte = chunk.add_constant(y);
                        // println!("{:?}", byte);
                        self.emit_byte(chunk, byte);
                    }
                    Err(..) => {
                        eprintln!("float conversion error");
                    }
                }
            }
            None => {
                eprintln!("Exception in number()");
            }
        }
    }

    fn parse_precedence(
        &mut self,
        precede: u8,
        source: &str,
        scanner: &mut scanner::Scanner,
        chunk: &mut Chunk,
    ) {
        self.advance(source, scanner, chunk);
        let owner = self.previous_token.to_owned().unwrap().kind;
        let (prefix, _, _) = parse_rule(owner);
        match prefix {
            "none" => self.error_at_prev("Expect expression."),
            "unary" => self.unary(source, scanner, chunk),
            "grouping" => self.grouping(source, scanner, chunk),
            "number" => self.number(source, chunk),
            _ => self.error_at_prev("This is not a valid token"),
        }
        loop {
            let owner = self.current_token.to_owned().unwrap().kind;
            let (_, infix, prec) = parse_rule(owner);

            if precede > prec {
                break;
            }

            self.advance(source, scanner, chunk);

            match infix {
                "none" => {
                    // self.error_at_current("Exception in parse_precedence");
                }
                "binary" => {
                    self.binary(source, scanner, chunk);
                }
                _ => {}
            }
        }
    }

    fn emit_byte(&self, chunk: &mut Chunk, byte: OpCode) {
        chunk.write_chunk(byte, self.previous_token.as_ref().unwrap().line);
    }

    fn emit_return(&self, chunk: &mut Chunk) {
        self.emit_byte(chunk, OpCode::OpReturn);
    }

    fn expression(&mut self, source: &str, scanner: &mut scanner::Scanner, chunk: &mut Chunk) {
        self.parse_precedence(PREC_ASSIGNMENT, source, scanner, chunk);
    }

    fn unary(&mut self, source: &str, scanner: &mut scanner::Scanner, chunk: &mut Chunk) {
        let token_kind = self.previous_token.to_owned().unwrap().kind;
        self.parse_precedence(PREC_UNARY, source, scanner, chunk);

        if token_kind == scanner::TokenKind::TokenMinus {
            self.emit_byte(chunk, OpCode::OpNegate);
        } else {
            return;
        }
    }

    fn binary(&mut self, source: &str, scanner: &mut scanner::Scanner, chunk: &mut Chunk) {
        let token_kind = self.previous_token.to_owned().unwrap().kind;
        let (prefix, infix, prec) = parse_rule(token_kind.to_owned());
        self.parse_precedence(prec + 1, source, scanner, chunk);

        match token_kind {
            scanner::TokenKind::TokenPlus => self.emit_byte(chunk, OpCode::OpAdd),
            scanner::TokenKind::TokenMinus => self.emit_byte(chunk, OpCode::OpSubtract),
            scanner::TokenKind::TokenSlash => self.emit_byte(chunk, OpCode::OpDivide),
            scanner::TokenKind::TokenStar => self.emit_byte(chunk, OpCode::OpMultiply),
            _ => return,
        }
    }

    fn grouping(&mut self, source: &str, scanner: &mut scanner::Scanner, chunk: &mut Chunk) {
        self.expression(source, scanner, chunk);
        self.consume(
            source,
            scanner::TokenKind::TokenRightParen,
            "exprected ')' after expression",
            scanner,
            chunk,
        );
    }
}

pub fn compile(
    source: &str,
    chunk: &mut Chunk,
    parser: &mut Parser,
    scanner: &mut scanner::Scanner,
) -> bool {
    parser.advance(source, scanner, chunk);
    parser.expression(source, scanner, chunk);

    // println!("{:?}", parser);

    parser.consume(
        source,
        scanner::TokenKind::TokenEof,
        "Expected end of expression in compile",
        scanner,
        chunk,
    );
    parser.emit_return(chunk);
    if std::env::args().any(|x| x == "debug_build") {
        disassemble_chunk(chunk, "Code");
    }
    !parser.had_error
}

fn parse_rule(owner: scanner::TokenKind) -> (&'static str, &'static str, u8) {
    match owner {
        scanner::TokenKind::TokenLeftParen => return ("grouping", "none", PREC_NONE),
        scanner::TokenKind::TokenPlus => return ("none", "binary", PREC_TERM),
        scanner::TokenKind::TokenMinus => return ("unary", "binary", PREC_TERM),
        scanner::TokenKind::TokenSlash => return ("none", "binary", PREC_FACTOR),
        scanner::TokenKind::TokenStar => return ("none", "binary", PREC_FACTOR),
        scanner::TokenKind::TokenNumber => return ("number", "none", PREC_NONE),
        _ => return ("none", "none", PREC_NONE),
    }
}
