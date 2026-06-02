use crate::token::*;

pub struct Lexer {
  input: String,
  position: usize,
  read_position: usize,
  ch: Option<char>,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut l = Lexer {
      input: input.to_string(),
      position: usize::default(),
      read_position: usize::default(),
      ch: None,
    };
    l.read_char();
    l
  }

  pub fn next_token(&mut self) -> Token {
    let tok: Token;

    self.skip_whitespace();

    if let Some(ch) = self.ch {
      match ch {
        '=' => {
          if let Some(next_ch) = self.peek_char()
            && next_ch == '='
          {
            self.read_char();

            let mut literal = String::from(ch);
            if let Some(c) = self.ch {
              literal.push(c);
            }

            tok = Token {
              token_type: TokenType::Eq,
              literal,
            };
          } else {
            tok = Token {
              token_type: TokenType::Assign,
              literal: ch.to_string(),
            };
          }
        }
        '+' => {
          tok = Token {
            token_type: TokenType::Plus,
            literal: ch.to_string(),
          };
        }
        '-' => {
          tok = Token {
            token_type: TokenType::Minus,
            literal: ch.to_string(),
          };
        }
        '!' => {
          if let Some(next_ch) = self.peek_char()
            && next_ch == '='
          {
            self.read_char();

            let mut literal = String::from(ch);
            if let Some(c) = self.ch {
              literal.push(c);
            }

            tok = Token {
              token_type: TokenType::NotEq,
              literal,
            };
          } else {
            tok = Token {
              token_type: TokenType::Bang,
              literal: ch.to_string(),
            };
          }
        }
        '/' => {
          tok = Token {
            token_type: TokenType::Slash,
            literal: ch.to_string(),
          };
        }
        '*' => {
          tok = Token {
            token_type: TokenType::Asterisk,
            literal: ch.to_string(),
          };
        }
        '<' => {
          tok = Token {
            token_type: TokenType::Lt,
            literal: ch.to_string(),
          };
        }
        '>' => {
          tok = Token {
            token_type: TokenType::Gt,
            literal: ch.to_string(),
          };
        }
        ';' => {
          tok = Token {
            token_type: TokenType::Semicolon,
            literal: ch.to_string(),
          };
        }
        ',' => {
          tok = Token {
            token_type: TokenType::Comma,
            literal: ch.to_string(),
          };
        }
        '{' => {
          tok = Token {
            token_type: TokenType::LBrace,
            literal: ch.to_string(),
          };
        }
        '}' => {
          tok = Token {
            token_type: TokenType::RBrace,
            literal: ch.to_string(),
          };
        }
        '(' => {
          tok = Token {
            token_type: TokenType::LParen,
            literal: ch.to_string(),
          };
        }
        ')' => {
          tok = Token {
            token_type: TokenType::RParen,
            literal: ch.to_string(),
          };
        }
        _ => {
          // we return early from identier and number branches because read_identifier and read_number advance
          // ch past the last character of the literal, so we do not need to read_char again at the end of this function
          if is_letter(ch) {
            let literal = self.read_identifier();
            let token_type = lookup_ident(&literal);
            return Token {
              token_type,
              literal,
            };
          } else if is_digit(ch) {
            return Token {
              token_type: TokenType::Int,
              literal: self.read_number(),
            };
          } else {
            tok = Token {
              token_type: TokenType::Illegal,
              literal: ch.to_string(),
            };
          }
        }
      }
    } else {
      tok = Token {
        token_type: TokenType::Eof,
        literal: "".to_string(),
      };
    }

    self.read_char();

    tok
  }

  fn read_char(&mut self) {
    self.ch = self.input.chars().nth(self.read_position);
    self.position = self.read_position;
    self.read_position += 1;
  }

  fn peek_char(&self) -> Option<char> {
    self.input.chars().nth(self.read_position)
  }

  fn read_identifier(&mut self) -> String {
    let position = self.position;
    while let Some(ch) = self.ch
      && is_letter(ch)
    {
      self.read_char();
    }
    self.input[position..self.position].to_string()
  }

  fn read_number(&mut self) -> String {
    let position = self.position;
    while let Some(ch) = self.ch
      && is_digit(ch)
    {
      self.read_char();
    }
    self.input[position..self.position].to_string()
  }

  fn skip_whitespace(&mut self) {
    while let Some(ch) = self.ch
      && ch.is_ascii_whitespace()
    {
      self.read_char();
    }
  }
}

