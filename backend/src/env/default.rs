use lispers_common::{StringInterner, Backend, Symbol};
use crate::data::{Value, Function};
use super::{Env, primitives};


pub fn default_env<S, B>(interner: &mut StringInterner<B>) -> Env<S>
  where
    S: Symbol,
    B: Backend<S>
{
  let mut env = Env::new();

  env.define(
    interner.get_or_intern("+"),
    Value::Function(Function::NativeFn(primitives::arithmetic::iadd)),
  );
  env.define(
    interner.get_or_intern("-"),
    Value::Function(Function::NativeFn(primitives::arithmetic::isub)),
  );
  env.define(
    interner.get_or_intern("*"),
    Value::Function(Function::NativeFn(primitives::arithmetic::imul)),
  );
  env.define(
    interner.get_or_intern("/"),
    Value::Function(Function::NativeFn(primitives::arithmetic::idiv)),
  );
  env.define(
    interner.get_or_intern(".+"),
    Value::Function(Function::NativeFn(primitives::arithmetic::fadd)),
  );
  env.define(
    interner.get_or_intern(".-"),
    Value::Function(Function::NativeFn(primitives::arithmetic::fsub)),
  );
  env.define(
    interner.get_or_intern(".*"),
    Value::Function(Function::NativeFn(primitives::arithmetic::fmul)),
  );
  env.define(
    interner.get_or_intern("./"),
    Value::Function(Function::NativeFn(primitives::arithmetic::fdiv)),
  );
  env.define(
    interner.get_or_intern("<"),
    Value::Function(Function::NativeFn(primitives::comparison::ilt)),
  );
  env.define(
    interner.get_or_intern("<="),
    Value::Function(Function::NativeFn(primitives::comparison::ilte)),
  );
  env.define(
    interner.get_or_intern(">="),
    Value::Function(Function::NativeFn(primitives::comparison::igte)),
  );
  env.define(
    interner.get_or_intern(">"),
    Value::Function(Function::NativeFn(primitives::comparison::igt)),
  );
  env.define(
    interner.get_or_intern(".<"),
    Value::Function(Function::NativeFn(primitives::comparison::flt)),
  );
  env.define(
    interner.get_or_intern(".<="),
    Value::Function(Function::NativeFn(primitives::comparison::flte)),
  );
  env.define(
    interner.get_or_intern(".>="),
    Value::Function(Function::NativeFn(primitives::comparison::fgte)),
  );
  env.define(
    interner.get_or_intern(".>"),
    Value::Function(Function::NativeFn(primitives::comparison::fgt)),
  );
  env.define(
    interner.get_or_intern("="),
    Value::Function(Function::NativeFn(primitives::comparison::eq)),
  );
  env.define(
    interner.get_or_intern("!="),
    Value::Function(Function::NativeFn(primitives::comparison::ne)),
  );
  env.define(
    interner.get_or_intern("exit"),
    Value::Function(Function::NativeFn(primitives::proc::exit)),
  );

  env
}
