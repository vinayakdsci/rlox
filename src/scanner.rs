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
    TokenWhile,

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
        source.get(self.current - 1..self.current).unwrap()
    }
    
    fn match_with(&mut self, source: &String, expected: char, length: usize) -> bool {
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
                return;
            }
            let x = self.peek(source).unwrap();
            match x {
                ' ' | '\r' | '\t' => {
                    self.current += 1;
                },
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
            Some(
                source
                .get(self.current..=self.current)
                .unwrap()
                .chars()
                .nth(0)
                .unwrap(),
            )
        }
    }

    fn peek_next(&mut self, source: &String) -> Option<char> {

        if source.get(self.current + 1..=self.current + 1).is_none() {
            None
        }
        else {
            Some(
                source
                .get(self.current + 1..=self.current+1)
                .unwrap()
                .chars()
                .nth(0)
                .unwrap(),
            )
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

    fn next_char(&self, source: &String) -> Option<char> {
        source.get(self.current + 1..=self.current + 1).unwrap().chars().nth(0)
    }

    fn identifier_type(&mut self, source: &String) -> TokenKind {
        //build the trie
        match source.get(self.start..=self.start) {
            Some(x) => {
                let y = x.chars().nth(0).unwrap();
                match y {
                    'a' => return self.check_keyword(1, 2, "nd", TokenKind::TokenAnd, &source),
                    'c' => return self.check_keyword(1, 4, "lass", TokenKind::TokenClass, &source),
                    'e' => return self.check_keyword(1, 3, "lse", TokenKind::TokenElse, &source),
                    'i' => return self.check_keyword(1, 1, "f", TokenKind::TokenIf, &source),
                    'n' => return self.check_keyword(1, 2, "il", TokenKind::TokenNil, &source),
                    'o' => return self.check_keyword(1, 1, "r", TokenKind::TokenOr, &source),
                    'p' => return self.check_keyword(1, 4, "rint", TokenKind::TokenPrint, &source),
                    'r' => return self.check_keyword(1, 5, "eturn", TokenKind::TokenReturn, &source),
                    's' => return self.check_keyword(1, 4, "uper", TokenKind::TokenSuper, &source),
                    'w' => return self.check_keyword(1, 4, "hile", TokenKind::TokenWhile, &source),
                    _ => {
                        println!("{}", y);
                        return TokenKind::TokenIdentifier;
                    }
                }
            },
            None => {
                return TokenKind::TokenError(String::from("Ran out of characters in identifier_type()"));
            }
        }
        // TokenKind::TokenIdentifier
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_kind: TokenKind, source: &String) -> TokenKind {
        if self.current - self.start == start + length
            && source.get(self.start + start..self.start + start + length).unwrap() == rest 
        {
            return token_kind;
        }
        TokenKind::TokenIdentifier
    }

    fn number(&mut self, source: &String) -> Token {
        while self.peek(source).is_some() && self.peek(source).unwrap().is_ascii_digit() {
            self.current += 1;
        }

        if self.peek(source).is_some() 
            && self.peek(source).unwrap() == '.' 
            && self.peek_next(source).unwrap().is_ascii_digit() 
        {
               self.current += 1;
               while self.peek(source).is_some() && self.peek(source).unwrap().is_ascii_digit() {
                   self.current += 1;
               }
        }
        self.make_token(TokenKind::TokenNumber)
    }

    fn identifier(&mut self, source: &String) -> Token {
        while self.peek(source).is_some() 
            && (self.peek(source).unwrap().is_ascii_alphanumeric() 
                || self.peek(source).unwrap() == '_') 
        {
                self.current += 1;
        }
        // borrow rules
        let token_for_id = self.identifier_type(&source);
        self.make_token(token_for_id)
    }

    fn string(&mut self, source: &String) -> Token {
        //consume chars till another '"' is encountered, take care of newlines
        let poss_line = self.line;
        let poss_col = self.current;
        while self.peek(source).is_some() && self.peek(source).unwrap() != '"' {
            if self.peek(source).unwrap() == '\n' { 
                self.line += 1; 
            }
            self.current += 1;
        }
        
        if self.peek(source).is_none() {
            println!("Unterminated String {} col {} error:", poss_line, poss_col);
            return self.error_token("Unterminated String encountered.");
        }

        //consume the closing quote!
        self.current += 1;
        self.make_token(TokenKind::TokenString)
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
    if c.is_ascii_alphabetic() || c == '_' {
        return scanner.identifier(&iter_over_source) 
    }
    if c.is_ascii_digit() {
        return scanner.number(&iter_over_source) 
    }

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
        '!' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenBangEqual) }    else {return scanner.make_token(TokenKind::TokenBang)},
        '=' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenEqualEqual) }   else {return scanner.make_token(TokenKind::TokenEqual)},
        '<' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenLessEqual) }    else {return scanner.make_token(TokenKind::TokenLess)},
        '>' => if scanner.match_with(&iter_over_source, '=', length) { return scanner.make_token(TokenKind::TokenGreaterEqual) } else {return scanner.make_token(TokenKind::TokenGreater)},
        '"' => return scanner.string(&iter_over_source),
         _  => return scanner.error_token("Unexpected character encountered."),
    }


}

