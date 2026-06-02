mod lexer;
mod repl;
mod token;

use std::io;

use repl::start;

fn main() -> io::Result<()> {
  let username = whoami::username().unwrap_or("user".to_string());

  println!("Hello {username}! This is the Monkey programming language!");
  println!("Feel free to type in commands");

  start(io::stdin().lock(), io::stdout().lock())
}
