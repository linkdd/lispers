use std::hash::Hash;
use string_interner::Symbol;

use crate::{prelude::*, let_irrefutable};
use crate::data::{Type, Value};
use crate::eval::VM;

use crate::utils::assert_exactly_args;

pub fn cons<S: Symbol + Hash>(
  vm: &mut VM<S>,
  args: &[Value<S>],
) -> Result<Value<S>> {
  assert_exactly_args(2, args.len())?;

  let elem_arg = vm.resolve_value(&args[0])?;
  let list_arg = vm.resolve_value(&args[1])?;
  list_arg.assert_type(Type::List)?;

  let_irrefutable!(list, Value::List(list) = list_arg);
  Ok(Value::List(list.cons(elem_arg)))
}

pub fn car<S: Symbol + Hash>(
  vm: &mut VM<S>,
  args: &[Value<S>],
) -> Result<Value<S>> {
  assert_exactly_args(1, args.len())?;

  let list_arg = vm.resolve_value(&args[0])?;
  list_arg.assert_type(Type::List)?;

  let_irrefutable!(list, Value::List(list) = list_arg);
  list.car()
}

pub fn cdr<S: Symbol + Hash>(
  vm: &mut VM<S>,
  args: &[Value<S>],
) -> Result<Value<S>> {
  assert_exactly_args(1, args.len())?;

  let list_arg = vm.resolve_value(&args[0])?;
  list_arg.assert_type(Type::List)?;

  let_irrefutable!(list, Value::List(list) = list_arg);
  Ok(Value::List(list.cdr()))
}
