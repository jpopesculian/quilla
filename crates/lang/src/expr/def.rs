use alloc::string::String;
use alloc::vec::Vec;

use crate::num::Num;
use crate::span::Spanned;

pub enum Expr {
    Ident(Spanned<String>),
    Num(Spanned<Num>),
    Op(Spanned<Op>),
    Group {
        lparen: Spanned<LParen>,
        exprs: Vec<Expr>,
        rparen: Spanned<RParen>,
    },
}

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub struct LParen;
pub struct RParen;
