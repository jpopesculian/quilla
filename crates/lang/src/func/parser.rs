use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::expr::{CalcError, Expr, ExprParseItem, LParen, Op, RParen};
use crate::span::{Span, Spanned};

use super::def::Func;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncParseError {
    ExpectedIdent,
    InvalidArg(CalcError),
}

impl core::fmt::Display for FuncParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FuncParseError::ExpectedIdent => write!(f, "expected identifier"),
            FuncParseError::InvalidArg(e) => write!(f, "invalid argument: {e}"),
        }
    }
}

impl core::error::Error for FuncParseError {}

/// A streaming parser that groups `ExprParseItem`s into `Func` values.
///
/// Feed items with [`feed`](FuncParser::feed), signal end-of-stream with
/// [`close`](FuncParser::close), then drain results with [`next_func`](FuncParser::next_func).
///
/// Each line is one function call: the first expression must be an identifier
/// (the function name) and any remaining expressions are evaluated as numeric
/// arguments.
pub struct FuncParser {
    items: VecDeque<ExprParseItem>,
    current: Vec<Expr>,
    eof: bool,
}

impl FuncParser {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
            current: Vec::new(),
            eof: false,
        }
    }

    pub fn feed(&mut self, item: ExprParseItem) {
        self.items.push_back(item);
    }

    pub fn close(&mut self) {
        self.eof = true;
    }

    pub fn next_func(&mut self) -> Result<Option<Func>, Spanned<FuncParseError>> {
        loop {
            match self.items.pop_front() {
                Some(ExprParseItem::Expr(expr)) => {
                    self.current.push(expr);
                }
                Some(ExprParseItem::Newline(_)) => {
                    if !self.current.is_empty() {
                        return self.flush();
                    }
                }
                None => {
                    if self.eof && !self.current.is_empty() {
                        return self.flush();
                    }
                    return Ok(None);
                }
            }
        }
    }

    fn flush(&mut self) -> Result<Option<Func>, Spanned<FuncParseError>> {
        let mut exprs = core::mem::take(&mut self.current).into_iter();

        let ident = match exprs.next() {
            Some(Expr::Ident(s)) => s,
            Some(expr) => {
                return Err(Spanned::new(FuncParseError::ExpectedIdent, expr_span(&expr)));
            }
            None => return Ok(None),
        };

        let mut args = Vec::new();
        while let Some(expr) = exprs.next() {
            let (calc_expr, arg_span) = match expr {
                Expr::Op(op) if matches!(op.inner, Op::Sub) => {
                    let op_span = op.span;
                    let next = exprs.next().ok_or_else(|| {
                        Spanned::new(
                            FuncParseError::InvalidArg(CalcError::MissingOperand),
                            op_span,
                        )
                    })?;
                    let next_span = expr_span(&next);
                    (
                        Expr::Group {
                            lparen: Spanned::new(LParen, op_span),
                            exprs: alloc::vec![Expr::Op(op), next],
                            rparen: Spanned::new(RParen, next_span),
                        },
                        Span { start: op_span.start, end: next_span.end },
                    )
                }
                expr => {
                    let span = expr_span(&expr);
                    (expr, span)
                }
            };
            let num = calc_expr
                .calc()
                .map_err(|e| Spanned::new(FuncParseError::InvalidArg(e.inner), e.span))?;
            args.push(Spanned::new(num, arg_span));
        }

        Ok(Some(Func { ident, args }))
    }
}

impl Default for FuncParser {
    fn default() -> Self {
        Self::new()
    }
}

