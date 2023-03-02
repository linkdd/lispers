use line_col::LineColLookup;
use logos::{Logos, Span};

use crate::prelude::*;

mod tokenizer;
pub use self::tokenizer::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenLocation {
  pub filename: Option<std::path::PathBuf>,
  pub linecol: (usize, usize),
  pub token: Option<Token>,
}

impl std::fmt::Display for TokenLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (line, col) = self.linecol;
    let filename = self.filename
      .as_ref()
      .and_then(|path| path.to_str())
      .unwrap_or("<>");

    write!(f, "{}[{};{}]", filename, line, col)
  }
}

pub struct TokenStream<'source> {
  filename: Option<std::path::PathBuf>,
  size: usize,
  tokens: Vec<(Token, Span)>,
  linecol_lookup: LineColLookup<'source>,
}

impl<'source> TokenStream<'source> {
  pub fn new(filename: Option<std::path::PathBuf>, input: &'source str) -> Result<Self> {
    let token_stream = Self {
      filename,
      size: input.len(),
      tokens: Token::lexer(input).spanned().collect(),
      linecol_lookup: LineColLookup::new(input),
    };

    for (token, span) in token_stream.tokens.iter() {
      if let Token::Error = token {
        let (line, col) = token_stream.linecol_lookup.get(span.start);

        return Err(SyntaxError::InvalidToken {
          token: format!("{}", token),
          filename: token_stream.filename,
          line,
          col
        })
      }
    }

    Ok(token_stream)
  }
}

impl<'source> peg::Parse for TokenStream<'source> {
  type PositionRepr = TokenLocation;

  fn start<'input>(&'input self) -> usize {
    0
  }

  fn is_eof<'input>(&'input self, pos: usize) -> bool {
    pos >= self.tokens.len()
  }

  fn position_repr<'input>(&'input self, pos: usize) -> Self::PositionRepr {
    let (token, linecol) = match self.tokens.get(pos) {
      Some((token, span)) => {
        (Some(token.clone()), self.linecol_lookup.get(span.start))
      },
      None => {
        (None, self.linecol_lookup.get(self.size))
      }
    };

    Self::PositionRepr {
      filename: self.filename.clone(),
      linecol,
      token,
    }
  }
}

impl<'source, 'input> peg::ParseElem<'input> for TokenStream<'source> {
  type Element = &'input Token;

  fn parse_elem(&'input self, pos: usize) -> peg::RuleResult<Self::Element> {
    match self.tokens.get(pos) {
      Some((token, _)) => {
        peg::RuleResult::Matched(pos + 1, token)
      },
      None => {
        peg::RuleResult::Failed
      },
    }
  }
}

impl <'source> peg::ParseLiteral for TokenStream<'source> {
  fn parse_string_literal(&self, pos: usize, literal: &str) -> peg::RuleResult<()> {
    match self.tokens.get(pos) {
      Some((Token::Symbol(sym), _)) if sym == literal => {
        peg::RuleResult::Matched(pos + 1, ())
      },
      _ => {
        peg::RuleResult::Failed
      }
    }
  }
}

impl<'source, 'input> peg::ParseSlice<'input> for TokenStream<'source> {
  type Slice = Vec<&'input Token>;

  fn parse_slice(&'input self, begin_pos: usize, end_pos: usize) -> Self::Slice {
    self.tokens[begin_pos .. end_pos]
      .into_iter()
      .map(|(token, _)| token)
      .collect()
  }
}
