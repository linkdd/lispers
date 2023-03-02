use std::hash::Hash;
use string_interner::Symbol;

use crate::{prelude::*, let_irrefutable};
use crate::data::{Type, Value};
use crate::eval::VM;

use crate::utils::assert_exactly_args;

pub fn exit<S: Symbol + Hash>(
  vm: &mut VM<S>,
  args: &[Value<S>],
) -> Result<Value<S>> {
  assert_exactly_args(1, args.len())?;

  let arg = vm.resolve_value(&args[0])?;
  arg.assert_type(Type::Integer)?;

  let_irrefutable!(code, Value::Integer(code) = arg);
  std::process::exit(code as i32);
}
