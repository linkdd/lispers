use std::{rc::Rc, cell::RefCell};
use lispers_common::Symbol;

use crate::prelude::*;
use crate::data::Value;
use crate::env::Env;

use crate::utils::assert_at_least_args;

pub fn ilt<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: i64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val < val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}

pub fn ilte<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: i64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val <= val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}

pub fn igte<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: i64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val >= val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}

pub fn igt<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: i64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val > val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}


pub fn flt<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: f64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val < val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}

pub fn flte<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: f64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val <= val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}

pub fn fgte<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: f64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val >= val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}

pub fn fgt<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_val = None;

  for arg in args {
    let val: f64 = arg.try_into()?;

    if let Some(prev_val) = prev_val {
      result = prev_val > val;
    }

    prev_val = Some(val);
  }

  Ok(Value::Boolean(result))
}

pub fn eq<S: Symbol>(
  env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_at_least_args(2, args.len())?;

  let mut result = true;
  let mut prev_arg = None;

  for arg in args {
    if let Some(prev_arg) = prev_arg {
      result = match (&prev_arg, &arg) {
        (Value::Boolean(a), Value::Boolean(b)) => a == b,
        (Value::Integer(a), Value::Integer(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Symbol(a), Value::Symbol(b)) => a.as_symbol() == b.as_symbol(),
        (Value::List(a), Value::List(b)) => {
          let mut matching = true;

          for (item_a, item_b) in std::iter::zip(a.into_iter(), b.into_iter()) {
            let item_eq: bool = eq(env.clone(), vec![item_a, item_b])?.try_into()?;
            matching &= item_eq;
          }

          matching
        },
        _ => false,
      };
    }

    prev_arg = Some(arg);
  }

  Ok(Value::Boolean(result))
}

pub fn ne<S: Symbol>(
  env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  let equal: bool = eq(env.clone(), args)?.try_into()?;
  Ok(Value::Boolean(!equal))
}
