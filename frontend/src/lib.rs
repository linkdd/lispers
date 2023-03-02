use lispers_common::{StringInterner, Backend, Symbol};

mod prelude;
mod lexer;
mod ast;
mod grammar;

use self::prelude::*;
pub use self::{
  prelude::SyntaxError,
  ast::{SExpression, Literal},
};

pub fn parse<S: Symbol, B: Backend<S>>(
  filename: Option<std::path::PathBuf>,
  input: &str,
  interner: &mut StringInterner<B>,
) -> Result<Vec<SExpression<S>>>
{
  let stream = lexer::TokenStream::new(
    filename,
    input
  )?;
  let ast = grammar::module_parser::module(&stream, interner)?;
  Ok(ast)
}
