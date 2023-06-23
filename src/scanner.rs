use std::str::Chars;

#[derive(PartialEq)]
pub enum TokenKind {
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenPeriod,
    TokenMinus,
    TokenPlus,
    TokenSemiColon,
    TokenSlash,
    TokenStar,

    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,

    TokenIdentifier,
    TokenString,
    TokenNumber,

    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFun,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenSuper,
    TokenThis,
    TokenTrue,
    TokenVar,
    Tokenwhile,

    TokenError(String),
    TokenEof,
}

// #[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
    pub start: usize,
    pub line: i32,
}

#[derive(Debug)]
pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: i32,
}


impl Scanner {
    pub fn init_scanner() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn advance(&mut self, source: &mut Chars<'_>) -> char {
        self.current += 1;
        println!("{:?}", source);
        source.nth(self.current - 1).unwrap()
    }
    
    pub fn match_with(&mut self, source: &mut Chars<'_>,  expected: char, length: usize) -> bool {
        if self.current == length - 1 {
            return false;
        }
        if expected != source.nth(self.current).unwrap() {
            return false;
        }

        self.current += 1;
        true
    }

    fn skip_whitespaces(&mut self, source: &mut Chars<'_>) {
        loop {
            let x = source.nth(self.current).unwrap();

            match x {
                ' ' | '\r' | '\t' => {self.current += 1;},
                '\n' => {
                    self.line += 1;
                    self.current += 1;
                },
                '#' => {
                    while source.next().unwrap() != '\n' {
                        self.current += 1;
                    }
                }
                _ => return,
            }
        } 
    }
}


pub fn scan_token(mut scanner: &mut Scanner, source: &String) -> Token {
    let mut iter_over_source = source.chars();
    scanner.skip_whitespaces(&mut iter_over_source);
    scanner.start = scanner.current;

    let length = source.len();
    println!("Source length {length}");
    println!("Scanner at {}", scanner.current);
    if scanner.current == length - 1 {
        return make_token(&mut scanner, TokenKind::TokenEof);
    }
    let c = scanner.advance(&mut iter_over_source);

    match c {
        '(' => return make_token(&mut scanner, TokenKind::TokenLeftParen),
        ')' => return make_token(&mut scanner, TokenKind::TokenRightParen),
        '{' => return make_token(&mut scanner, TokenKind::TokenLeftBrace),
        '}' => return make_token(&mut scanner, TokenKind::TokenRightBrace),
        ';' => return make_token(&mut scanner, TokenKind::TokenSemiColon),
        ',' => return make_token(&mut scanner, TokenKind::TokenComma),
        '.' => return make_token(&mut scanner, TokenKind::TokenPeriod),
        '-' => return make_token(&mut scanner, TokenKind::TokenMinus),
        '+' => return make_token(&mut scanner, TokenKind::TokenPlus),
        '*' => return make_token(&mut scanner, TokenKind::TokenStar),
        '!' => if scanner.match_with(&mut iter_over_source, '=', length) { return make_token(&mut scanner,TokenKind::TokenBangEqual) } else {return make_token(&mut scanner, TokenKind::TokenBang)},
        '=' => if scanner.match_with(&mut iter_over_source, '=', length) { return make_token(&mut scanner,TokenKind::TokenEqualEqual) } else {return make_token(&mut scanner, TokenKind::TokenEqual)},
        '<' => if scanner.match_with(&mut iter_over_source, '=', length) { return make_token(&mut scanner,TokenKind::TokenLessEqual) } else {return make_token(&mut scanner, TokenKind::TokenLess)},
        '>' => if scanner.match_with(&mut iter_over_source, '=', length) { return make_token(&mut scanner, TokenKind::TokenGreaterEqual) } else {return make_token(&mut scanner, TokenKind::TokenGreater)},
        _ =>  return error_token(&mut scanner, "Unexpected character encountered.")
    }


}



pub fn make_token(scanner: &mut Scanner, kind: TokenKind) -> Token {
    let token = Token {
        kind: kind,
        start: scanner.start,
        length: scanner.current - scanner.start,
        line: scanner.line,
    };
    token
}

pub fn error_token(scanner: &mut Scanner, message: &str) -> Token {
    let mut token = Token {
        kind: TokenKind::TokenError(String::from(message)),
        start: 0,
        length: message.len(),
        line: scanner.line,
    };
    token
}

