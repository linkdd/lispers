use crate::prelude::*;

pub fn assert_exactly_args(expected: usize, got: usize) -> Result<()> {
  if got < expected {
    Err(RuntimeError::TooFewArguments { expected, got })
  }
  else if got > expected {
    Err(RuntimeError::TooManyArguments { expected, got })
  }
  else {
    Ok(())
  }
}

pub fn assert_at_least_args(expected: usize, got: usize) -> Result<()> {
  if got < expected {
    Err(RuntimeError::TooFewArguments { expected, got })
  }
  else {
    Ok(())
  }
}
