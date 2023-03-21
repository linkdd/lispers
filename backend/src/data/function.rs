use std::{rc::Rc, cell::RefCell};
use lispers_common::Symbol;

use crate::prelude::*;
use crate::env::{Env, RTE};
use super::{Value, Lambda};

pub type NativeFn<S> = fn(Rc<RefCell<Env<S>>>, Vec<Value<S>>) -> Result<Value<S>>;

#[derive(Clone)]
pub struct Closure<S: Symbol> {
  pub rte: Rc<RefCell<RTE<S>>>,
  pub func: Function<S>,
}

#[derive(Clone)]
pub enum Function<S: Symbol> {
  NativeFn(Option<Rc<RefCell<RTE<S>>>>, NativeFn<S>),
  Lambda(Option<Rc<RefCell<RTE<S>>>>, Lambda<S>),
}
