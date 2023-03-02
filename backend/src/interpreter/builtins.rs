use std::{rc::Rc, cell::RefCell};
use lispers_common::{Backend, Symbol};

use crate::prelude::*;
use crate::data::{Value, Sym, List, Function, Lambda};
use crate::env::Env;
use super::Interpreter;

use crate::utils::{assert_exactly_args, assert_at_least_args};

impl<S: Symbol, B: Backend<S>> Interpreter<S, B> {
  pub fn builtin_println(&mut self, env: Rc<RefCell<Env<S>>>, args: Vec<Value<S>>) -> Result<Value<S>> {
    assert_at_least_args(1, args.len())?;

    let mut values = Vec::with_capacity(args.len());

    for arg in args {
      let val = self.eval_expression(env.clone(), arg)?;
      values.push(val);
    }

    let output = values
      .iter()
      .map(|val| self.format_value(val))
      .collect::<Vec<String>>()
      .join(" ");

    println!("{}", output);
    Ok(Value::default())
  }

  pub fn builtin_quote(&mut self, args: Vec<Value<S>>) -> Result<Value<S>> {
    assert_exactly_args(1, args.len())?;
    let arg = &args[0];
    Ok(arg.clone())
  }

  pub fn builtin_define(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    args: Vec<Value<S>>,
  ) -> Result<Value<S>> {
    assert_exactly_args(2, args.len())?;
    let var = &args[0];
    let val = &args[1];

    let sym: Sym<S> = var.try_into()?;
    let sym = sym.as_symbol();

    let val = self.eval_expression(env.clone(), val.clone())?;
    env.borrow_mut().define(sym, val.clone());
    Ok(val)
  }

  pub fn builtin_set(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    args: Vec<Value<S>>,
  ) -> Result<Value<S>> {
    assert_exactly_args(2, args.len())?;
    let var = &args[0];
    let val = &args[1];

    let sym: Sym<S> = var.try_into()?;
    let sym = sym.as_symbol();

    let val = self.eval_expression(env.clone(), val.clone())?;
    env.borrow_mut().set(sym, val.clone())
      .map_err(|sym| RuntimeError::UndefinedSymbol {
        detail: self.interner.resolve(sym).unwrap_or("<>").to_string(),
      })?;

    Ok(val)
  }

  pub fn builtin_controlflow_if(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    args: Vec<Value<S>>,
  ) -> Result<Value<S>> {
    assert_exactly_args(3, args.len())?;

    let test = &args[0];
    let true_branch = &args[1];
    let false_branch = &args[2];

    let test_result = self.eval_expression(env.clone(), test.clone())?;
    let test_result: bool = test_result.try_into()?;

    let branch = if test_result {
      true_branch
    }
    else {
      false_branch
    };

    self.eval_expression(env.clone(), branch.clone())
  }

  pub fn builtin_lambda(&mut self, args: Vec<Value<S>>) -> Result<Value<S>> {
    assert_exactly_args(2, args.len())?;

    let params = &args[0];
    let body = &args[1];

    let params: List<S> = params.try_into()?;
    let mut param_names = Vec::new();

    for param in params.into_iter() {
      let param_name: Sym<S> = param.try_into()?;
      param_names.push(param_name.as_symbol());
    }

    let lambda = Value::Function(Function::Lambda(Lambda {
      params: param_names,
      body: Box::new(body.clone()),
    }));

    Ok(lambda)
  }

  pub fn builtin_let_expression(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    args: Vec<Value<S>>,
  ) -> Result<Value<S>> {
    assert_exactly_args(2, args.len())?;
    let decls = &args[0];
    let body = &args[1];

    let decls: List<S> = decls.try_into()?;
    let mut params = Vec::new();
    let mut args = Vec::new();

    for decl in decls.into_iter() {
      let decl: List<S> = decl.try_into()?;
      let decl: Vec<Value<S>> = decl.into_iter().collect();
      assert_exactly_args(2, decl.len())?;

      let sym = &decl[0];
      let val = &decl[1];

      let sym: Sym<S> = sym.try_into()?;
      let val = self.eval_expression(env.clone(), val.clone())?;

      params.push(sym.as_symbol());
      args.push(val);
    }

    let lambda = Function::Lambda(Lambda {
      params,
      body: Box::new(body.clone()),
    });

    self.eval_function(env.clone(), lambda, args)
  }
}
