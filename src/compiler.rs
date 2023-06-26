use std::borrow::Borrow;
#[path = "scanner.rs"]
pub mod scanner;
use crate::chunk::Chunk;

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

    fn advance(&mut self, source: &str, scanner: &mut scanner::Scanner) {
        self.previous_token = match &self.current_token {
            Some(x) => {
                Some(x.to_owned())
            }
            None => {
                None
            }
        };

        loop {
            self.current_token = Some(scanner.scan_token(source));
            match &self.current_token {
                Some (x) => {
                    match x.kind {
                        scanner::TokenKind::TokenError => {
                            self.error_at_current("Error encountered passing through to advance");
                        }
                        _ => break,
                    }
                }
                None => {
                    // Do nothing
                }
            }
        }
    }

    fn consume(&mut self, source: &str, token_kind: scanner::TokenKind, msg: &str, scanner: &mut scanner::Scanner) {
        match &self.current_token {
            Some(x) => {
                match &x.kind {
                    token_kind => {
                        self.advance(source, scanner);
                        return;
                    }
                    _ => {
                        self.error_at_current(msg);
                    }
                }
            }
            None => {
                eprintln!("None in consume")
            } 
        }
    }
    // otherwise, throw error

    fn error_at_current(&mut self, message: &str) {
        self.error_at("current" ,message);
    }

    fn error_at_prev(&mut self, message: &str) {

        self.error_at("previous", message);
    }

    fn error_at(&mut self, tok: &str, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprintln!("{}", format!("[line {}] Error", self.current_token.as_ref().unwrap().line));
        match tok {
            "current"=> {
                match &self.current_token {
                    Some(x) => {
                        match x.kind {
                            scanner::TokenKind::TokenEof => {
                                eprintln!(" at the end of the source code.");
                            },
                            scanner::TokenKind::TokenError => {  },
                            _ => {
                                eprintln!("{}", format!(" at col {} to {}", x.start, x.start + x.length));
                            }
                        }
                    }
                    None => {  }
                }
            },
            "previous" => {
                match &self.current_token {
                    Some(x) => {
                        match &x.kind {
                            scanner::TokenKind::TokenEof => {
                                eprintln!(" at the end of the source code.");
                            },
                            scanner::TokenKind::TokenError => {  },
                            _ => {
                                eprintln!("{}", format!(" at col {} to {}", x.start, x.start + x.length));
                            }
                        }
                    }
                    None => {  }
                }
            }
            _ => {
                eprintln!("unreachable state in error_At")
            }
        }
        eprintln!("{}", format!(" :{}", message));
        self.had_error = true;
    }
}
pub fn compile(source: &str, chunk: &Chunk, parser: &mut Parser, scanner: &mut scanner::Scanner) -> bool {
    // the compiler is single pass, so init the parser here?
    parser.advance(source, scanner);
    // compile_expression();
    println!("{:?}", parser);
    parser.consume(source, scanner::TokenKind::TokenEof, "Expected end of expression in compile", scanner);

    !parser.had_error
}



