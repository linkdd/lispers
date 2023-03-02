use std::{rc::Rc, cell::RefCell};

use lispers_common::{StringInterner, Backend, Symbol};
use lispers_frontend::{SExpression, Literal};
use crate::prelude::*;
use crate::data::{Value, List, Function};
use crate::env::{Env, default_env};

use crate::utils::assert_exactly_args;

mod builtins;

pub struct Interpreter<S: Symbol, B: Backend<S>> {
  interner: StringInterner<B>,
  marker: std::marker::PhantomData<S>,
}

impl<S: Symbol, B: Backend<S>> Interpreter<S, B> {
  pub fn new() -> Self {
    Self {
      interner: StringInterner::new(),
      marker: std::marker::PhantomData{},
    }
  }

  pub fn default_env(&mut self) -> Rc<RefCell<Env<S>>> {
    Rc::new(RefCell::new(default_env(&mut self.interner)))
  }

  pub fn format_value(&self, value: &Value<S>) -> String {
    match value {
      Value::Boolean(val) => format!("{}", val),
      Value::Integer(val) => format!("{}", val),
      Value::Float(val) => format!("{}", val),
      Value::String(val) => format!("{}", val),
      Value::Symbol(sym) => {
        self.interner.resolve(sym.as_symbol()).unwrap_or("<>").to_string()
      },
      Value::List(list) => {
        let repr = list
          .into_iter()
          .map(|value| self.format_value(&value))
          .collect::<Vec<String>>()
          .join(" ");

        format!("({})", repr)
      },
      Value::Function(func) => match func {
        Function::NativeFn(native_func) => {
          format!("[function {:p}]", native_func)
        },
        Function::Lambda(lambda) => {
          format!("[function {:p}]", lambda)
        },
      },
    }
  }

  pub fn eval_file<P: AsRef<std::path::Path>>(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    input_path: P,
  ) -> Result<Value<S>> {
    let input = std::fs::read_to_string(input_path)?;
    self.eval_string(env, &input)
  }

  pub fn eval_string(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    input: &str,
  ) -> Result<Value<S>> {
    let sexpressions = lispers_frontend::parse(
      None,
      &input,
      &mut self.interner,
    )?;

    let mut last_result = Value::default();

    for sexpression in sexpressions.iter() {
      let expression = self.parse_sexpression(sexpression)?;
      last_result = self.eval_expression(env.clone(), expression)?;
    }

    Ok(last_result)
  }

  pub fn eval_expression(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    expression: Value<S>,
  ) -> Result<Value<S>> {
    match expression {
      Value::Symbol(sym) => {
        let sym = sym.as_symbol();
        env.borrow().get(sym).ok_or_else(|| RuntimeError::UndefinedSymbol {
          detail: self.interner.resolve(sym).unwrap_or("<>").to_string()
        })
      },
      Value::List(list) if !list.empty() => {
        let func = list.car()?;
        let args: Vec<Value<S>> = list.cdr().into_iter().collect();

        if let Value::Symbol(sym) = &func {
          let sym = sym.as_symbol();
          let func_name = self.interner.resolve(sym).unwrap_or("<>");

          match func_name {
            "println" => return self.builtin_println(env.clone(), args),
            "quote" => return self.builtin_quote(args),
            "def" => return self.builtin_define(env.clone(), args),
            "set!" => return self.builtin_set(env.clone(), args),
            "if" => return self.builtin_controlflow_if(env.clone(), args),
            "lambda" => return self.builtin_lambda(args),
            "let" => return self.builtin_let_expression(env.clone(), args),
            _ => {},
          }
        }

        let func = self.eval_expression(env.clone(), func)?;
        let func: Function<S> = func.try_into()?;

        let mut eval_args = Vec::with_capacity(args.len());

        for arg in args {
          let arg = self.eval_expression(env.clone(), arg)?;
          eval_args.push(arg);
        }

        self.eval_function(env.clone(), func, eval_args)
      },
      _ => {
        Ok(expression)
      },
    }
  }

  fn eval_function(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    func: Function<S>,
    args: Vec<Value<S>>,
  ) -> Result<Value<S>> {
    match func {
      Function::NativeFn(func) => {
        func(env.clone(), args)
      },
      Function::Lambda(lambda) => {
        assert_exactly_args(lambda.params.len(), args.len())?;

        let mut env = Env::extend(env.clone());

        for (name, arg) in std::iter::zip(lambda.params, args) {
          env.define(name, arg);
        }

        self.eval_expression(
          Rc::new(RefCell::new(env)),
          lambda.body.as_ref().clone(),
        )
      },
    }
  }

  fn parse_sexpression(&self, sexpression: &SExpression<S>) -> Result<Value<S>> {
    match sexpression {
      SExpression::Literal(val) => {
        Ok(self.parse_literal(val))
      },
      SExpression::List(elements) => {
        let mut list = List::NIL;

        for element in elements.iter().rev() {
          let val = self.parse_sexpression(element)?;
          list = list.cons(val);
        }

        Ok(Value::List(list))
      }
    }
  }

  fn parse_literal(&self, literal: &Literal<S>) -> Value<S> {
    match literal {
      Literal::Boolean(val) => Value::Boolean(*val),
      Literal::Integer(val) => Value::Integer(*val),
      Literal::Float(val) => Value::Float(*val),
      Literal::String(val) => Value::String(val.clone()),
      Literal::Symbol(sym) => Value::Symbol(sym.into()),
    }
  }
}
