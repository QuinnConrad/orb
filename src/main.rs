use std::env;
use std::fs;
use orb::{
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
    println!("{}", args[1]);
    let file_path = &args[1];
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let tokens = lexer.tokenize(&input);
    if let Ok(tokens) = tokens {
      let mut parser = Parser::new(tokens);
      let stmts = parser.parse_program();
      if let Ok(stmts) = stmts {
        for s in stmts {
          println!("{s:?}");
        }
      }
      else {
        println!("hit an err: {stmts:?}");
      }
      //println!("matched something!");
      //println!("{}", tokens.len());
      //for token in tokens {
      //    println!("{token:?}");
      //}
    }
    else {
      println!("hit an err: {tokens:?}");
    }
}
