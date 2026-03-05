use alloc::vec::Vec;

use crate::expr::{ExprParseError, ExprParser};
use crate::func::{FuncParseError, FuncParser};
use crate::instruction::{Instruction, InstructionError};
use crate::lexer::{Lexer, LexerError};
use crate::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Lex(LexerError),
    Expr(ExprParseError),
    Func(FuncParseError),
    Instruction(InstructionError),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::Lex(e) => write!(f, "{e}"),
            ParseError::Expr(e) => write!(f, "{e}"),
            ParseError::Func(e) => write!(f, "{e}"),
            ParseError::Instruction(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ParseError::Lex(e) => Some(e),
            ParseError::Expr(e) => Some(e),
            ParseError::Func(e) => Some(e),
            ParseError::Instruction(e) => Some(e),
        }
    }
}

/// Parse a complete `&str` into a list of [`Instruction`]s.
///
/// Runs the lexer, expression parser, function parser, and instruction
/// conversion in sequence, returning all instructions found or the first
/// error encountered.
pub fn parse(input: &str) -> Result<Vec<Spanned<Instruction>>, Spanned<ParseError>> {
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

    let mut instrs = Vec::new();
    while let Some(func) = func_parser
        .next_func()
        .map_err(|e| Spanned::new(ParseError::Func(e.inner), e.span))?
    {
        let instr = Spanned::<Instruction>::try_from(func)
            .map_err(|e| Spanned::new(ParseError::Instruction(e.inner), e.span))?;
        instrs.push(instr);
    }
    Ok(instrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(parse("").unwrap().len(), 0);
    }

    #[test]
    fn test_single_func() {
        let instrs = parse("h 0\n").unwrap();
        assert_eq!(instrs.len(), 1);
        assert!(matches!(instrs[0].inner, Instruction::H { target: 0 }));
    }

    #[test]
    fn test_multiple_funcs() {
        let instrs = parse("h 0\ncx 0 1\n").unwrap();
        assert_eq!(instrs.len(), 2);
        assert!(matches!(instrs[0].inner, Instruction::H { .. }));
        assert!(matches!(instrs[1].inner, Instruction::CX { .. }));
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

    #[test]
    fn test_instruction_error() {
        let err = parse("foo 0\n").unwrap_err();
        assert!(matches!(
            err.inner,
            ParseError::Instruction(InstructionError::UnknownGate(_))
        ));
    }
}
