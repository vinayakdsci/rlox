use std::str::Chars;

#[derive(Debug, PartialEq)]
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

    pub fn advance<'a>(&'a mut self, source: &'a String) -> &str {
        self.current += 1;
        // println!("{}", self.current);
        source.get(self.current-1..self.current).unwrap()
    }
    
    pub fn match_with(&mut self, source: &String,  expected: char, length: usize) -> bool {
        if self.current == length {
            return false;
        }
        if expected != self.peek(source).unwrap() {
            return false;
        }

        self.current += 1;
        true
    }

    fn skip_whitespaces(&mut self, source: &String) {
        loop {
            // let x = source.nth(self.current).unwrap();
            if self.current == source.len() {
                return
            }
            let x = self.peek(source).unwrap();
            match x {
                ' ' | '\r' | '\t' => {self.current += 1;},
                '\n' => {
                    self.line += 1;
                    self.current += 1;
                },
                '#' => {
                    while self.current < source.len() && self.peek(source).unwrap() != '\n' {
                        self.current += 1;
                    }
                }
                _ => return,
            }
        } 
    }

    fn peek(&mut self, source: &String) -> Option<char> {
        if source.get(self.current..=self.current).is_none() {
            None
        }
        else {
            Some(source.get(self.current..=self.current).unwrap().chars().nth(0).unwrap())
        }
    }

    fn peek_next(&mut self, source: &String) -> Option<char> {

        if source.get(self.current + 1..=self.current+1).is_none() {
            None
        }
        else {
            Some(source.get(self.current + 1..=self.current+1).unwrap().chars().nth(0).unwrap())
        }
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let token = Token {
            kind: kind,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        };
        token
    }

    fn error_token(&mut self, message: &str) -> Token {
        let mut token = Token {
            kind: TokenKind::TokenError(String::from(message)),
            start: 0,
            length: message.len(),
            line: self.line,
        };
        token
    }

    fn identifier_type(&mut self) -> TokenKind {
        TokenKind::TokenIdentifier
    }

    fn number(&mut self, source: &String) -> Token {
        while self.peek(source).is_some() && self.peek(source).unwrap().is_ascii_digit() {
            self.current += 1;
        }

        if self.peek(source).is_some() && self.peek(source).unwrap() == '.' && self.peek_next(source).unwrap().is_ascii_digit() {
               self.current += 1;
               while self.peek(source).is_some() && self.peek(source).unwrap().is_ascii_digit() {
                   self.current += 1;
               }
        }
        self.make_token(TokenKind::TokenNumber)
    }

    fn identifier(&mut self, source: &String) -> Token {
        while self.peek(source).is_some() && (self.peek(source).unwrap().is_ascii_alphanumeric() || self.peek(source).unwrap() == '_') {
            self.current += 1;
        }
        // borrow rules
        let token_for_id = self.identifier_type();
        self.make_token(TokenKind::TokenIdentifier)
    }

}


pub fn scan_token(mut scanner: &mut Scanner, source: &String) -> Token {

    let length = source.len();
    let mut iter_over_source = source;
    scanner.skip_whitespaces(&iter_over_source);
    scanner.start = scanner.current;

    // println!("Source length {length}");
    // println!("Scanner at {}", scanner.current);

    if scanner.current == length {
        return scanner.make_token(TokenKind::TokenEof);
    }
    let c = scanner.advance(&iter_over_source).chars().nth(0).unwrap();

    // match for identifiers
    if c.is_ascii_alphabetic() || c == '_' { return scanner.identifier(&iter_over_source) };
    if c.is_ascii_digit() { return scanner.number(&iter_over_source) };

    // println!("Matched char {}", c);
    match c {
        '(' => return scanner.make_token(TokenKind::TokenLeftParen),
        ')' => return scanner.make_token(TokenKind::TokenRightParen),
        '{' => return scanner.make_token(TokenKind::TokenLeftBrace),
        '}' => return scanner.make_token(TokenKind::TokenRightBrace),
        ';' => return scanner.make_token(TokenKind::TokenSemiColon),
        ',' => return scanner.make_token(TokenKind::TokenComma),
        '.' => return scanner.make_token(TokenKind::TokenPeriod),
        '-' => return scanner.make_token(TokenKind::TokenMinus),
        '+' => return scanner.make_token(TokenKind::TokenPlus),
        '*' => return scanner.make_token(TokenKind::TokenStar),
        '!' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenBangEqual) } else {return scanner.make_token(TokenKind::TokenBang)},
        '=' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenEqualEqual) } else {return scanner.make_token(TokenKind::TokenEqual)},
        '<' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenLessEqual) } else {return scanner.make_token(TokenKind::TokenLess)},
        '>' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenGreaterEqual) } else {return scanner.make_token(TokenKind::TokenGreater)},
        _ =>  return scanner.error_token("Unexpected character encountered.")
    }


}

