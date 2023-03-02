use std::{rc::Rc, cell::RefCell};
use lispers_common::Symbol;
use super::Value;

#[derive(Clone)]
pub struct ConsCell<S: Symbol> {
  pub car: Value<S>,
  pub cdr: Option<Rc<RefCell<ConsCell<S>>>>,
}
