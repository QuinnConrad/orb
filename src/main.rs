pub mod ast;
pub mod lex;
pub mod token;
pub mod parser;

use std::env;
use std::fs;
use crate::{
  lex::Lexer,
  parser::Parser,
};

fn main() {
    let lexer = Lexer::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: no file passed");
        return;
    }
    println!("{}", &args[1]);
    let file_path = &args[1];
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let tokens = lexer.tokenize(&input);
    if let Ok(tokens) = tokens {
      let _parser = Parser::new(tokens.clone());
      println!("matched something!");
      println!("{}", tokens.len());
      for token in tokens {
          println!("{token:?}");
      }
    }
    else {
      println!("hit an err: {tokens:?}");
    }
}
