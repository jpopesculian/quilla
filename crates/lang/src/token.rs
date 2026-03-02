use alloc::string::String;

use crate::num::Num;
use crate::span::{Span, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Newline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(Spanned<String>),
    Num(Spanned<Num>),
    Symbol(Spanned<Symbol>),
}

impl Token {
    #[inline]
    pub fn ident(inner: String, span: Span) -> Self {
        Self::Ident(Spanned::new(inner, span))
    }

    #[inline]
    pub fn num(inner: Num, span: Span) -> Self {
        Self::Num(Spanned::new(inner, span))
    }

    #[inline]
    pub fn symbol(inner: Symbol, span: Span) -> Self {
        Self::Symbol(Spanned::new(inner, span))
    }

    pub fn span(&self) -> &Span {
        match self {
            Self::Ident(Spanned { span, .. })
            | Self::Num(Spanned { span, .. })
            | Self::Symbol(Spanned { span, .. }) => span,
        }
    }
}
