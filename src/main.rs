#![allow(unused)]
use std::str::Chars;

struct Lexer<'a> {
  // source text
  source: &'a str,
  // remaining characters
  chars: Chars<'a>
}

impl<'a> Lexer<'a> {
  fn read_next_kind(&mut self) -> Kind {
    while let Some(c) = self.chars.next() {
      match c {
        '+' => return Kind::Plus,
        _ => {}
      }
    }
    Kind::Eof
  }
  
  fn read_next_token(&mut self) -> Token {
    let start = self.offset();
    let kind = self.read_next_kind();
    let end = self.offset();
    Token { kind, start, end }
  }

  // Get the length offset from the source text, in UTF-8 bytes
  fn offset(&self) -> usize {
    self.source.len() - self.chars.as_str().len()
  }

  // To tokenize multi-character operators such as ++ or +=
  fn peek(&self) -> Option<char> {
    // here clone() is necessary because calling next() on an iterator consumes the iterator and moves its internal state.
    self.chars.clone().next()
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
  pub kind: Kind,
  pub start: usize,
  pub end: usize,
  pub value: TokenValue
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(f64),
    String(String)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
  Eof,
  Plus,
  Identifier,
  Number,
  String
}

fn main() {
  let text = "Hello, world!";
  let chars_iter: Chars<'_> = text.chars();
}
