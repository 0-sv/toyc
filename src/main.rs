use std::{env, fs};

#[derive(Debug)]
enum TokenType {
    OpenBrace,
    CloseBrace
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_contents = fs::read_to_string(&args[1]).expect("Unable to read file");

    let mut tokens = vec![];
    for c in file_contents.chars() {
        match c {
            '{' => tokens.push(TokenType::OpenBrace),
            '}' => tokens.push(TokenType::CloseBrace),
            _ => {}
        }
    }

    println!("{:?}", tokens);
}
