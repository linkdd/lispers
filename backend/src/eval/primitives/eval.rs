use std::hash::Hash;
use string_interner::Symbol;

use crate::prelude::*;
use crate::data::Value;
use crate::eval::VM;

use crate::utils::assert_exactly_args;

pub fn eval<S: Symbol + Hash>(
  vm: &mut VM<S>,
  args: &[Value<S>],
) -> Result<Value<S>> {
  assert_exactly_args(1, args.len())?;

  let form = vm.resolve_value(&args[0])?;
  vm.eval_form(&form)
}
