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

#[derive(Debug, Clone)]
pub enum ParseItem {
    Instruction(Instruction),
    Error(ParseError),
}

/// Parse a complete `&str` into a list of [`Instruction`]s.
///
/// Runs the lexer, expression parser, function parser, and instruction
/// conversion in sequence, returning all instructions found or the first
/// error encountered.
pub fn parse(input: &str) -> Vec<Spanned<Result<Instruction, ParseError>>> {
    let mut results = Vec::new();

    let mut lexer = Lexer::new();
    lexer.feed(input.as_bytes());
    lexer.close();

    let mut expr_parser = ExprParser::new();
    loop {
        match lexer.next_token() {
            Ok(Some(token)) => expr_parser.feed(token),
            Ok(None) => break,
            Err(err) => {
                let is_recoverable = err.inner.is_recoverable();
                results.push(err.map(ParseError::Lex).map(Err));
                if !is_recoverable {
                    break;
                }
            }
        }
    }
    expr_parser.close();

    let mut func_parser = FuncParser::new();
    loop {
        match expr_parser.next_expr() {
            Ok(Some(item)) => func_parser.feed(item),
            Ok(None) => break,
            Err(err) => results.push(err.map(ParseError::Expr).map(Err)),
        }
    }
    func_parser.close();

    loop {
        match func_parser.next_func() {
            Ok(Some(func)) => match Spanned::<Instruction>::try_from(func) {
                Ok(instr) => results.push(instr.map(Ok)),
                Err(err) => results.push(err.map(ParseError::Instruction).map(Err)),
            },
            Ok(None) => break,
            Err(err) => results.push(err.map(ParseError::Func).map(Err)),
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(parse("").len(), 0);
    }

    #[test]
    fn test_single_func() {
        let results = parse("h 0\n");
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0].inner, Ok(Instruction::H { target: 0 })));
    }

    #[test]
    fn test_multiple_funcs() {
        let results = parse("h 0\ncx 0 1\n");
        assert_eq!(results.len(), 2);
        assert!(matches!(results[0].inner, Ok(Instruction::H { .. })));
        assert!(matches!(results[1].inner, Ok(Instruction::CX { .. })));
    }

    #[test]
    fn test_lex_error() {
        let results = parse("h @\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Lex(LexerError::UnexpectedChar { .. }))
        ));
    }

    #[test]
    fn test_expr_error() {
        let results = parse("h (0\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Expr(ExprParseError::UnclosedParenthesis))
        ));
    }

    #[test]
    fn test_func_error() {
        let results = parse("rx foo\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Func(FuncParseError::InvalidArg(_)))
        ));
    }

    #[test]
    fn test_instruction_error() {
        let results = parse("foo 0\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Instruction(InstructionError::UnknownGate(_)))
        ));
    }

    #[test]
    fn test_unexpected_rparen() {
        let results = parse("h )\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Expr(ExprParseError::UnexpectedRParen))
        ));
    }

    #[test]
    fn test_expected_ident() {
        let results = parse("0 1\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Func(FuncParseError::ExpectedIdent))
        ));
    }

    #[test]
    fn test_wrong_arg_count() {
        let results = parse("h 0 1\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Instruction(InstructionError::WrongArgCount {
                expected: 1,
                got: 2
            }))
        ));
    }

    #[test]
    fn test_invalid_index() {
        let results = parse("h 1.5\n");
        let err = results.iter().find(|r| r.inner.is_err()).unwrap();
        assert!(matches!(
            err.inner,
            Err(ParseError::Instruction(InstructionError::InvalidIndex))
        ));
    }
}
