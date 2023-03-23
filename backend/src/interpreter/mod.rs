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

pub type RtArgs<S> = (Rc<RefCell<Env<S>>>, Rc<RefCell<RTE<S>>>, Rc<Op<S>>);
pub type RtThunk<S> = Promise<RtArgs<S>, Result<Value<S>>>;

pub fn make_delay<S: Symbol>(gle: Rc<RefCell<Env<S>>>, rte: Rc<RefCell<RTE<S>>>, func: Rc<Op<S>>) -> RtArgs<S> {
  return (gle, rte, func)
}

pub fn make_op_global_ref<S: Symbol>(sym: S) -> RtOp<S> { return Rc::new(Op::FetchGle(sym)) }
pub fn make_op_lexical_ref<S: Symbol>(depth: i64, index: i64) -> RtOp<S> {
 return Rc::new(Op::RefRTE(depth as usize, index as usize))
}
pub fn make_op_if<S: Symbol>(test: RtOp<S>, then: RtOp<S>, other: RtOp<S>) -> RtOp<S> { return Rc::new(Op::If(test, then, other)) }
pub fn make_op_finish<S: Symbol>(value: Value<S>) -> RtOp<S> { return Rc::new(Op::Finish(value)) }
pub fn make_op_enclose<S: Symbol>(func: Function<S>) -> RtOp<S> { return Rc::new(Op::Enclose(func)) }
pub fn make_op_apply<S: Symbol>(func: RtOp<S>, args: Vec<RtOp<S>>) -> RtOp<S> { return Rc::new(Op::Apply(func, args)) }
pub fn make_op_println<S: Symbol>(args: Vec<RtOp<S>>) -> RtOp<S> { return Rc::new(Op::PRINTLN(args)) }


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
        Function::NativeFn(native_func) => {
          format!("[function {:p}]", native_func)
        },
        Function::Lambda(lambda) => {
          format!("[function {:p} code {:p}]", lambda, lambda.code.as_ref())
        },
        Function::Closure(rte, arity, code) => {
          format!("[closure arity {:}code {:p} {:}]", arity, code.as_ref(), rte.borrow().format())
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
      input,
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
  ) -> Result<RtOp<S>> {
    match expression {
      Value::Symbol(sym) => {
        let sym = sym.as_symbol();
        let (depth, index) = env.borrow().lookup(sym);
        if depth==-1 {
          if index == -1 {
            Err(RuntimeError::UndefinedSymbol {
                detail: self.interner.resolve(sym).unwrap_or("<>").to_string()
            })
          } else {
            Ok(make_op_global_ref(sym))
          }
        } else {
          Ok(make_op_lexical_ref(depth, index))
        }
      },
      Value::List(list) if !list.empty() => {
        let func = list.car()?;
        let args: Vec<Value<S>> = list.cdr().into_iter().collect();

        if let Value::Symbol(sym) = &func {
          let sym = sym.as_symbol();
          let func_name = self.interner.resolve(sym).unwrap_or("<>");

          match func_name {
            "println" => return self.builtin_println(env, args),
            "quote" => return Ok(make_op_finish(self.builtin_quote(args)?)),
            "def" => return Ok(make_op_finish(self.builtin_define(env, args)?)),
            "set!" => return Ok(make_op_finish(self.builtin_set(env, args)?)),
            "if" => return self.builtin_controlflow_if(env, args),
            "lambda" => return self.builtin_lambda(env, args),
            "let" => return self.builtin_let_expression(env, args),
            _ => {},
          }
        }

        let func = self.compile_expression(env.clone(), func)?;

        let mut eval_args = Vec::with_capacity(args.len());

        for arg in args {
          let arg = self.compile_expression(env.clone(), arg)?;
          eval_args.push(arg);
        }

        Ok(make_op_apply(func, eval_args))
      },
      _ => {
        Ok(make_op_finish(expression))
      },
    }
  }

  pub fn enclose(&mut self, rte: Rc<RefCell<RTE<S>>>, func: &Function<S>) -> Value<S> {
    // println!(" Enclose {:} ", rte.borrow().format());
    match func {
      Function::Lambda(lambda) => {
        return Value::Function(Function::Closure(rte, lambda.params.len(), lambda.code.clone())) // cloning Rc
      }
      Function::NativeFn(_native_func) => {
        return Value::Function(func.clone()) // FIXME deep copy
      }
      Function::Closure(_rte, arity, code) => { // FIXME: is this actually allowed?
        println!("enclose: questionable case");
        return Value::Function(Function::Closure(rte, *arity, code.clone())) // cloning Rc
      }
    }
  }

  pub fn ret42(&mut self, content: RtArgs<S>) -> RtThunk<S> {
  let (gle, rte, op) = content;
  match op.as_ref() {
    Op::RefRTE(depth, index) => {
      // println!("RefRTE {:} depth {:} index {:}", rte.borrow().format(), depth, index);
      if let Some(value) = rte.borrow().get(*depth, *index) {
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
      match self.exec_evaluation(&gle, rte.clone(), test) {
        Ok(Value::Boolean(val)) => {
          if val {return Delay((gle, rte, then.clone()))}
          else {return Delay((gle, rte, otherwise.clone()))}
        }
        Ok(_) => {return Delay((gle, rte, then.clone()))}
        other => Done(other)
      }
    }
    Op::Enclose(func) => { return Done(Ok(self.enclose(rte, func))) }
    Op::Apply(op, args) => {
      // println!("Op::Apply {:}", rte.borrow().format());
      let args_count = args.len();
      let mut val_args : Vec<Value<S>> = Vec::with_capacity(args_count);

      for arg in args {
         match self.exec_evaluation(&gle, rte.clone(), arg) {
           Ok(arg) => { val_args.push(arg) }
           err => { return Done(err) }
         }
       }

       match self.exec_evaluation(&gle, rte, op) {
         Ok(Value::Function(Function::NativeFn(native_func))) => {
          match native_func(gle.clone(), val_args) {
            Ok(val) => { return Done(Ok(val)) }
            err => { return Done(err) }
          }
        }

        Ok(Value::Function(Function::Closure(ref irte, arity, ref code))) => {
          match assert_exactly_args(arity, args_count) {
            Err(err) => { return Done(Err(err)) }
            _ => {}
          }
          let rte = RTE::extend(irte.clone(), val_args);
          Delay((gle, rte, code.clone()))
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
      return Done(Ok(val.clone()))
    }
    Op::PRINTLN(args) => { // FIXME: can't get that to work properly
      // println!("Op::PRINTLN {:}", rte.borrow().format());
      let args_count = args.len();
      let mut val_args : Vec<Value<S>> = Vec::with_capacity(args_count);

      for arg in args {
         match self.exec_evaluation(&gle, rte.clone(), arg) {
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
    global_env: &Rc<RefCell<Env<S>>>,
    env: Rc<RefCell<RTE<S>>>,
    todo: &Rc<Op<S>>
  ) -> Result<Value<S>> {
    let ret23 = |rt| self.ret42(rt);
    return force_promise(ret23, make_delay(global_env.clone(), env, todo.clone()))
  }

  pub fn eval_expression(
    &mut self,
    env: Rc<RefCell<Env<S>>>,
    expression: Value<S>,
  ) -> Result<Value<S>> {
    let todo = self.compile_expression(env.clone(), expression)?;
// println!("running now");
    let rte = RTE::new();
    self.exec_evaluation(&env, rte, &todo)
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
