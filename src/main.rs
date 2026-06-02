mod lexer;
mod repl;
mod token;

use std::io;

use repl::start;

fn main() -> io::Result<()> {
  start(io::stdin().lock(), io::stdout().lock())
}
