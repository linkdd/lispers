use std::{
  collections::HashMap,
  rc::Rc,
  cell::RefCell,
};
use lispers_common::Symbol;
use crate::data::Value;

mod primitives;
mod default;
pub use default::default_env;

pub struct Env<S: Symbol> {
  parent: Option<Rc<RefCell<Env<S>>>>,
  values: HashMap<S, Value<S>>,
}

impl<S: Symbol> Env<S> {
  pub fn new() -> Self {
    Self {
      parent: None,
      values: HashMap::new(),
    }
  }

  pub fn extend(parent: Rc<RefCell<Env<S>>>) -> Self {
    Self {
      parent: Some(parent),
      values: HashMap::new(),
    }
  }

  pub fn define(&mut self, symbol: S, value: Value<S>) {
    self.values.insert(symbol, value);
  }

  pub fn undefine(&mut self, symbol: S) {
    if self.values.contains_key(&symbol) {
      self.values.remove(&symbol);
    }
    else if let Some(parent) = &self.parent {
      parent.borrow_mut().undefine(symbol);
    }
  }

  pub fn get(&self, symbol: S) -> Option<Value<S>> {
    if let Some(val) = self.values.get(&symbol) {
      Some(val.clone())
    }
    else if let Some(parent) = &self.parent {
      parent.borrow().get(symbol)
    }
    else {
      None
    }
  }

  pub fn set(&mut self, symbol: S, value: Value<S>) -> Result<(), S> {
    if self.values.contains_key(&symbol) {
      self.values.insert(symbol, value);
      Ok(())
    }
    else if let Some(parent) = &self.parent {
      parent.borrow_mut().set(symbol, value)
    }
    else {
      Err(symbol)
    }
  }
}
