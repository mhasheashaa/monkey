use std::io;

use crate::{lexer::Lexer, token::TokenType};

const PROMPT: &str = ">> ";

pub fn start<R: io::BufRead, W: io::Write>(mut input: R, mut output: W) -> io::Result<()> {
  let mut line = String::new();
  loop {
    write!(output, "{PROMPT}")?;
    output.flush()?;

    line.clear();

    let bytes_read = input.read_line(&mut line)?;
    if bytes_read == 0 {
      return Ok(());
    }

    let mut lexer = Lexer::new(&line);

    loop {
      let tok = lexer.next_token();

      if tok.token_type == TokenType::Eof {
        break;
      }

      writeln!(output, "{tok:#?}")?;
    }
  }
}
