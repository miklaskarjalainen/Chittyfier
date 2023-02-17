#![allow(unused_imports)]

mod lexer;
mod value;
mod transpiler;

use std::env;
use std::fs;

fn main() {
    let filename = "C:\\Users\\Auzer\\Documents\\c_code_transpiler\\main.c"; //env::args().nth(1).expect("no filename provided");
    let file_contents = fs::read_to_string(filename).expect("unable to open file");

    let tokens = lexer::Lexer::lex(file_contents);
    println!("{:#?}", tokens);
    transpiler::transpile(&tokens);

    /*
    println!("{:?}", file_contents);
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    
    let words: Vec<&str> = file_contents.split_whitespace().collect();
    for word in words {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    
    for (word, count) in word_counts {
        println!("{}: {}", word, count);
    }
    */
}

