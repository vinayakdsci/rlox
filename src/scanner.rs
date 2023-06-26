
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

    TokenError,
    TokenEof,
}

#[derive(Debug, Clone)]
pub struct Token{
    pub kind: TokenKind,
    pub length: usize,
    pub start: usize,
    pub line: i32,
}

#[derive(Debug, Clone)]
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

    pub fn advance<'a>(&mut self, source: &'a str) -> &'a str {
        self.current += 1;
        // println!("{}", self.current);
        source.get(self.current - 1..self.current).unwrap()
    }

    fn match_with(&mut self, source: &str, expected: char, length: usize) -> bool {
        if self.current == length {
            return false;
        }
        if expected != self.peek(source).unwrap() {
            return false;
        }

        self.current += 1;
        true
    }

    fn skip_whitespaces(&mut self, source: &str) {
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

    fn peek(&mut self, source: &str) -> Option<char> {
        if source.get(self.current..=self.current).is_none() {
            None
        }
        else {
            Some(
                source
                .get(self.current..=self.current)
                .unwrap()
                .chars().nth(0)
                .unwrap(),
            )
        }
    }

    fn peek_next(&mut self, source: &str) -> Option<char> {

        if source.get(self.current + 1..=self.current + 1).is_none() {
            println!("cursor at {}", self.current);
            None
        }
        else {
            Some(
                source
                .get(self.current + 1..=self.current+1)
                .unwrap()
                .chars().nth(0)
                .unwrap(),
            )
        }
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        Token {
            kind,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    pub  fn error_token(&mut self, message: &str) -> Token {
        eprintln!("{}", format!("Error encountered at {}, col{} to {}", self.line, self.start, self.current));
        Token {
            kind: TokenKind::TokenError,
            start: 0,
            length: message.len(),
            line: self.line,
        }
    }

    fn char_at_start(&self, source: & str) -> Option<char> {
        match source.get(self.start + 1..=self.start + 1) {
            Some(x) => return x.chars().nth(0),
            None => return None,
        }
    }

    fn identifier_type(&mut self, source: & str) -> TokenKind {
        //build the trie
        match source.get(self.start..=self.start) {
            Some(x) => {
                let y = x.chars().nth(0).unwrap();
                match y {
                    'a' => return self.check_keyword(1, 2, "nd", TokenKind::TokenAnd, source),
                    'c' => return self.check_keyword(1, 4, "lass", TokenKind::TokenClass, source),
                    'e' => return self.check_keyword(1, 3, "lse", TokenKind::TokenElse, source),
                    'i' => return self.check_keyword(1, 1, "f", TokenKind::TokenIf, source),
                    'n' => return self.check_keyword(1, 2, "il", TokenKind::TokenNil, source),
                    'o' => return self.check_keyword(1, 1, "r", TokenKind::TokenOr, source),
                    'p' => return self.check_keyword(1, 4, "rint", TokenKind::TokenPrint, source),
                    'r' => return self.check_keyword(1, 5, "eturn", TokenKind::TokenReturn, source),
                    's' => return self.check_keyword(1, 4, "uper", TokenKind::TokenSuper, source),
                    'w' => return self.check_keyword(1, 4, "hile", TokenKind::TokenWhile, source),
                    // trie now branches
                    'f' => {
                        if self.current - self.start > 1 {
                            match self.char_at_start(source) {
                                Some(x) => {
                                    match x {
                                        'o' => return self.check_keyword(2, 1, "r", TokenKind::TokenFor, source),
                                        'u' => return self.check_keyword(2, 1, "n", TokenKind::TokenFun, source),
                                        'a' => return self.check_keyword(2, 3, "lse", TokenKind::TokenFalse, source),
                                        _  => {
                                            println!("No match for f");
                                            let id = source.get(self.start..self.current).unwrap().to_string();
                                            return TokenKind::TokenIdentifier;
                                        }
                                    };
                                },
                                None => {
                                    return TokenKind::TokenError;
                                }
                            }
                        } else { 
                            let id = source.get(self.start..self.current).unwrap().to_string();
                            return TokenKind::TokenIdentifier;
                        }
                    }
                    't' =>  {
                        if self.current - self.start > 1 {
                            match self.char_at_start(source) {
                                Some(x) => {
                                    match x {
                                        'h' => return self.check_keyword(2, 2, "is", TokenKind::TokenThis, source),
                                        'r' => return self.check_keyword(2, 2, "ue", TokenKind::TokenTrue, source),
                                        _  => return TokenKind::TokenIdentifier,
                                    };
                                },
                                None => {
                                    return TokenKind::TokenError;
                                }
                            }
                        } else {
                            return TokenKind::TokenIdentifier;
                        }
                    }
                    _ => {
                        // println!("{}", y)
                        return TokenKind::TokenIdentifier;
                    }
                }
            },
            None => {
                return TokenKind::TokenError;
            }
        }
        // TokenKind::TokenIdentifier
    }

    fn check_keyword(&self, start: usize, length: usize, rest: & str, token_kind: TokenKind, source: & str) -> TokenKind {
        if self.current - self.start == start + length
            && source.get(self.start + start..self.start + start + length).unwrap() == rest 
        {
            return token_kind;
        }
        let id = source.get(self.start..self.current).unwrap().to_string();
        TokenKind::TokenIdentifier
    }

    fn number(&mut self, source: & str) -> Token {
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

        //check if there is no space making an invalid identifier
        if self.peek(source).is_some()
            && (self.peek(source).unwrap().is_ascii_alphabetic() 
                || self.peek(source).unwrap() == '_')
        {
            return self.error_token("Error: cannot start an identifier with a number!");
        }
        return self.make_token(TokenKind::TokenNumber);
    }

    fn identifier(&mut self, source: & str) -> Token {
        while self.peek(source).is_some() 
            && (self.peek(source).unwrap().is_ascii_alphanumeric() 
                || self.peek(source).unwrap() == '_') 
            {
                self.current += 1;
            }
        // borrow rules
        let token_for_id = self.identifier_type(source);
        self.make_token(token_for_id)
    }

    fn string(&mut self, source: & str) -> Token {
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
    pub fn scan_token(&mut self, source: &str) -> Token {

        let length = source.len();
        self.skip_whitespaces(source);
        self.start = self.current;

        // println!("Source length {length}");
        // println!("Scanner at {}", scanner.current);

        if self.current == length {
            return self.make_token(TokenKind::TokenEof);
        }
        let c = self.advance(source).chars().nth(0).unwrap();

        // match for identifiers
        if c.is_ascii_alphabetic() || c == '_' {
            return self.identifier(source) 
        }
        if c.is_ascii_digit() {
            return self.number(source) 
        }

        // println!("Matched char {}", c);
        match c {
            '(' => return self.make_token(TokenKind::TokenLeftParen),
            ')' => return self.make_token(TokenKind::TokenRightParen),
            '{' => return self.make_token(TokenKind::TokenLeftBrace),
            '}' => return self.make_token(TokenKind::TokenRightBrace),
            ';' => return self.make_token(TokenKind::TokenSemiColon),
            ',' => return self.make_token(TokenKind::TokenComma),
            '.' => return self.make_token(TokenKind::TokenPeriod),
            '-' => return self.make_token(TokenKind::TokenMinus),
            '+' => return self.make_token(TokenKind::TokenPlus),
            '*' => return self.make_token(TokenKind::TokenStar),
            '!' => if self.match_with(source, '=', length) { return self.make_token(TokenKind::TokenBangEqual) }    else {return self.make_token(TokenKind::TokenBang)},
            '=' => if self.match_with(source, '=', length) { return self.make_token(TokenKind::TokenEqualEqual) }   else {return self.make_token(TokenKind::TokenEqual)},
            '<' => if self.match_with(source, '=', length) { return self.make_token(TokenKind::TokenLessEqual) }    else {return self.make_token(TokenKind::TokenLess)},
            '>' => if self.match_with(source, '=', length) { return self.make_token(TokenKind::TokenGreaterEqual) } else {return self.make_token(TokenKind::TokenGreater)},
            '"' => self.string(source),
            _  => self.error_token("Unexpected character encountered."),
        }


    }


}


