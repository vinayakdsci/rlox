#[path = "scanner.rs"]
mod scanner;
use crate::chunk::Chunk;

//Define the Parser
#[derive(Debug)]
pub struct Parser{
    previous_token: scanner::Token,
    current_token: scanner::Token,
    had_error: bool,
    // set the panic mode to supress other errors
    panic_mode: bool,
}

impl Parser {
    // remember, parser.current is a token!
    // so parser.current.start is a valid statement

    fn init_parser() -> Parser {
        Parser {
            previous_token: {
                scanner::Token {
                    kind: scanner::TokenKind::TokenEof,
                    length: 0,
                    start: 0,
                    line: -1,
                }
            },
            current_token: {
                scanner::Token {
                    kind: scanner::TokenKind::TokenEof,
                    length: 0,
                    start: 0,
                    line: -1,
                }
            },
            had_error: false,
            panic_mode: false,
        }
    }

}

pub fn compile(source: &str, chunk: &Chunk) -> bool {
    // the compiler is single pass, so init the parser here?
    let mut scanner = scanner::Scanner::init_scanner();
    let mut parser = Parser::init_parser();
    advance(&mut parser, &mut scanner, source);
    // compile_expression();
    consume(&mut parser, &mut scanner, source, scanner::TokenKind::TokenEof, "Expected end of expression");

    !parser.had_error
}

fn error_at_current(parser: &mut Parser, message: &str) {
    let tok = parser.current_token.clone();
    error_at(parser, tok, message);
}

fn error_at_prev(parser: &mut Parser, message: &str) {
    let tok = parser.previous_token.clone();
    error_at(parser, tok, message);
}

fn error_at(parser: &mut Parser, tok: scanner::Token, message: &str) {
    if parser.panic_mode {
        return;
    }
    eprintln!("{}", format!("[line {}] Error", parser.current_token.line));
    match tok.kind {
        scanner::TokenKind::TokenEof => {
            eprintln!(" at the end of the source code.");
        },
        scanner::TokenKind::TokenError(msg) => {
            eprintln!("{}", format!("{}", msg));
        },
        _ => {
            eprintln!("{}", format!(" at col {} to {}", tok.start, tok.start + tok.length));
        }
    }
    eprintln!("{}", format!(" :{}", message));
    parser.had_error = true;
}


fn advance(parser: &mut Parser, scanner: &mut scanner::Scanner, source: &str) {
    let copy = parser.current_token.clone();
    parser.previous_token = copy;

    loop {
        parser.current_token = scanner::scan_token(scanner, source);
        let fool = parser.current_token.kind.clone();
        match fool {
            scanner::TokenKind::TokenError(msg) => {
                error_at_current(parser, msg.get(0..).unwrap());
            }
            _ => break,
        }
    }
}

fn consume(parser: &mut Parser, scanner: &mut scanner::Scanner, source: &str, token_kind: scanner::TokenKind, msg: &str) {
    if parser.current_token.kind == token_kind {
        advance(parser, scanner, source);
    } else {
        return;
    }
    // otherwise, throw error
    error_at_current(parser, msg);
}

