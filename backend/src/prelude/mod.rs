use lispers_frontend::SyntaxError;

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
  IOError(std::io::Error),
  SyntaxError(SyntaxError),
  NilValue { detail: String },
  UndefinedSymbol { detail: String },
  TooFewArguments { expected: usize, got: usize },
  TooManyArguments { expected: usize, got: usize },
  TypeError { expected: String, got: String },
}

impl std::fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::IOError(err) => {
        write!(f, "IOError: {}", err)
      },
      Self::SyntaxError(err) => {
        write!(f, "SyntaxError: {}", err)
      },
      Self::NilValue { detail } => {
        write!(f, "NilValueError: {}", detail)
      },
      Self::UndefinedSymbol { detail }=> {
        write!(f, "UndefinedSymbol: {}", detail)
      },
      Self::TooFewArguments { expected, got } => {
        write!(
          f,
          "ArityError: Too few arguments for function, expected {} but got {}",
          expected,
          got,
        )
      },
      Self::TooManyArguments { expected, got } => {
        write!(
          f,
          "ArityError: Too many arguments for function, expected {} but got {}",
          expected,
          got,
        )
      },
      Self::TypeError { expected, got } => {
        write!(f, "TypeError: expected <{}> but got <{}>", expected, got)
      }
    }
  }
}

impl std::error::Error for RuntimeError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}

impl From<std::io::Error> for RuntimeError {
  fn from(err: std::io::Error) -> Self {
    Self::IOError(err)
  }
}

impl From<SyntaxError> for RuntimeError {
  fn from(err: SyntaxError) -> Self {
    Self::SyntaxError(err)
  }
}
