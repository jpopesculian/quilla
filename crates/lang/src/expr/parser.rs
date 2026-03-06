use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::span::{Span, Spanned};
use crate::token::{Symbol, Token};

use super::def::{Expr, LParen, Op, RParen};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprParseError {
    UnclosedParenthesis,
    UnexpectedRParen,
}

impl core::fmt::Display for ExprParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExprParseError::UnclosedParenthesis => write!(f, "unclosed parenthesis"),
            ExprParseError::UnexpectedRParen => write!(f, "unexpected )"),
        }
    }
}

impl core::error::Error for ExprParseError {}

/// A streaming parser that converts a token stream into `Expr` values.
///
/// Feed tokens with [`feed`](Parser::feed), signal end-of-stream with
/// [`close`](Parser::close), then drain expressions with [`next`](Parser::next).
pub struct ExprParser {
    tokens: VecDeque<Token>,
    /// Stack of open groups: (lparen span, accumulated exprs).
    stack: Vec<(Span, Vec<Expr>)>,
    eof: bool,
}

pub enum ExprParseItem {
    Expr(Expr),
    Newline(Span),
}

impl ExprParser {
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::new(),
            stack: Vec::new(),
            eof: false,
        }
    }

    pub fn feed(&mut self, token: Token) {
        self.tokens.push_back(token);
    }

    pub fn close(&mut self) {
        self.eof = true;
    }

    pub fn next_expr(&mut self) -> Result<Option<ExprParseItem>, Spanned<ExprParseError>> {
        loop {
            let Some(token) = self.tokens.pop_front() else {
                if self.eof {
                    if let Some((lparen_span, _)) = self.stack.pop() {
                        return Err(Spanned::new(
                            ExprParseError::UnclosedParenthesis,
                            lparen_span,
                        ));
                    }
                }
                return Ok(None);
            };

            let expr = match token {
                Token::Ident(s) => Expr::Ident(s),
                Token::Num(s) => Expr::Num(s),
                Token::Symbol(Spanned {
                    inner: Symbol::Plus,
                    span,
                }) => Expr::Op(Spanned::new(Op::Add, span)),
                Token::Symbol(Spanned {
                    inner: Symbol::Minus,
                    span,
                }) => Expr::Op(Spanned::new(Op::Sub, span)),
                Token::Symbol(Spanned {
                    inner: Symbol::Star,
                    span,
                }) => Expr::Op(Spanned::new(Op::Mul, span)),
                Token::Symbol(Spanned {
                    inner: Symbol::Slash,
                    span,
                }) => Expr::Op(Spanned::new(Op::Div, span)),
                Token::Symbol(Spanned {
                    inner: Symbol::Caret,
                    span,
                }) => Expr::Op(Spanned::new(Op::Pow, span)),
                Token::Symbol(Spanned {
                    inner: Symbol::LParen,
                    span,
                }) => {
                    self.stack.push((span, Vec::new()));
                    continue;
                }
                Token::Symbol(Spanned {
                    inner: Symbol::RParen,
                    span,
                }) => {
                    let (lparen_span, exprs) = self
                        .stack
                        .pop()
                        .ok_or_else(|| Spanned::new(ExprParseError::UnexpectedRParen, span))?;
                    Expr::Group {
                        lparen: Spanned::new(LParen, lparen_span),
                        exprs,
                        rparen: Spanned::new(RParen, span),
                    }
                }
                Token::Symbol(Spanned {
                    inner: Symbol::Newline,
                    span,
                }) => {
                    if let Some((lparen_span, _)) = self.stack.pop() {
                        return Err(Spanned::new(
                            ExprParseError::UnclosedParenthesis,
                            lparen_span,
                        ));
                    }
                    return Ok(Some(ExprParseItem::Newline(span)));
                }
            };

            if let Some((_, frame)) = self.stack.last_mut() {
                frame.push(expr);
            } else {
                return Ok(Some(ExprParseItem::Expr(expr)));
            }
        }
    }
}

