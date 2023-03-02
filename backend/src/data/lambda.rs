use lispers_common::Symbol;

use super::Value;

#[derive(Clone)]
pub struct Lambda<S: Symbol> {
  pub params: Vec<S>,
  pub body: Box<Value<S>>,
}
