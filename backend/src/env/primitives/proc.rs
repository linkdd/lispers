use std::{rc::Rc, cell::RefCell};
use lispers_common::Symbol;

use crate::prelude::*;
use crate::data::Value;
use crate::env::Env;

use crate::utils::assert_exactly_args;

pub fn exit<S: Symbol>(
  _env: Rc<RefCell<Env<S>>>,
  args: Vec<Value<S>>,
) -> Result<Value<S>> {
  assert_exactly_args(1, args.len())?;

  let code: i64 = (&args[0]).try_into()?;
  std::process::exit(code as i32);
}
