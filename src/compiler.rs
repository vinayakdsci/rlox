#[path = "scanner.rs"]
mod scanner;


pub fn compile(source: String) {
    let mut my_scanner = scanner::Scanner::init_scanner(); // Let the scanner kick in
    let mut line = -1;

    loop {
        let token: scanner::Token = scanner::scan_token(&mut my_scanner, &source);

        // find the line number of the next token if not on the same line
        if token.line != line {
            println!("     {}", token.line);
            line = token.line;
        } else {
            println!("    |  ")
        }
        // println!("{:?}", token);
        println!("{:?}, {:?}", token.kind, token.length);
        if token.kind == scanner::TokenKind::TokenEof {
            // println!("TokenEof");
            break;
        }
    }
}
