use std::hash::Hash;

pub use string_interner::{symbol, backend};

pub trait Symbol: symbol::Symbol + Hash {}
pub trait Backend<S: Symbol>: backend::Backend<Symbol = S> {}

impl Symbol for symbol::SymbolU16 {}
impl Symbol for symbol::SymbolU32 {}
impl Symbol for symbol::SymbolUsize {}

impl<S: Symbol> Backend<S> for backend::SimpleBackend<S> {}
impl<S: Symbol> Backend<S> for backend::StringBackend<S> {}
impl<S: Symbol> Backend<S> for backend::BufferBackend<S> {}
impl<S: Symbol> Backend<S> for backend::BucketBackend<S> {}

pub use string_interner::StringInterner;
