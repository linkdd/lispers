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
  NYIE{ detail: String}
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
      Self::NYIE { detail }=> {
        write!(f, "NYIE: {}", detail)
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

pub enum Promise<E, R> {
  Delay(E),
  Done(R)
}

#[inline(always)]
pub fn force_promise<StepFn, E, R>(mut step: StepFn, mut expr: E) -> R
where
    StepFn: FnMut(E) -> Promise<E, R> // TBD: try FnMut here?
{
  loop {
    match step(expr) {
      Promise::Delay(tail) => {
        expr = tail;
        continue;
      }
      Promise::Done(result) => {
        break result;
      }
    }
  }
}

pub use Promise::*;

use super::data::{Value, Function};

// use std::{rc::Rc}; // TBD: Should we use Rc over Box here?
#[derive(Clone)]
pub enum Op<S: lispers_common::Symbol> {
  FetchGle(S), // read global variable
  RefRTE(usize, usize),
  If(Box<Op<S>>, Box<Op<S>>, Box<Op<S>>),
  Finish(Value<S>),
  Enclose(Function<S>), // close over rte
  Apply(Box<Op<S>>, Vec<Op<S>>),
  PRINTLN(Vec<Op<S>>), // FIXME: can't get that to work properly
}
