use std::{rc::Rc, cell::RefCell};
use lispers_common::Symbol;

use crate::prelude::*;
use crate::data::Value;
use crate::env::Env;

use crate::utils::assert_at_least_args;

pub fn iadd<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = 0;

  for arg in args {
    let val: i64 = arg.try_into()?;
    result += val;
  }

  Ok(Value::Integer(result))
}

pub fn isub<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let first_arg = &args[0];
  let mut result: i64 = first_arg.try_into()?;

  for arg in args[1..].iter() {
    let val: i64 = arg.try_into()?;
    result -= val;
  }

  Ok(Value::Integer(result))
}

pub fn imul<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = 1;

  for arg in args {
    let val: i64 = arg.try_into()?;
    result *= val;
  }

  Ok(Value::Integer(result))
}

pub fn idiv<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let first_arg = &args[0];
  let mut result: i64 = first_arg.try_into()?;

  for arg in args[1..].iter() {
    let val: i64 = arg.try_into()?;
    result /= val;
  }

  Ok(Value::Integer(result))
}

pub fn fadd<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = 0.0;

  for arg in args {
    let val: f64 = arg.try_into()?;
    result += val;
  }

  Ok(Value::Float(result))
}

pub fn fsub<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let first_arg = &args[0];
  let mut result: f64 = first_arg.try_into()?;

  for arg in args[1..].iter() {
    let val: f64 = arg.try_into()?;
    result -= val;
  }

  Ok(Value::Float(result))
}

pub fn fmul<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = 1.0;

  for arg in args {
    let val: f64 = arg.try_into()?;
    result *= val;
  }

  Ok(Value::Float(result))
}

pub fn fdiv<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let first_arg = &args[0];
  let mut result: f64 = first_arg.try_into()?;

  for arg in args[1..].iter() {
    let val: f64 = arg.try_into()?;
    result /= val;
  }

  Ok(Value::Float(result))
}
