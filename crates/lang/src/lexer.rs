use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{take_while, take_while1},
    character::complete::{char, digit1},
    combinator::{map, opt, recognize},
    sequence::pair,
};

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::num::Num;
use crate::span::{Span, Spanned};
use crate::token::{Symbol, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    InvalidUtf8,
    InvalidNumber { raw: String },
    UnexpectedChar { ch: char },
}

impl core::fmt::Display for LexerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LexerError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            LexerError::InvalidNumber { raw } => write!(f, "invalid number '{raw}'"),
            LexerError::UnexpectedChar { ch } => write!(f, "unexpected character '{ch}'"),
        }
    }
}

impl core::error::Error for LexerError {}

pub type Error = Spanned<LexerError>;

/// A streaming lexer that tokenizes UTF-8 input fed in chunks.
///
/// Feed data with [`feed`](Lexer::feed), then call [`next_token`](Lexer::next_token)
/// repeatedly to drain tokens. When the input stream ends, call [`close`](Lexer::close)
/// before draining the final tokens.
pub struct Lexer {
    buffer: Vec<u8>,
    offset: usize,
    eof: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            offset: 0,
            eof: false,
        }
    }

    /// Append more input bytes to the internal buffer.
    pub fn feed(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// Signal end-of-stream. After closing, `next_token` will flush any remaining token.
    pub fn close(&mut self) {
        self.eof = true;
    }

    fn buffer_str(&self) -> Result<&str, Spanned<LexerError>> {
        match core::str::from_utf8(&self.buffer) {
            Ok(s) => Ok(s),
            Err(e) if !self.eof => {
                core::str::from_utf8(&self.buffer[..e.valid_up_to()]).map_err(|_| {
                    Spanned::new(
                        LexerError::InvalidUtf8,
                        utf8_error_span(self.offset, self.buffer.len(), e),
                    )
                })
            }
            Err(e) => Err(Spanned::new(
                LexerError::InvalidUtf8,
                utf8_error_span(self.offset, self.buffer.len(), e),
            )),
        }
    }

    /// Return the next complete token, or `Ok(None)` if more input is needed.
    pub fn next_token(&mut self) -> Result<Option<Token>, Spanned<LexerError>> {
        // Discard whitespace and line comments, interleaved.
        loop {
            // Skip leading spaces and tabs (never tokenised).
            let ws = self
                .buffer
                .iter()
                .take_while(|&&b| b == b' ' || b == b'\t')
                .count();
            if ws > 0 {
                self.buffer.drain(..ws);
                self.offset += ws;
            }

            // Skip line comments: -- ... <newline>. The newline is NOT consumed
            // so it can still be emitted as a token.
            if self.buffer.starts_with(b"--") {
                match self.buffer.iter().position(|&b| b == b'\n') {
                    Some(nl) => {
                        self.buffer.drain(..nl);
                        self.offset += nl;
                    }
                    // Comment not yet terminated; wait for more data.
                    None if !self.eof => return Ok(None),
                    // EOF inside a comment; consume and stop.
                    None => {
                        self.offset += self.buffer.len();
                        self.buffer.clear();
                        break;
                    }
                }
            } else {
                break;
            }
        }

        if self.buffer.is_empty() {
            return Ok(None);
        }

        let input = self.buffer_str()?;

        if input.is_empty() {
            return Ok(None);
        }

        match parse_token(input) {
            Ok((remaining, parsed)) => {
                // In streaming mode a token is only complete when we can see what
                // comes after it (remaining non-empty) or we know the stream ended.
                if remaining.is_empty() && !self.eof {
                    return Ok(None);
                }
                let consumed = input.len() - remaining.len();
                let span = Span {
                    start: self.offset,
                    end: self.offset + consumed,
                };
                let token = make_token(parsed, span)?;
                self.buffer.drain(..consumed);
                self.offset += consumed;
                Ok(Some(token))
            }
            Err(_) => {
                let ch = input.chars().next().unwrap(); // safe: buffer is non-empty
                let span = Span {
                    start: self.offset,
                    end: self.offset + ch.len_utf8(),
                };
                Err(Spanned::new(LexerError::UnexpectedChar { ch }, span))
            }
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

// ── internal parsers ────────────────────────────────────────────────────────

enum Parsed<'a> {
    Identifier(&'a str),
    Number(&'a str),
    Symbol(Symbol),
}

fn parse_token(input: &str) -> IResult<&str, Parsed<'_>> {
    alt((
        map(parse_identifier, Parsed::Identifier),
        map(parse_number, Parsed::Number),
        map(parse_symbol, Parsed::Symbol),
    ))
    .parse(input)
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(|c: char| c.is_ascii_alphabetic()),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
    ))
    .parse(input)
}

/// Matches `digits` or `digits.digits` (float).
fn parse_number(input: &str) -> IResult<&str, &str> {
    recognize(pair(digit1, opt(pair(char('.'), digit1)))).parse(input)
}

fn parse_symbol(input: &str) -> IResult<&str, Symbol> {
    alt((
        map(char('+'), |_| Symbol::Plus),
        map(char('-'), |_| Symbol::Minus),
        map(char('*'), |_| Symbol::Star),
        map(char('/'), |_| Symbol::Slash),
        map(char('^'), |_| Symbol::Caret),
        map(char('('), |_| Symbol::LParen),
        map(char(')'), |_| Symbol::RParen),
        map(char('\n'), |_| Symbol::Newline),
    ))
    .parse(input)
}

fn utf8_error_span(offset: usize, buffer_len: usize, error: core::str::Utf8Error) -> Span {
    let start = offset + error.valid_up_to();
    let mut end = match error.error_len() {
        Some(n) => start + n,
        None => offset + buffer_len,
    };
    if end <= start {
        end = start + 1;
    }
    Span { start, end }
}

fn make_token(parsed: Parsed<'_>, span: Span) -> Result<Token, Spanned<LexerError>> {
    match parsed {
        Parsed::Identifier(s) => Ok(Token::ident(s.to_string(), span)),
        Parsed::Number(s) => {
            if s.contains('.') {
                s.parse::<f64>()
                    .map(|f| Token::num(Num::Float(f), span))
                    .map_err(|_| Spanned::new(LexerError::InvalidNumber {
                        raw: s.to_string(),
                    }, span))
            } else {
                s.parse::<i64>()
                    .map(|i| Token::num(Num::Int(i), span))
                    .map_err(|_| Spanned::new(LexerError::InvalidNumber {
                        raw: s.to_string(),
                    }, span))
            }
        }
        Parsed::Symbol(sym) => Ok(Token::symbol(sym, span)),
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;
    use crate::num::Num;
    use crate::span::{Span, Spanned};
    use crate::token::{Symbol, Token};

    fn lex_all(input: &[u8]) -> Vec<Token> {
        let mut lexer = Lexer::new();
        lexer.feed(input);
        lexer.close();
        let mut tokens = Vec::new();
        while let Some(tok) = lexer.next_token().unwrap() {
            tokens.push(tok);
        }
        tokens
    }

    #[test]
    fn test_identifier() {
        let tokens = lex_all(b"hello world_2");
        assert_eq!(
            tokens,
            vec![
                Token::ident("hello".into(), Span { start: 0, end: 5 }),
                Token::ident("world_2".into(), Span { start: 6, end: 13 }),
            ]
        );
    }

    #[test]
    fn test_trailing_whitespace() {
        let tokens = lex_all(b"hello ");
        assert_eq!(
            tokens,
            vec![Token::ident("hello".into(), Span { start: 0, end: 5 })]
        );
    }

    #[test]
    fn test_leading_whitespace() {
        let tokens = lex_all(b" hello");
        assert_eq!(
            tokens,
            vec![Token::ident("hello".into(), Span { start: 1, end: 6 })]
        );
    }

    #[test]
    fn test_integer() {
        let tokens = lex_all(b"123 456");
        assert_eq!(
            tokens,
            vec![
                Token::num(Num::Int(123), Span { start: 0, end: 3 }),
                Token::num(Num::Int(456), Span { start: 4, end: 7 }),
            ]
        );
    }

    #[test]
    fn test_float() {
        let tokens = lex_all(b"1.23 0.5");
        assert_eq!(
            tokens,
            vec![
                Token::num(Num::Float(1.23), Span { start: 0, end: 4 }),
                Token::num(Num::Float(0.5), Span { start: 5, end: 8 }),
            ]
        );
    }

    #[test]
    fn test_symbols() {
        let tokens = lex_all(b"+-*/^()\n");
        assert_eq!(
            tokens,
            vec![
                Token::symbol(Symbol::Plus, Span { start: 0, end: 1 }),
                Token::symbol(Symbol::Minus, Span { start: 1, end: 2 }),
                Token::symbol(Symbol::Star, Span { start: 2, end: 3 }),
                Token::symbol(Symbol::Slash, Span { start: 3, end: 4 }),
                Token::symbol(Symbol::Caret, Span { start: 4, end: 5 }),
                Token::symbol(Symbol::LParen, Span { start: 5, end: 6 }),
                Token::symbol(Symbol::RParen, Span { start: 6, end: 7 }),
                Token::symbol(Symbol::Newline, Span { start: 7, end: 8 }),
            ]
        );
    }

    #[test]
    fn test_mixed() {
        let tokens = lex_all(b"sin(x) + 1.23");
        assert_eq!(
            tokens,
            vec![
                Token::ident("sin".into(), Span { start: 0, end: 3 }),
                Token::symbol(Symbol::LParen, Span { start: 3, end: 4 }),
                Token::ident("x".into(), Span { start: 4, end: 5 }),
                Token::symbol(Symbol::RParen, Span { start: 5, end: 6 }),
                Token::symbol(Symbol::Plus, Span { start: 7, end: 8 }),
                Token::num(Num::Float(1.23), Span { start: 9, end: 13 }),
            ]
        );
    }

    #[test]
    fn test_streaming_chunks() {
        let mut lexer = Lexer::new();
        // Feed partial identifier — no complete token yet.
        lexer.feed(b"hel");
        assert_eq!(lexer.next_token().unwrap(), None);
        // Feed the rest with a terminator.
        lexer.feed(b"lo ");
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::ident("hello".into(), Span { start: 0, end: 5 }))
        );
    }

    #[test]
    fn test_streaming_eof_flushes() {
        let mut lexer = Lexer::new();
        lexer.feed(b"42");
        assert_eq!(lexer.next_token().unwrap(), None); // incomplete
        lexer.close();
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::num(Num::Int(42), Span { start: 0, end: 2 }))
        );
        assert_eq!(lexer.next_token().unwrap(), None); // done
    }

    #[test]
    fn test_unexpected_char() {
        let mut lexer = Lexer::new();
        lexer.feed(b"@");
        lexer.close();
        assert!(matches!(
            lexer.next_token(),
            Err(Spanned {
                inner: LexerError::UnexpectedChar { ch: '@' },
                span: Span { start: 0, end: 1 },
            })
        ));
    }

    #[test]
    fn test_comment_skipped() {
        let tokens = lex_all(b"a -- this is a comment\nb");
        assert_eq!(
            tokens,
            vec![
                Token::ident("a".into(), Span { start: 0, end: 1 }),
                Token::symbol(Symbol::Newline, Span { start: 22, end: 23 }),
                Token::ident("b".into(), Span { start: 23, end: 24 }),
            ]
        );
    }

    #[test]
    fn test_comment_at_eof_no_newline() {
        let tokens = lex_all(b"a -- comment");
        assert_eq!(
            tokens,
            vec![Token::ident("a".into(), Span { start: 0, end: 1 })]
        );
    }

    #[test]
    fn test_comment_streaming_waits_for_newline() {
        let mut lexer = Lexer::new();
        lexer.feed(b"-- comment");
        assert_eq!(lexer.next_token().unwrap(), None);
        lexer.feed(b" continues\nx");
        lexer.close();
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::symbol(Symbol::Newline, Span { start: 20, end: 21 }))
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Some(Token::ident("x".into(), Span { start: 21, end: 22 }))
        );
    }

    #[test]
    fn test_multiple_comments() {
        let tokens = lex_all(b"-- first\n-- second\nx");
        assert_eq!(
            tokens,
            vec![
                Token::symbol(Symbol::Newline, Span { start: 8, end: 9 }),
                Token::symbol(Symbol::Newline, Span { start: 18, end: 19 }),
                Token::ident("x".into(), Span { start: 19, end: 20 }),
            ]
        );
    }
}