impl Default for ExprParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;
    use crate::expr::{Expr, Op};
    use crate::lexer::Lexer;
    use crate::span::Spanned;

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

    fn parse_all(input: &str) -> Vec<ExprParseItem> {
        let mut parser = ExprParser::new();
        for tok in lex(input) {
            parser.feed(tok);
        }
        parser.close();
        let mut items = Vec::new();
        while let Some(item) = parser.next_expr().unwrap() {
            items.push(item);
        }
        items
    }

    #[test]
    fn test_empty() {
        assert_eq!(parse_all("").len(), 0);
    }

    #[test]
    fn test_ident() {
        let items = parse_all("hello");
        assert_eq!(items.len(), 1);
        assert!(matches!(items[0], ExprParseItem::Expr(Expr::Ident(..))));
    }

    #[test]
    fn test_int_and_float() {
        use crate::num::Num;
        let items = parse_all("42 3.14");
        assert!(matches!(
            items[0],
            ExprParseItem::Expr(Expr::Num(Spanned {
                inner: Num::Int(_),
                ..
            }))
        ));
        assert!(matches!(
            items[1],
            ExprParseItem::Expr(Expr::Num(Spanned {
                inner: Num::Float(_),
                ..
            }))
        ));
    }

    #[test]
    fn test_ops() {
        let items = parse_all("+ - * / ^");
        assert!(matches!(
            items[0],
            ExprParseItem::Expr(Expr::Op(Spanned { inner: Op::Add, .. }))
        ));
        assert!(matches!(
            items[1],
            ExprParseItem::Expr(Expr::Op(Spanned { inner: Op::Sub, .. }))
        ));
        assert!(matches!(
            items[2],
            ExprParseItem::Expr(Expr::Op(Spanned { inner: Op::Mul, .. }))
        ));
        assert!(matches!(
            items[3],
            ExprParseItem::Expr(Expr::Op(Spanned { inner: Op::Div, .. }))
        ));
        assert!(matches!(
            items[4],
            ExprParseItem::Expr(Expr::Op(Spanned { inner: Op::Pow, .. }))
        ));
    }

    #[test]
    fn test_newline_emitted() {
        let items = parse_all("a\nb");
        assert!(matches!(items[0], ExprParseItem::Expr(Expr::Ident(..))));
        assert!(matches!(items[1], ExprParseItem::Newline(..)));
        assert!(matches!(items[2], ExprParseItem::Expr(Expr::Ident(..))));
    }

    #[test]
    fn test_group() {
        let items = parse_all("(x 1)");
        assert_eq!(items.len(), 1);
        let ExprParseItem::Expr(Expr::Group { exprs: inner, .. }) = &items[0] else {
            panic!("expected group");
        };
        assert_eq!(inner.len(), 2);
        assert!(matches!(inner[0], Expr::Ident(..)));
        assert!(matches!(inner[1], Expr::Num(..)));
    }

    #[test]
    fn test_nested_group() {
        let items = parse_all("(x (1 2))");
        let ExprParseItem::Expr(Expr::Group { exprs: inner, .. }) = &items[0] else {
            panic!("expected group");
        };
        assert!(matches!(inner[1], Expr::Group { .. }));
    }

    #[test]
    fn test_streaming_feed_one_by_one() {
        let tokens = lex("a b");
        let mut parser = ExprParser::new();
        parser.feed(tokens[0].clone());
        // first item available immediately
        let item = parser.next_expr().unwrap();
        assert!(matches!(item, Some(ExprParseItem::Expr(Expr::Ident(..)))));
        parser.feed(tokens[1].clone());
        parser.close();
        let item = parser.next_expr().unwrap();
        assert!(matches!(item, Some(ExprParseItem::Expr(Expr::Ident(..)))));
        assert!(parser.next_expr().unwrap().is_none());
    }

    #[test]
    fn test_unclosed_paren_at_eof() {
        let mut parser = ExprParser::new();
        for tok in lex("(x") {
            parser.feed(tok);
        }
        parser.close();
        let err = parser.next_expr().err().unwrap();
        assert!(matches!(err.inner, ExprParseError::UnclosedParenthesis));
        assert_eq!(err.span, Span { start: 0, end: 1 });
    }

    #[test]
    fn test_unclosed_paren_at_newline() {
        let mut parser = ExprParser::new();
        for tok in lex("(x\n") {
            parser.feed(tok);
        }
        parser.close();
        let err = parser.next_expr().err().unwrap();
        assert!(matches!(err.inner, ExprParseError::UnclosedParenthesis));
        assert_eq!(err.span, Span { start: 0, end: 1 });
    }

    #[test]
    fn test_unexpected_rparen() {
        let mut parser = ExprParser::new();
        for tok in lex(")") {
            parser.feed(tok);
        }
        parser.close();
        let err = parser.next_expr().err().unwrap();
        assert!(matches!(err.inner, ExprParseError::UnexpectedRParen));
        assert_eq!(err.span, Span { start: 0, end: 1 });
    }
}
