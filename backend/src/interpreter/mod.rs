use std::{rc::Rc, cell::RefCell};

use lispers_common::{StringInterner, Backend, Symbol};
use lispers_frontend::{SExpression, Literal};
use crate::prelude::*;
use crate::data::{Value, List, Function};
use crate::env::{Env, default_env, RTE};

use crate::utils::assert_exactly_args;

pub struct Interpreter<S: Symbol, B: Backend<S>> {
  interner: StringInterner<B>,
  marker: std::marker::PhantomData<S>,
}

// FIXME: last arg Vec<Value<S>> is actually unused!
pub type RtArgs<S> = (Rc<RefCell<Env<S>>>, Rc<RefCell<RTE<S>>>, Op<S>);
pub type RtThunk<S> = Promise<RtArgs<S>, Result<Value<S>>>;

pub fn make_delay<S: Symbol>(gle: Rc<RefCell<Env<S>>>, rte: Rc<RefCell<RTE<S>>>, func: Op<S>) -> RtArgs<S> {
  return (gle, rte, func)
}

mod builtins;

impl<S: Symbol, B: Backend<S>> Interpreter<S, B> {
  pub fn new() -> Self {
    let result =
    Self {
      interner: StringInterner::new(),
      marker: std::marker::PhantomData{},
    };
    return result
  }