fn expr_span(expr: &Expr) -> Span {
    match expr {
        Expr::Ident(s) => s.span,
        Expr::Num(s) => s.span,
        Expr::Op(s) => s.span,
        Expr::Group { lparen, rparen, .. } => Span {
            start: lparen.span.start,
            end: rparen.span.end,
        },
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;
    use crate::expr::ExprParser;
    use crate::lexer::Lexer;
    use crate::num::Num;
    use crate::token::Token;

    fn lex(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new();
        lexer.feed(input.as_bytes());
        lexer.close();
        let mut tokens = Vec::new();
        while let Some(tok) = lexer.next_token().unwrap() {
            tokens.push(tok);
        }
        tokens
    }

    fn parse_funcs(input: &str) -> Vec<Func> {
        let mut expr_parser = ExprParser::new();
        for tok in lex(input) {
            expr_parser.feed(tok);
        }
        expr_parser.close();

        let mut func_parser = FuncParser::new();
        while let Some(item) = expr_parser.next_expr().unwrap() {
            func_parser.feed(item);
        }
        func_parser.close();

        let mut funcs = Vec::new();
        while let Some(func) = func_parser.next_func().unwrap() {
            funcs.push(func);
        }
        funcs
    }

    #[test]
    fn test_empty() {
        assert_eq!(parse_funcs("").len(), 0);
    }

    #[test]
    fn test_no_args() {
        let funcs = parse_funcs("h\n");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].ident.inner, "h");
        assert_eq!(funcs[0].args.len(), 0);
    }

    #[test]
    fn test_int_arg() {
        let funcs = parse_funcs("rx 1\n");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].ident.inner, "rx");
        assert_eq!(funcs[0].args.len(), 1);
        assert!(matches!(funcs[0].args[0].inner, Num::Int(1)));
    }

    #[test]
    fn test_multiple_args() {
        let funcs = parse_funcs("cx 0 1\n");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].ident.inner, "cx");
        assert_eq!(funcs[0].args.len(), 2);
        assert!(matches!(funcs[0].args[0].inner, Num::Int(0)));
        assert!(matches!(funcs[0].args[1].inner, Num::Int(1)));
    }

    #[test]
    fn test_group_arg() {
        let funcs = parse_funcs("rx (pi / 2)\n");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].ident.inner, "rx");
        assert_eq!(funcs[0].args.len(), 1);
        let Num::Float(f) = funcs[0].args[0].inner else {
            panic!("expected float");
        };
        assert!((f - core::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiple_funcs() {
        let funcs = parse_funcs("h 0\ncx 0 1\n");
        assert_eq!(funcs.len(), 2);
        assert_eq!(funcs[0].ident.inner, "h");
        assert_eq!(funcs[1].ident.inner, "cx");
    }

    #[test]
    fn test_eof_without_newline() {
        let funcs = parse_funcs("h 0");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].ident.inner, "h");
        assert_eq!(funcs[0].args.len(), 1);
    }

    #[test]
    fn test_blank_lines_skipped() {
        let funcs = parse_funcs("h\n\ncx 0 1\n");
        assert_eq!(funcs.len(), 2);
        assert_eq!(funcs[0].ident.inner, "h");
        assert_eq!(funcs[1].ident.inner, "cx");
    }

    #[test]
    fn test_expected_ident_error() {
        let mut expr_parser = ExprParser::new();
        for tok in lex("42 1\n") {
            expr_parser.feed(tok);
        }
        expr_parser.close();

        let mut func_parser = FuncParser::new();
        while let Some(item) = expr_parser.next_expr().unwrap() {
            func_parser.feed(item);
        }
        func_parser.close();

        let err = func_parser.next_func().unwrap_err();
        assert!(matches!(err.inner, FuncParseError::ExpectedIdent));
    }

    #[test]
    fn test_invalid_arg_error() {
        let mut expr_parser = ExprParser::new();
        for tok in lex("rx foo\n") {
            expr_parser.feed(tok);
        }
        expr_parser.close();

        let mut func_parser = FuncParser::new();
        while let Some(item) = expr_parser.next_expr().unwrap() {
            func_parser.feed(item);
        }
        func_parser.close();

        let err = func_parser.next_func().unwrap_err();
        assert!(matches!(
            err.inner,
            FuncParseError::InvalidArg(CalcError::UnknownConstant(_))
        ));
    }

    #[test]
    fn test_streaming_feed_one_by_one() {
        let tokens = lex("rx 1\n");
        let mut expr_parser = ExprParser::new();
        let mut func_parser = FuncParser::new();

        // Feed "rx" and "1" but not the newline yet — no func should be emitted
        for tok in tokens[..tokens.len() - 1].iter().cloned() {
            expr_parser.feed(tok);
            while let Some(item) = expr_parser.next_expr().unwrap() {
                func_parser.feed(item);
            }
        }
        assert!(func_parser.next_func().unwrap().is_none());

        // Feed the newline — func should now be ready
        expr_parser.feed(tokens[tokens.len() - 1].clone());
        expr_parser.close();
        while let Some(item) = expr_parser.next_expr().unwrap() {
            func_parser.feed(item);
        }
        func_parser.close();

        let func = func_parser.next_func().unwrap().unwrap();
        assert_eq!(func.ident.inner, "rx");
        assert_eq!(func.args.len(), 1);
        assert!(func_parser.next_func().unwrap().is_none());
    }

    #[test]
    fn test_unary_minus_produces_two_args() {
        // "foo 1 - 2" → ident "foo", args [1, -2]
        let funcs = parse_funcs("foo 1 - 2\n");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].ident.inner, "foo");
        assert_eq!(funcs[0].args.len(), 2);
        assert!(matches!(funcs[0].args[0].inner, Num::Int(1)));
        assert!(matches!(funcs[0].args[1].inner, Num::Int(-2)));
    }

    #[test]
    fn test_unary_minus_only_arg() {
        // "foo - 1" → ident "foo", args [-1]
        let funcs = parse_funcs("foo - 1\n");
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].args.len(), 1);
        assert!(matches!(funcs[0].args[0].inner, Num::Int(-1)));
    }

    #[test]
    fn test_unary_plus_is_error() {
        let mut expr_parser = ExprParser::new();
        for tok in lex("foo + 3\n") {
            expr_parser.feed(tok);
        }
        expr_parser.close();

        let mut func_parser = FuncParser::new();
        while let Some(item) = expr_parser.next_expr().unwrap() {
            func_parser.feed(item);
        }
        func_parser.close();

        let err = func_parser.next_func().unwrap_err();
        assert!(matches!(
            err.inner,
            FuncParseError::InvalidArg(CalcError::BareOperator)
        ));
    }

    #[test]
    fn test_unary_minus_missing_operand() {
        let mut expr_parser = ExprParser::new();
        for tok in lex("foo -\n") {
            expr_parser.feed(tok);
        }
        expr_parser.close();

        let mut func_parser = FuncParser::new();
        while let Some(item) = expr_parser.next_expr().unwrap() {
            func_parser.feed(item);
        }
        func_parser.close();

        let err = func_parser.next_func().unwrap_err();
        assert!(matches!(
            err.inner,
            FuncParseError::InvalidArg(CalcError::MissingOperand)
        ));
    }
}
