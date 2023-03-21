use std::{rc::Rc, cell::RefCell};
use lispers_common::{Backend, Symbol};

use crate::prelude::*;
use crate::data::{Value, Sym, List, Function, Lambda};
use crate::env::{Env, RTE};
use super::Interpreter;
//use super::{Runnable, make_runnable, make_delay};

use crate::utils::{assert_exactly_args, assert_at_least_args};

impl<S: Symbol, B: Backend<S>> Interpreter<S, B> {
/* // PRINTLN(Vec<Op<S>>), // FIXME: can't get that to work properly
  pub fn builtin_println_rt2(&self, env: Rc<RefCell<RTE<S>>>, args: Vec<Value<S>>) -> Result<Value<S>>
  {
    let output = args
      .iter()
      .map(|val| self.format_value(val))
      .collect::<Vec<String>>()
      .join(" ");

    println!("{}", output);
    Ok(Value::default())
  }
// */
  pub fn builtin_println(&mut self, env: Rc<RefCell<Env<S>>>, args: Vec<Value<S>>) -> Result<Op<S>> {
    assert_at_least_args(1, args.len())?;

    let mut values = Vec::with_capacity(args.len());

    for arg in args {
      let val = self.compile_expression(env.clone(), arg)?;
      values.push(val);
    }
/*  // FIXME: can't get that to work properly
    let func = Function::NativeFn(None, |rte, args| { println!(""); Ok(Value::Boolean(false)) //self.buildin_println_rt1(rte, args)
    });
    let func = Op::Enclose(func);
    // Ok(Op::Apply(Box::new(func), values))
    // Op::PRINTLN(Vec<Op<S>>), // FIXME: can't get that to work properly
// */
    Ok(Op::PRINTLN(values)) // FIXME: can't get that to work properly
  }

  pub fn builtin_println_ct(&mut self, env: Rc<RefCell<Env<S>>>, args: Vec<Value<S>>) -> Result<Value<S>> {
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

    env.borrow_mut().define(sym, var.clone());

    let val = self.eval_expression(env.clone(), val.clone())?;
    match env.borrow_mut().set(sym, val.clone()) {
      Ok(_) => { Ok(val) }
      _other => { Ok(val) } // FIXME should signal error
    }
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
  ) -> Result<Op<S>> {
    assert_exactly_args(3, args.len())?;

    let test = &args[0];
    let true_branch = &args[1];
    let false_branch = &args[2];

/*
    let test_compiled = Rc::new(self.compile_expression(env.clone(), test.clone())?);
    let true_compiled = Rc::new(self.compile_expression(env.clone(), true_branch.clone())?);
    let false_compiled = Rc::new(self.compile_expression(env.clone(), false_branch.clone())?);
*/
    let test_compiled = Box::new(self.compile_expression(env.clone(), test.clone())?);
    let true_compiled = Box::new(self.compile_expression(env.clone(), true_branch.clone())?);
    let false_compiled = Box::new(self.compile_expression(env.clone(), false_branch.clone())?);

    Ok(Op::If(test_compiled, true_compiled, false_compiled))

  }

  pub fn builtin_lambda(&mut self, env: Rc<RefCell<Env<S>>>, args: Vec<Value<S>>) -> Result<Op<S>> {
    assert_exactly_args(2, args.len())?;

    let params = &args[0];
    let body = &args[1];

    let params: List<S> = params.try_into()?;
    let mut param_names = Vec::new();

    for param in params.into_iter() {
      let param_name: Sym<S> = param.try_into()?;
      param_names.push(param_name.as_symbol());
    }

    let mut env = Env::extend(env.clone());

    /*
     for (name, arg) in std::iter::zip(lambda.params, args) {
        env.define(name, self.exec_evaluation(arg)?);
      }
    */
    let mut index : i64 = 0;
    let limit = param_names.len() as i64;
    while index < limit {
      let name = &param_names[index as usize];
      env.define(name.clone(), Value::Integer(index));
      index = index + 1;
    }
    let lambda = Function::Lambda(None, Lambda {
      params: param_names,
      code: Box::new(self.compile_expression(Rc::new(RefCell::new(env)), body.clone())?),
    });

    Ok(Op::Enclose(lambda))
  }

  pub fn builtin_let_expression(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    args: Vec<Value<S>>,
  ) -> Result<Op<S>> {
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
      let val = self.compile_expression(env.clone(), val.clone())?;

      params.push(sym.as_symbol());
      args.push(val);
    }

    let mut inner_env = Env::extend(env.clone());
    let mut index : i64 = 0;
    let limit = params.len() as i64;
    while index < limit {
      let name = &params[index as usize];
      inner_env.define(name.clone(), Value::Integer(index));
      index = index + 1;
    }
    let code = self.compile_expression(Rc::new(RefCell::new(inner_env)), body.clone())?;

    let lambda = Function::Lambda(None, Lambda {
      params,
      code: Box::new(code),
    });

    Ok(Op::Apply(Box::new(Op::Enclose(lambda)), args))
  }
}
