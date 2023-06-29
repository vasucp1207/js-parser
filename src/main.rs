use std::str::Chars;

struct  Lexer<'a> {
  source: &'a str,
  chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      source,
      chars: source.chars()
    }
  }

  fn read_next_kind(&mut self) -> Kind {
    while let Some(c) = self.chars.next() {
      match c {
        '+' => return Kind::Plus,
        _ => {}    
      }
    }
    Kind::Eof
  }

  fn offset(&self) -> usize {
    self.source.len() - self.chars.as_str().len()
  }
  
  fn read_next_token(&mut self) -> Token {
    let kind = self.read_next_kind();
    let start: usize = self.offset();
    let end = self.offset();

    Token { kind, start, end }
  }
}

pub struct Token {
  pub kind: Kind,
  pub start: usize,
  pub end: usize,
}

pub enum Kind {
  Eof,
  Plus
}

fn main() {
}
