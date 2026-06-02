#[derive(Debug, PartialEq)]
pub enum TokenType {
  Illegal,
  Eof,

  // Identifiers + literals
  Ident, // add, foobar, x, y, ...
  Int,   // 1343456

  // Operators
  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,

  Lt,
  Gt,

  Eq,
  NotEq,

  // Delimiters
  Comma,
  Semicolon,

  LParen,
  RParen,
  LBrace,
  RBrace,

  // Keywords
  Function,
  Let,
  True,
  False,
  If,
  Else,
  Return,
}

#[derive(Debug, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

// keywords
pub fn lookup_keyword(keyword: &str) -> Option<TokenType> {
  match keyword {
    "fn" => Some(TokenType::Function),
    "let" => Some(TokenType::Let),
    "true" => Some(TokenType::True),
    "false" => Some(TokenType::False),
    "if" => Some(TokenType::If),
    "else" => Some(TokenType::Else),
    "return" => Some(TokenType::Return),
    _ => None,
  }
}

// LookupIdent
pub fn lookup_ident(ident: &str) -> TokenType {
  lookup_keyword(ident).unwrap_or(TokenType::Ident)
}
