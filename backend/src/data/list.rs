use std::{rc::Rc, cell::RefCell};
use lispers_common::Symbol;

use crate::prelude::*;
use super::{Value, ConsCell};

#[derive(Clone)]
pub struct List<S: Symbol> {
  head: Option<Rc<RefCell<ConsCell<S>>>>,
}

#[derive(Clone)]
pub struct ListIterator<S: Symbol>(Option<Rc<RefCell<ConsCell<S>>>>);

impl<S: Symbol> List<S> {
  pub const NIL: Self = Self { head: None };

  pub fn empty(&self) -> bool {
    self.head.is_none()
  }

  pub fn car(&self) -> Result<Value<S>> {
    self.head
      .as_ref()
      .map(|rc| rc.borrow().car.clone())
      .ok_or_else(|| RuntimeError::NilValue { detail: "car called on empty list".to_string() })
  }

  pub fn cdr(&self) -> Self {
    let head = self.head
      .as_ref()
      .and_then(|rc| rc.borrow().cdr.as_ref().cloned());

    Self { head }
  }

  pub fn cons(&self, val: Value<S>) -> Self {
    Self {
      head: Some(Rc::new(RefCell::new(ConsCell {
        car: val,
        cdr: self.head.clone()
      })))
    }
  }
}

impl<'a, S: Symbol> IntoIterator for &'a List<S> {
  type Item = Value<S>;
  type IntoIter = ListIterator<S>;

  fn into_iter(self) -> Self::IntoIter {
    ListIterator(self.head.clone())
  }
}

impl<S: Symbol> Iterator for ListIterator<S> {
  type Item = Value<S>;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.clone().map(|cons| {
      let val = cons.borrow().car.clone();
      self.0 = cons.borrow().cdr.clone();
      val
    })
  }
}
