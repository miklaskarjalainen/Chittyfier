#![allow(unused_imports)]

mod lexer;
mod value;
mod transpiler;

use std::env;
use std::fs;

fn main() {
    let filename: String;
    if env::args().count() > 1 {
        filename = env::args().nth(1).unwrap();
    }
    else {
        if cfg!(debug_assertions) {
            filename = String::from("./main.c");
            println!("Debug!");
        } else {
            println!("No input file was given!");
            return;
        }
    }

    // Read the file and lex the file
    let file_contents = fs::read_to_string(filename).expect("unable to open file");
    let tokens = lexer::Lexer::lex(file_contents);
    
    // Prints the Lexer tokens on debug builds.
    #[cfg(debug_assertions)]
    println!("{:#?}", tokens);

    // Transpile
    transpiler::transpile(&tokens);
}

