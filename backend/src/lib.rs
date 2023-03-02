mod prelude;
mod utils;
mod data;
mod env;
mod interpreter;

pub use self::{
  prelude::RuntimeError,
  interpreter::Interpreter,
};
