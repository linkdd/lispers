use logos::Logos;
use snailquote::unescape;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
  #[token("(")]
  ParenOpen,

  #[token(")")]
  ParenClose,

  #[token("true")]
  True,

  #[token("false")]
  False,

  #[regex("[^ \\t\\r\\n\\f\"\\(\\)]+", |lex| lex.slice().parse())]
  Symbol(String),

  #[regex("\"(?:[^\"]|\\\\\")*\"", |lex| {
    unescape(lex.slice())
  })]
  String(String),

  #[regex(r"[+-]?((\d+\.?\d*)|(\.\d+))(([eE][+-]?)?\d+)?", |lex| {
    lex.slice().parse()
  }, priority = 2)]
  Float(f64),

  #[regex(r"0b_*[01][_01]*", |lex| {
    parse_int::parse::<i64>(lex.slice())
  }, priority = 3)]
  IntegerBase2(i64),

  #[regex(r"0o_*[0-7][_0-7]*", |lex| {
    parse_int::parse::<i64>(lex.slice())
  }, priority = 3)]
  IntegerBase8(i64),

  #[regex(r"[+-]?(0|[1-9][_0-9]*)", |lex| {
    parse_int::parse::<i64>(lex.slice())
  }, priority = 3)]
  IntegerBase10(i64),

  #[regex(r"0x_*[0-9a-fA-F][_0-9a-fA-F]*", |lex| {
    parse_int::parse::<i64>(lex.slice())
  }, priority = 3)]
  IntegerBase16(i64),

  #[error]
  #[regex(r"[ \t\r\n\f]+", logos::skip)]
  Error,
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
