use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::num::Num;
use crate::span::{Span, Spanned};

use super::def::{Expr, Op};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcError {
    UnknownConstant(String),
    BareOperator,
    EmptyGroup,
    MissingOperand,
    ExpectedOperator,
    UnsupportedUnaryOp,
    ExponentOutOfRange,
}

impl core::fmt::Display for CalcError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CalcError::UnknownConstant(name) => write!(f, "unknown constant: {name}"),
            CalcError::BareOperator => write!(f, "cannot evaluate a bare operator"),
            CalcError::EmptyGroup => write!(f, "empty group"),
            CalcError::MissingOperand => write!(f, "expected expression after operator"),
            CalcError::ExpectedOperator => write!(f, "expected operator"),
            CalcError::UnsupportedUnaryOp => write!(f, "unsupported unary operator"),
            CalcError::ExponentOutOfRange => write!(f, "exponent out of range"),
        }
    }
}

impl core::error::Error for CalcError {}

impl Expr {
    pub fn calc(&self) -> Result<Num, Spanned<CalcError>> {
        match self {
            Expr::Num(s) => Ok(s.inner),
            Expr::Ident(s) => match s.inner.as_str() {
                "pi" | "PI" => Ok(Num::Float(core::f64::consts::PI)),
                name => Err(Spanned::new(
                    CalcError::UnknownConstant(name.to_string()),
                    s.span,
                )),
            },
            Expr::Op(s) => Err(Spanned::new(CalcError::BareOperator, s.span)),
            Expr::Group {
                lparen,
                exprs,
                rparen,
            } => {
                if let Some(val) = eval_group(exprs)? {
                    Ok(val)
                } else {
                    Err(Spanned::new(
                        CalcError::EmptyGroup,
                        Span {
                            start: lparen.span.start,
                            end: rparen.span.end,
                        },
                    ))
                }
            }
        }
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

fn precedence(op: &Op) -> u8 {
    match op {
        Op::Add | Op::Sub => 1,
        Op::Mul | Op::Div => 2,
        Op::Pow => 3,
    }
}

/// Evaluate an infix expression using the shunting-yard algorithm.
/// The slice must be `[expr, op, expr, op, expr, ...]`, optionally
/// preceded by a unary `+` or `-` applied to the first expr.
fn eval_group(exprs: &[Expr]) -> Result<Option<Num>, Spanned<CalcError>> {
    let mut iter = exprs.iter();
    let mut values: Vec<Num> = Vec::new();
    let mut ops: Vec<&Spanned<Op>> = Vec::new();

    let first = match iter.next() {
        Some(Expr::Op(unary)) => {
            let operand_expr = iter
                .next()
                .ok_or_else(|| Spanned::new(CalcError::MissingOperand, unary.span))?;
            apply_unary(unary, operand_expr.calc()?)?
        }
        Some(expr) => expr.calc()?,
        None => return Ok(None),
    };
    values.push(first);

    while let Some(op_expr) = iter.next() {
        let Expr::Op(op) = op_expr else {
            return Err(Spanned::new(
                CalcError::ExpectedOperator,
                expr_span(op_expr),
            ));
        };

        // Pop operators from the stack while they have greater precedence, or equal
        // precedence for left-associative operators (all except Pow).
        while let Some(&top) = ops.last() {
            let pop = if matches!(op.inner, Op::Pow) {
                precedence(&top.inner) > precedence(&op.inner)
            } else {
                precedence(&top.inner) >= precedence(&op.inner)
            };
            if pop {
                ops.pop();
                let b = values.pop().unwrap();
                let a = values.pop().unwrap();
                values.push(apply_op(top, a, b)?);
            } else {
                break;
            }
        }
        ops.push(op);
        let rhs_expr = iter
            .next()
            .ok_or_else(|| Spanned::new(CalcError::MissingOperand, op.span))?;
        let rhs = match rhs_expr {
            Expr::Op(unary) => {
                let operand = iter
                    .next()
                    .ok_or_else(|| Spanned::new(CalcError::MissingOperand, unary.span))?;
                apply_unary(unary, operand.calc()?)?
            }
            other => other.calc()?,
        };
        values.push(rhs);
    }

    while let Some(op) = ops.pop() {
        let b = values.pop().unwrap();
        let a = values.pop().unwrap();
        values.push(apply_op(op, a, b)?);
    }

    Ok(values.pop())
}

fn apply_unary(op: &Spanned<Op>, a: Num) -> Result<Num, Spanned<CalcError>> {
    match op.inner {
        Op::Sub => Ok(match a {
            Num::Int(i) => Num::Int(-i),
            Num::Float(f) => Num::Float(-f),
        }),
        _ => Err(Spanned::new(CalcError::UnsupportedUnaryOp, op.span)),
    }
}

fn apply_op(op: &Spanned<Op>, a: Num, b: Num) -> Result<Num, Spanned<CalcError>> {
    match (a, b) {
        (Num::Int(a), Num::Int(b)) => Ok(match op.inner {
            Op::Add => Num::Int(a + b),
            Op::Sub => Num::Int(a - b),
            Op::Mul => Num::Int(a * b),
            Op::Div => Num::Int(a / b),
            Op::Pow => {
                if b < 0 {
                    apply_op(op, Num::Float(1.0 / (a as f64)), Num::Int(-b))?
                } else {
                    let exp = u32::try_from(b)
                        .map_err(|_| Spanned::new(CalcError::ExponentOutOfRange, op.span))?;
                    Num::Int(a.pow(exp))
                }
            }
        }),
        (a, b) => {
            let a = match a {
                Num::Int(i) => i as f64,
                Num::Float(f) => f,
            };
            let b = match b {
                Num::Int(i) => i as f64,
                Num::Float(f) => f,
            };
            Ok(Num::Float(match op.inner {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Mul => a * b,
                Op::Div => a / b,
                #[cfg(feature = "libm")]
                Op::Pow => libm::pow(a, b),
                #[cfg(not(feature = "libm"))]
                Op::Pow => a.powf(b),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;
    use crate::expr::def::{LParen, RParen};
    use crate::span::{Span, Spanned};

    fn sp<T>(inner: T) -> Spanned<T> {
        Spanned::new(inner, Span { start: 0, end: 0 })
    }

    fn sp_at<T>(inner: T, start: usize, end: usize) -> Spanned<T> {
        Spanned::new(inner, Span { start, end })
    }

    fn group(exprs: Vec<Expr>) -> Expr {
        Expr::Group {
            lparen: sp(LParen),
            exprs,
            rparen: sp(RParen),
        }
    }

    #[test]
    fn test_num_int() {
        assert!(matches!(
            Expr::Num(sp(Num::Int(42))).calc().unwrap(),
            Num::Int(42)
        ));
    }

    #[test]
    fn test_single_group() {
        assert!(matches!(
            group(vec![Expr::Num(sp(Num::Int(42)))]).calc().unwrap(),
            Num::Int(42)
        ));
    }

    #[test]
    fn test_neg_num() {
        assert!(matches!(
            group(vec![Expr::Op(sp(Op::Sub)), Expr::Num(sp(Num::Int(42)))])
                .calc()
                .unwrap(),
            Num::Int(-42)
        ));
    }

    #[test]
    fn test_num_float() {
        let Num::Float(f) = Expr::Num(sp(Num::Float(1.5))).calc().unwrap() else {
            panic!()
        };
        assert_eq!(f, 1.5);
    }

    #[test]
    fn test_ident_pi_lowercase() {
        let Num::Float(f) = Expr::Ident(sp("pi".to_string())).calc().unwrap() else {
            panic!()
        };
        assert_eq!(f, core::f64::consts::PI);
    }

    #[test]
    fn test_ident_pi_uppercase() {
        let Num::Float(f) = Expr::Ident(sp("PI".to_string())).calc().unwrap() else {
            panic!()
        };
        assert_eq!(f, core::f64::consts::PI);
    }

    #[test]
    fn test_add_ints() {
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(3))),
                Expr::Op(sp(Op::Add)),
                Expr::Num(sp(Num::Int(4))),
            ])
            .calc()
            .unwrap(),
            Num::Int(7)
        ));
    }

    #[test]
    fn test_sub_ints() {
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(10))),
                Expr::Op(sp(Op::Sub)),
                Expr::Num(sp(Num::Int(3))),
            ])
            .calc()
            .unwrap(),
            Num::Int(7)
        ));
    }

    #[test]
    fn test_mul_ints() {
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(3))),
                Expr::Op(sp(Op::Mul)),
                Expr::Num(sp(Num::Int(4))),
            ])
            .calc()
            .unwrap(),
            Num::Int(12)
        ));
    }

    #[test]
    fn test_div_ints() {
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(12))),
                Expr::Op(sp(Op::Div)),
                Expr::Num(sp(Num::Int(4))),
            ])
            .calc()
            .unwrap(),
            Num::Int(3)
        ));
    }

    #[test]
    fn test_pow_ints() {
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(2))),
                Expr::Op(sp(Op::Pow)),
                Expr::Num(sp(Num::Int(10))),
            ])
            .calc()
            .unwrap(),
            Num::Int(1024)
        ));
    }

    #[test]
    fn test_add_floats() {
        let Num::Float(f) = group(vec![
            Expr::Num(sp(Num::Float(1.5))),
            Expr::Op(sp(Op::Add)),
            Expr::Num(sp(Num::Float(2.5))),
        ])
        .calc()
        .unwrap() else {
            panic!()
        };
        assert_eq!(f, 4.0);
    }

    #[test]
    fn test_mixed_int_float() {
        let Num::Float(f) = group(vec![
            Expr::Num(sp(Num::Int(1))),
            Expr::Op(sp(Op::Add)),
            Expr::Num(sp(Num::Float(0.5))),
        ])
        .calc()
        .unwrap() else {
            panic!()
        };
        assert_eq!(f, 1.5);
    }

    #[test]
    fn test_nested_group() {
        // (4 + (2 * 3)) = 10
        let inner = group(vec![
            Expr::Num(sp(Num::Int(2))),
            Expr::Op(sp(Op::Mul)),
            Expr::Num(sp(Num::Int(3))),
        ]);
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(4))),
                Expr::Op(sp(Op::Add)),
                inner
            ])
            .calc()
            .unwrap(),
            Num::Int(10)
        ));
    }

    #[test]
    fn test_chained_ops() {
        // (1 + 2 + 3) = 6, left-to-right
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(1))),
                Expr::Op(sp(Op::Add)),
                Expr::Num(sp(Num::Int(2))),
                Expr::Op(sp(Op::Add)),
                Expr::Num(sp(Num::Int(3))),
            ])
            .calc()
            .unwrap(),
            Num::Int(6)
        ));
    }

    #[test]
    fn test_order_of_ops() {
        // (1 + 2 * 3) = 7
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(1))),
                Expr::Op(sp(Op::Add)),
                Expr::Num(sp(Num::Int(2))),
                Expr::Op(sp(Op::Mul)),
                Expr::Num(sp(Num::Int(3))),
            ])
            .calc()
            .unwrap(),
            Num::Int(7)
        ));
    }

    #[test]
    fn test_binary_unary_precedence() {
        // (2 * - 3) = -6
        assert!(matches!(
            group(vec![
                Expr::Num(sp(Num::Int(2))),
                Expr::Op(sp(Op::Mul)),
                Expr::Op(sp(Op::Sub)),
                Expr::Num(sp(Num::Int(3))),
            ])
            .calc()
            .unwrap(),
            Num::Int(-6)
        ));
    }

    #[test]
    fn test_single_element_group() {
        assert!(matches!(
            group(vec![Expr::Num(sp(Num::Int(42)))]).calc().unwrap(),
            Num::Int(42)
        ));
    }

    #[test]
    fn test_pi_in_expression() {
        // (2.0 * pi) ≈ 2π
        let Num::Float(f) = group(vec![
            Expr::Num(sp(Num::Float(2.0))),
            Expr::Op(sp(Op::Mul)),
            Expr::Ident(sp("pi".to_string())),
        ])
        .calc()
        .unwrap() else {
            panic!()
        };
        assert!((f - 2.0 * core::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_negative_exponent() {
        let Num::Float(f) = group(vec![
            Expr::Num(sp(Num::Int(2))),
            Expr::Op(sp_at(Op::Pow, 7, 8)),
            Expr::Num(sp(Num::Int(-1))),
        ])
        .calc()
        .unwrap() else {
            panic!()
        };
        assert_eq!(f, 0.5);
    }

    #[test]
    fn test_unknown_constant() {
        let err = Expr::Ident(sp_at("e".to_string(), 3, 4))
            .calc()
            .unwrap_err();
        assert!(matches!(err.inner, CalcError::UnknownConstant(ref s) if s == "e"));
        assert_eq!(err.span, Span { start: 3, end: 4 });
    }

    #[test]
    fn test_bare_operator() {
        let err = Expr::Op(sp_at(Op::Add, 5, 6)).calc().unwrap_err();
        assert!(matches!(err.inner, CalcError::BareOperator));
        assert_eq!(err.span, Span { start: 5, end: 6 });
    }

    #[test]
    fn test_unsupported_unary_op() {
        let err = group(vec![
            Expr::Op(sp_at(Op::Mul, 1, 2)),
            Expr::Num(sp(Num::Int(1))),
        ])
        .calc()
        .unwrap_err();
        assert!(matches!(err.inner, CalcError::UnsupportedUnaryOp));
        assert_eq!(err.span, Span { start: 1, end: 2 });
    }
}
