use std::{rc::Rc, cell::RefCell};
use lispers_common::Symbol;

use crate::prelude::*;
use crate::env::Env;
use super::{Value, Lambda};

pub type NativeFn<S> = fn(Rc<RefCell<Env<S>>>, Vec<Value<S>>) -> Result<Value<S>>;

#[derive(Clone)]
pub enum Function<S: Symbol> {
  NativeFn(NativeFn<S>),
  Lambda(Lambda<S>),
}
