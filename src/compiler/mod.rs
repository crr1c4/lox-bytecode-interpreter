use crate::scanner::Scanner;
// use crate::scanner::token::TokenKind;

pub fn compile(source: &String) {
    let mut scanner = Scanner::new(source);
    let mut line = -1;

    while let Some(token) = scanner.next() {
        if token.line != line {
            print!("{:>4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:>2} '{}'", token.kind as usize, token.source);
    }
}
