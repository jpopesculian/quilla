use alloc::vec::Vec;

use crate::expr::{ExprParseError, ExprParser};
use crate::func::{Func, FuncParseError, FuncParser};
use crate::lexer::{Lexer, LexerError};
use crate::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Lex(LexerError),
    Expr(ExprParseError),
    Func(FuncParseError),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::Lex(e) => write!(f, "{e}"),
            ParseError::Expr(e) => write!(f, "{e}"),
            ParseError::Func(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for ParseError {}

/// Parse a complete `&str` into a list of [`Func`]s.
///
/// Runs the lexer, expression parser, and function parser in sequence,
/// returning all functions found or the first error encountered.
pub fn parse(input: &str) -> Result<Vec<Func>, Spanned<ParseError>> {
    let mut lexer = Lexer::new();
    lexer.feed(input.as_bytes());
    lexer.close();

    let mut expr_parser = ExprParser::new();
    while let Some(token) = lexer
        .next_token()
        .map_err(|e| Spanned::new(ParseError::Lex(e.inner), e.span))?
    {
        expr_parser.feed(token);
    }
    expr_parser.close();

    let mut func_parser = FuncParser::new();
    while let Some(item) = expr_parser
        .next_expr()
        .map_err(|e| Spanned::new(ParseError::Expr(e.inner), e.span))?
    {
        func_parser.feed(item);
    }
    func_parser.close();

    let mut funcs = Vec::new();
    while let Some(func) = func_parser
        .next_func()
        .map_err(|e| Spanned::new(ParseError::Func(e.inner), e.span))?
    {
        funcs.push(func);
    }
    Ok(funcs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::Num;

    #[test]
    fn test_empty() {
        assert_eq!(parse("").unwrap().len(), 0);
    }

    #[test]
    fn test_single_func() {
        let funcs = parse("h 0\n").unwrap();
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].ident.inner, "h");
        assert!(matches!(funcs[0].args[0].inner, Num::Int(0)));
    }

    #[test]
    fn test_multiple_funcs() {
        let funcs = parse("h 0\ncx 0 1\n").unwrap();
        assert_eq!(funcs.len(), 2);
        assert_eq!(funcs[0].ident.inner, "h");
        assert_eq!(funcs[1].ident.inner, "cx");
    }

    #[test]
    fn test_lex_error() {
        let err = parse("h @\n").unwrap_err();
        assert!(matches!(
            err.inner,
            ParseError::Lex(LexerError::UnexpectedChar { .. })
        ));
    }

    #[test]
    fn test_expr_error() {
        let err = parse("h (0\n").unwrap_err();
        assert!(matches!(
            err.inner,
            ParseError::Expr(ExprParseError::UnclosedParenthesis)
        ));
    }

    #[test]
    fn test_func_error() {
        let err = parse("rx foo\n").unwrap_err();
        assert!(matches!(
            err.inner,
            ParseError::Func(FuncParseError::InvalidArg(_))
        ));
    }
}
