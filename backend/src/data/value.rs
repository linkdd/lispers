use lispers_common::Symbol;

use crate::prelude::*;
use super::{List, Function};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Boolean,
  Integer,
  Float,
  String,
  Symbol,
  List,
  Function,
}

impl Type {
  pub fn error(got: Type, expected: Type) -> RuntimeError {
    RuntimeError::TypeError {
      expected: format!("{:?}", expected),
      got: format!("{:?}", got)
    }
  }
}

#[derive(Clone)]
pub struct Sym<S: Symbol>(S);

impl<S: Symbol> Sym<S> {
  pub fn as_symbol(&self) -> S {
    self.0
  }
}

impl<S: Symbol> From<S> for Sym<S> {
  fn from(sym: S) -> Self {
    Self(sym)
  }
}

impl<S: Symbol> From<&S> for Sym<S> {
  fn from(sym: &S) -> Self {
    Self(*sym)
  }
}

#[derive(Clone)]
pub enum Value<S: Symbol> {
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Symbol(Sym<S>),
  List(List<S>),
  Function(Function<S>),
}

impl<S: Symbol> Default for Value<S> {
  fn default() -> Self {
    Self::List(List::NIL)
  }
}

impl<S: Symbol> Value<S> {
  pub fn as_type(&self) -> Type {
    match self {
      Value::Boolean(..) => Type::Boolean,
      Value::Integer(..) => Type::Integer,
      Value::Float(..) => Type::Float,
      Value::String(..) => Type::String,
      Value::Symbol(..) => Type::Symbol,
      Value::List(..) => Type::List,
      Value::Function(..) => Type::Function,
    }
  }
}

impl<S: Symbol> TryFrom<Value<S>> for bool {
  type Error = RuntimeError;

  fn try_from(value: Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Boolean(val) => Ok(val),
      _ => Err(Type::error(value.as_type(), Type::Boolean)),
    }
  }
}

impl<S: Symbol> TryFrom<Value<S>> for i64 {
  type Error = RuntimeError;

  fn try_from(value: Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Integer(val) => Ok(val),
      _ => Err(Type::error(value.as_type(), Type::Integer)),
    }
  }
}

impl<S: Symbol> TryFrom<Value<S>> for f64 {
  type Error = RuntimeError;

  fn try_from(value: Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Float(val) => Ok(val),
      _ => Err(Type::error(value.as_type(), Type::Float)),
    }
  }
}

impl<S: Symbol> TryFrom<Value<S>> for String {
  type Error = RuntimeError;

  fn try_from(value: Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::String(val) => Ok(val),
      _ => Err(Type::error(value.as_type(), Type::String)),
    }
  }
}

impl<S: Symbol> TryFrom<Value<S>> for Sym<S> {
  type Error = RuntimeError;

  fn try_from(value: Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Symbol(val) => Ok(val),
      _ => Err(Type::error(value.as_type(), Type::Symbol)),
    }
  }
}

impl<S: Symbol> TryFrom<Value<S>> for List<S> {
  type Error = RuntimeError;

  fn try_from(value: Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::List(val) => Ok(val),
      _ => Err(Type::error(value.as_type(), Type::List)),
    }
  }
}

impl<S: Symbol> TryFrom<Value<S>> for Function<S> {
  type Error = RuntimeError;

  fn try_from(value: Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Function(val) => Ok(val),
      _ => Err(Type::error(value.as_type(), Type::Function)),
    }
  }
}

impl<S: Symbol> TryFrom<&Value<S>> for bool {
  type Error = RuntimeError;

  fn try_from(value: &Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Boolean(val) => Ok(*val),
      _ => Err(Type::error(value.as_type(), Type::Boolean)),
    }
  }
}

impl<S: Symbol> TryFrom<&Value<S>> for i64 {
  type Error = RuntimeError;

  fn try_from(value: &Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Integer(val) => Ok(*val),
      _ => Err(Type::error(value.as_type(), Type::Integer)),
    }
  }
}

impl<S: Symbol> TryFrom<&Value<S>> for f64 {
  type Error = RuntimeError;

  fn try_from(value: &Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Float(val) => Ok(*val),
      _ => Err(Type::error(value.as_type(), Type::Float)),
    }
  }
}

impl<S: Symbol> TryFrom<&Value<S>> for String {
  type Error = RuntimeError;

  fn try_from(value: &Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::String(val) => Ok(val.clone()),
      _ => Err(Type::error(value.as_type(), Type::String)),
    }
  }
}

impl<S: Symbol> TryFrom<&Value<S>> for Sym<S> {
  type Error = RuntimeError;

  fn try_from(value: &Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Symbol(val) => Ok(val.clone()),
      _ => Err(Type::error(value.as_type(), Type::Symbol)),
    }
  }
}

impl<S: Symbol> TryFrom<&Value<S>> for List<S> {
  type Error = RuntimeError;

  fn try_from(value: &Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::List(val) => Ok(val.clone()),
      _ => Err(Type::error(value.as_type(), Type::List)),
    }
  }
}

impl<S: Symbol> TryFrom<&Value<S>> for Function<S> {
  type Error = RuntimeError;

  fn try_from(value: &Value<S>) -> std::result::Result<Self, Self::Error> {
    match value {
      Value::Function(val) => Ok(val.clone()),
      _ => Err(Type::error(value.as_type(), Type::Function)),
    }
  }
}
