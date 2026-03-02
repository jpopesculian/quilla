use alloc::string::String;
use alloc::vec::Vec;

use crate::num::Num;
use crate::span::Spanned;

#[derive(Debug)]
pub struct Func {
    pub ident: Spanned<String>,
    pub args: Vec<Spanned<Num>>,
}
