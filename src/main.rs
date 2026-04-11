mod lex;
mod token;

use lex::Lexer;

fn main() {
    let lexer = Lexer::new();
    let input = "echo -1 1 0.1 \"hello world\"\
                # i'm leaving a comment
                -0.1 perhaps otherwise";
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
