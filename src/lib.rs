#![feature(core, collections)]
//! For usage examples, please have a look at the *tests* and *benchmarks*.
mod lexer;
mod key_value_filter;
mod reader;
mod iter_ext;

pub use lexer::{Lexer, Token, TokenType, Span, BufferType, Buffer};
pub use key_value_filter::{FilterTypedKeyValuePairs};
pub use reader::{TokenReader};
pub use iter_ext::IteratorExt;