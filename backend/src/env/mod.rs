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

  fn intern_lookup(&self, symbol: S, depth: i64) -> (i64, i64) {
    if let Some(val) = self.values.get(&symbol) {
      match val {
        Value::Integer(val) => { (if let Some(_parent) = &self.parent {depth} else {-1}, *val) }
        _ => {(-1, 1)}
      }
    }
    else if let Some(parent) = &self.parent {
      parent.borrow().intern_lookup(symbol, depth + 1)
    }
    else {
      (-1, -1)
    }
  }
  pub fn lookup(&self, symbol: S) -> (i64, i64) {
    return self.intern_lookup(symbol, 0)
  }
  pub fn print(&self, msg: String) {
   println!("{} cte {:p} {}", msg, self, self.values.len());
  }
}

pub struct RTE<S: Symbol> {
  parent: Option<Rc<RefCell<RTE<S>>>>,
  values: Vec<Value<S>>,
}

impl<S: Symbol> RTE<S> {
  pub fn new() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      parent: None,
      values: Vec::new(),
    }))
  }

  pub fn extend(parent: Rc<RefCell<RTE<S>>>, values: Vec<Value<S>>) -> Rc<RefCell<Self>>  {
    Rc::new(RefCell::new(Self {
      parent: Some(parent.clone()),
      values: values,
    }))
  }

  pub fn get(&self, depth: usize, index: usize) -> Option<Value<S>> {
// self.print("get".to_string());
    if depth == 0 {
      return Some(self.values[index].clone());
    }
    else if let Some(parent) = &self.parent {
// println!("Again {}", depth-1);
      return parent.borrow().get(depth - 1, index)
    } else {
      return None
    }
  }

  pub fn gimmithevaluespleaseatallcost(&mut self) -> Vec<Value<S>> {
    let mut result: Vec<Value<S>> = Vec::with_capacity(self.values.len());
    for val in &self.values { result.push(val.clone()) }
    return result;
  }

  pub fn print(&self, msg: String) {
   println!("{} rte {:p} {}", msg, self, self.values.len());
  }

  pub fn printall(&self, msg: String) {
   println!("{} rte {:p} {}", msg, self, self.values.len());
   if let Some(parent) = &self.parent {
     print!("   parent: {:p}", parent.as_ref());
     parent.borrow().print("   ".to_string());
   }
  }

  pub fn format(&self) -> String {
   return format!("rte {:p} {}", self, self.values.len())
  }

}
