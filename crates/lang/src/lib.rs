#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod expr;
pub mod func;
pub mod lexer;
pub mod num;
pub mod parse;
pub mod span;
pub mod token;

pub use parse::{ParseError, parse};
