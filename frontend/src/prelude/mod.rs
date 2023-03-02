use crate::lexer::TokenLocation;

pub type Result<T> = std::result::Result<T, SyntaxError>;

#[derive(Debug)]
pub enum SyntaxError {
  InvalidToken {
    token: String,
    filename: Option<std::path::PathBuf>,
    line: usize,
    col: usize,
  },
  UnexpectedToken {
    token: String,
    filename: Option<std::path::PathBuf>,
    line: usize,
    col: usize,
  },
}

impl std::fmt::Display for SyntaxError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::InvalidToken { token, filename, line, col } => {
        let filename = filename
          .as_ref()
          .map(|path| path.as_path().display().to_string())
          .unwrap_or("<>".to_string());

        write!(f, "{}[{};{}] Invalid token '{}'", filename, line, col, token)
      },
      Self::UnexpectedToken { token, filename, line, col } => {
        let filename = filename
          .as_ref()
          .map(|path| path.as_path().display().to_string())
          .unwrap_or("<>".to_string());

        write!(f, "{}[{};{}] Unexpected token '{}'", filename, line, col, token)
      },
    }
  }
}

impl std::error::Error for SyntaxError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}

type ParseError = peg::error::ParseError<TokenLocation>;

impl From<ParseError> for SyntaxError {
  fn from(err: ParseError) -> Self {
    let TokenLocation {
      token,
      filename,
      linecol: (line, col),
    } = err.location;

    Self::UnexpectedToken {
      token: token.map(|tok| tok.to_string()).unwrap_or("<>".to_string()),
      filename,
      line,
      col,
    }
  }
}
