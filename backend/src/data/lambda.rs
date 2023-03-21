use lispers_common::Symbol;

use crate::prelude::*;

#[derive(Clone)]
pub struct Lambda<S: Symbol> {
  pub params: Vec<S>,
  pub code: Box<Op<S>>,
}
