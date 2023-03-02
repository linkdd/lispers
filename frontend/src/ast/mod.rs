use lispers_common::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<S: Symbol> {
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Symbol(S),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SExpression<S: Symbol> {
  Literal(Literal<S>),
  List(Vec<SExpression<S>>),
}