fn is_letter(ch: char) -> bool {
  ch.is_ascii_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
  ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_next_token() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
";

    let tests = vec![
      (TokenType::Let, "let"),
      (TokenType::Ident, "five"),
      (TokenType::Assign, "="),
      (TokenType::Int, "5"),
      (TokenType::Semicolon, ";"),
      (TokenType::Let, "let"),
      (TokenType::Ident, "ten"),
      (TokenType::Assign, "="),
      (TokenType::Int, "10"),
      (TokenType::Semicolon, ";"),
      (TokenType::Let, "let"),
      (TokenType::Ident, "add"),
      (TokenType::Assign, "="),
      (TokenType::Function, "fn"),
      (TokenType::LParen, "("),
      (TokenType::Ident, "x"),
      (TokenType::Comma, ","),
      (TokenType::Ident, "y"),
      (TokenType::RParen, ")"),
      (TokenType::LBrace, "{"),
      (TokenType::Ident, "x"),
      (TokenType::Plus, "+"),
      (TokenType::Ident, "y"),
      (TokenType::Semicolon, ";"),
      (TokenType::RBrace, "}"),
      (TokenType::Semicolon, ";"),
      (TokenType::Let, "let"),
      (TokenType::Ident, "result"),
      (TokenType::Assign, "="),
      (TokenType::Ident, "add"),
      (TokenType::LParen, "("),
      (TokenType::Ident, "five"),
      (TokenType::Comma, ","),
      (TokenType::Ident, "ten"),
      (TokenType::RParen, ")"),
      (TokenType::Semicolon, ";"),
      (TokenType::Bang, "!"),
      (TokenType::Minus, "-"),
      (TokenType::Slash, "/"),
      (TokenType::Asterisk, "*"),
      (TokenType::Int, "5"),
      (TokenType::Semicolon, ";"),
      (TokenType::Int, "5"),
      (TokenType::Lt, "<"),
      (TokenType::Int, "10"),
      (TokenType::Gt, ">"),
      (TokenType::Int, "5"),
      (TokenType::Semicolon, ";"),
      (TokenType::If, "if"),
      (TokenType::LParen, "("),
      (TokenType::Int, "5"),
      (TokenType::Lt, "<"),
      (TokenType::Int, "10"),
      (TokenType::RParen, ")"),
      (TokenType::LBrace, "{"),
      (TokenType::Return, "return"),
      (TokenType::True, "true"),
      (TokenType::Semicolon, ";"),
      (TokenType::RBrace, "}"),
      (TokenType::Else, "else"),
      (TokenType::LBrace, "{"),
      (TokenType::Return, "return"),
      (TokenType::False, "false"),
      (TokenType::Semicolon, ";"),
      (TokenType::RBrace, "}"),
      (TokenType::Int, "10"),
      (TokenType::Eq, "=="),
      (TokenType::Int, "10"),
      (TokenType::Semicolon, ";"),
      (TokenType::Int, "10"),
      (TokenType::NotEq, "!="),
      (TokenType::Int, "9"),
      (TokenType::Semicolon, ";"),
      (TokenType::Eof, ""),
    ];

    let mut l = Lexer::new(input);

    for (i, (expected_type, expected_literal)) in tests.into_iter().enumerate() {
      let tok = l.next_token();

      assert_eq!(
        tok.token_type, expected_type,
        "tests[{}] - tokentype wrong. expected={:#?}, got={:#?}",
        i, expected_type, tok.token_type
      );

      assert_eq!(
        tok.literal, expected_literal,
        "tests[{}] - literal wrong. expected={}, got={}",
        i, expected_literal, tok.literal
      );
    }
  }
}
