mod lex;
mod token;

use lex::Lexer;

fn main() {
    let lexer = Lexer::new();
    let input = "echo 111 3.14 \"hello world\"";
    let tokens = lexer.tokenize(input);
    if let Ok(tokens) = tokens {
      println!("matched something!");
      println!("{}", tokens.len());
      for token in tokens {
          println!("{:?}", token);
      }
    }
    else {
      println!("hit an err: {:?}", tokens);
    }
}