  pub fn default_env(&mut self) -> Rc<RefCell<Env<S>>> {
    let result = Rc::new(RefCell::new(default_env(&mut self.interner)));
    /*
  PRINTLN(Vec<Op<S>>), // FIXME: can't get that to work properly
    let func = move |rte, args| self.builtin_println_rt2(rte, args);
    result.borrow().define(
      self.interner.get_or_intern("println"),
      Value::Function(Function::NativeFn(None, func)),
    );
    // */
    return result
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
        Function::NativeFn(rte, native_func) => {
          // format!("[function {:p}]", native_func)
          let env = match rte {
            Some(rte) => { rte.borrow().format() }
            _ => { "None".to_string() }
          };
          format!("[function native {:p} {:}]", native_func, env)
        },
        Function::Lambda(rte, lambda) => {
          let env = match rte {
            Some(rte) => { rte.borrow().format() }
            _ => { "None".to_string() }
          };
          format!("[function {:p} code {:p} {:}]", lambda, lambda.code.as_ref(), env)
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

  pub fn compile_expression(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    expression: Value<S>,
  ) -> Result<Op<S>> {
    match expression {
      Value::Symbol(sym) => {
        let sym = sym.as_symbol();
        let (depth, index) = env.borrow().lookup(sym.clone());
        if depth==-1 {
          if index == -1 {
            Err(RuntimeError::UndefinedSymbol {
                detail: self.interner.resolve(sym).unwrap_or("<>").to_string()
            })
          } else {
            Ok(Op::FetchGle(sym))
          }
        } else {
          Ok(Op::RefRTE(depth as usize, index as usize))
        }
      },
      Value::List(list) if !list.empty() => {
        let func = list.car()?;
        let args: Vec<Value<S>> = list.cdr().into_iter().collect();

        if let Value::Symbol(sym) = &func {
          let sym = sym.as_symbol();
          let func_name = self.interner.resolve(sym).unwrap_or("<>");

          match func_name {
            "println" => return self.builtin_println(env.clone(), args),
            "quote" => return Ok(Op::Finish(self.builtin_quote(args)?)),
            "def" => return Ok(Op::Finish(self.builtin_define(env.clone(), args)?)),
            "set!" => return Ok(Op::Finish(self.builtin_set(env.clone(), args)?)),
            "if" => return self.builtin_controlflow_if(env, args),
            "lambda" => return self.builtin_lambda(env, args),
            "let" => return self.builtin_let_expression(env.clone(), args),
            _ => {},
          }
        }

        let func = Box::new(self.compile_expression(env.clone(), func)?);

        let mut eval_args = Vec::with_capacity(args.len());

        for arg in args {
          let arg = self.compile_expression(env.clone(), arg)?;
          eval_args.push(arg);
        }

        Ok(Op::Apply(func, eval_args))
      },
      _ => {
        Ok(Op::Finish(expression))
      },
    }
  }

  pub fn enclose(&mut self, rte: Rc<RefCell<RTE<S>>>, func: Function<S>) -> Value<S> {
    // println!(" Enclose {:} ", rte.borrow().format());
    match func {
      Function::Lambda(_rte, lambda) => {
        return Value::Function(Function::Lambda(Some(rte), lambda))
      }
      Function::NativeFn(_rte, native_func) => {
        return Value::Function(Function::NativeFn(Some(rte), native_func))
      }
    }
  }

  pub fn ret42(&mut self, content: RtArgs<S>) -> RtThunk<S> {
  let (gle, rte, op) = content;
  match op {
    Op::RefRTE(depth, index) => {
      // println!("RefRTE {:} depth {:} index {:}", rte.borrow().format(), depth, index);
      if let Some(value) = rte.borrow().get(depth, index) {
        return Done(Ok(value))
      } else {
        return Done(Err(RuntimeError::UndefinedSymbol{detail: "detail lost at runtime".to_string()} ))
      }
    }
    Op::FetchGle(ref sym) => {
      // println!("Op::FetchGle {:} sym {:}", rte.borrow().format(), self.interner.resolve(*sym).unwrap_or("<>").to_string());
      return Done(gle.borrow().get(*sym).ok_or_else(|| RuntimeError::UndefinedSymbol {
             detail: self.interner.resolve(*sym).unwrap_or("<>").to_string()
             }))
    }
    Op::If(test, then, otherwise) => {
      // println!("If {:} ", rte.borrow().format());
      match self.exec_evaluation(gle.clone(), rte.clone(), test.as_ref()) {
        Ok(Value::Boolean(val)) => {
          if val {return Delay((gle, rte, *then))}
          else {return Delay((gle, rte, *otherwise))}
        }
        Ok(_) => {return Delay((gle, rte, *then))}
        other => Done(other)
      }
    }
    Op::Enclose(func) => { return Done(Ok(self.enclose(rte, func))) }
    Op::Apply(op, args) => {
      // println!("Op::Apply {:}", rte.borrow().format());
      let args_count = args.len();
      let mut val_args : Vec<Value<S>> = Vec::with_capacity(args_count);
      let mut val_args_again : Vec<Value<S>> = Vec::with_capacity(args_count); // FIXME: can we remove this duplication?

      for arg in args {
         match self.exec_evaluation(gle.clone(), rte.clone(), &arg) {
           Ok(arg) => {
             val_args_again.push(arg.clone()); val_args.push(arg) }
           err => { return Done(err) }
         }
       }

       match self.exec_evaluation(gle.clone(), rte.clone(), &*op) {
         Ok(Value::Function(Function::NativeFn(irte, native_func))) => {
          let rte = match irte { None => { rte }, Some(ref rte) => rte.clone() };
          let _rte = RTE::extend(rte.clone(), val_args);

          match native_func(gle.clone(), val_args_again) {
            Ok(val) => { return Done(Ok(val)) }
            err => { return Done(err) }
          }
        }

        Ok(Value::Function(Function::Lambda(ref irte, ref lambda))) => {
          match assert_exactly_args(lambda.params.len(), args_count) {
            Err(err) => { return Done(Err(err)) }
            _ => {}
          }
          let rte = match irte {
            None => {return Done(Err(RuntimeError::NYIE{detail: "should not happen 69".to_string()}))},
            Some(ref rte) => rte.clone()
          };
          let rte = RTE::extend(rte.clone(), val_args);
          Delay((gle, rte, *lambda.code.clone()))
        }
        Ok(value) => {
          let func : Result<Function<S>> = value.try_into();
          match func {
            Err(err) => { return Done(Err(err)) }
            Ok(_func) => {return Done(Err(RuntimeError::NYIE{detail: "ret42 strange case 69a".to_string()}))}
          }
        }
        Err(err) => { return Done(Err(err)) }
      }
    }
    Op::Finish(val) => {
      // println!("Op::Finish {:} => {}", rte.borrow().format(), self.format_value(&val));
      return Done(Ok(val))
    }
    Op::PRINTLN(args) => { // FIXME: can't get that to work properly
      // println!("Op::PRINTLN {:}", rte.borrow().format());
      let args_count = args.len();
      let mut val_args : Vec<Value<S>> = Vec::with_capacity(args_count);

      for arg in args {
         match self.exec_evaluation(gle.clone(), rte.clone(), &arg) {
           Ok(arg) => { val_args.push(arg) }
           err => { return Done(err) }
         }
      }

      let output = val_args
        .iter()
        .map(|val| self.format_value(val))
        .collect::<Vec<String>>()
        .join(" ");

      println!("{}", output);
      Done(Ok(Value::default()))
    }
    // _ => {return Done(Err(RuntimeError::NYIE{detail: "ret42 Opcode".to_string()}))}
    }
  }

  pub fn exec_evaluation(
    &mut self,
    global_env: Rc<RefCell<Env<S>>>,
    env: Rc<RefCell<RTE<S>>>,
    todo: &Op<S>
  ) -> Result<Value<S>> {
    let ret23 = |rt| self.ret42(rt);
    return force_promise(ret23, make_delay(global_env, env, todo.clone()))
  }

  pub fn eval_expression(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    expression: Value<S>,
  ) -> Result<Value<S>> {
    let todo = self.compile_expression(env.clone(), expression)?;
// println!("running now");
    let rte = RTE::new();
    self.exec_evaluation(env, rte, &todo)
  }

  fn parse_sexpression(&self, sexpression: &SExpression<S>) -> Result<Value<S>> {
    // FIXME: Am I just too new to rust or do we traverse the tree twice here?
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
