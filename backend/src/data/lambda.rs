use lispers_common::Symbol;

use crate::prelude::*;

//use std::{rc::Rc};

#[derive(Clone)]
pub struct Lambda<S: Symbol> {
  pub params: Vec<S>,
  pub code: RtOp<S>,
}
