use std::fs::{File};
use std::io::Write;
use std::error::Error;

pub enum Token {
    Plus,     // +
    Minus,    // -
    Less,     // <
    Greater,  // >
    LBrace,   // [
    RBrace,   // ]
    Dot,      // .
    Comma     // ,
}

pub fn lex(source: &String) -> Vec<Token> {
    let mut ret = Vec::new();

    for ch in source.chars() {
        match ch {
            '+' => ret.push(Token::Plus),
            '-' => ret.push(Token::Minus),
            '<' => ret.push(Token::Less),
            '>' => ret.push(Token::Greater),
            '[' => ret.push(Token::LBrace),
            ']' => ret.push(Token::RBrace),
            '.' => ret.push(Token::Dot),
            ',' => ret.push(Token::Comma),
            _ => {}
        }
    }

    ret
}

fn write(f: &mut File, indent: u8, text: &str) {
    let _ = writeln!(f, "{}{text}", "    ".repeat(indent as usize));
}

pub fn build_c(tokens: &Vec<Token>, name: &str) -> Result<(), Box<dyn Error>> {
    let mut indent = 1u8;

    let mut f = File::create_new(name)?;

    // base
    let _ = writeln!(f, "#include <stdio.h>\n");

    let _ = writeln!(f, "int main() {{");
    let _ = writeln!(f, "    char arr[256] = {{0}};");
    let _ = writeln!(f, "    unsigned char i = 0;\n");

    let _ = writeln!(f, "    // BRAINFUCK");

    for token in tokens {
        match token {
            Token::Plus => {
                write(&mut f, indent, "arr[i]++;");
            },
            Token::Minus => {
                write(&mut f, indent, "arr[i]--;");
            },
            Token::Less => {
                write(&mut f, indent, "i--;");
            },
            Token::Greater => {
                write(&mut f, indent, "i++;");
            },
            Token::LBrace => {
                write(&mut f, indent, "while (arr[i] > 0) {");
                indent += 1;
            },
            Token::RBrace => {
                write(&mut f, indent - 1, "}");
                indent -= 1;
            },
            Token::Dot => {
                write(&mut f, indent, "printf(\"%c\", arr[i]);");
            },
            Token::Comma =>{
                write(&mut f, indent, "arr[i] = getchar();");
            },
        }
    }

    let _ = writeln!(f, "}}");

    Ok(())
}