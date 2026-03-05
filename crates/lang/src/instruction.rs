use alloc::string::{String, ToString};

use crate::func::Func;
use crate::num::Num;
use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum Instruction {
    H { target: usize },
    I { target: usize },
    X { target: usize },
    Y { target: usize },
    Z { target: usize },
    S { target: usize },
    Sdg { target: usize },
    T { target: usize },
    Tdg { target: usize },
    CX { control: usize, target: usize },
    CY { control: usize, target: usize },
    CZ { control: usize, target: usize },
    Swap { first: usize, second: usize },
    RX { theta: f64, target: usize },
    RY { theta: f64, target: usize },
    RZ { theta: f64, target: usize },
    Meas { qbit: usize, cbit: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionError {
    UnknownGate(String),
    WrongArgCount { expected: usize, got: usize },
    InvalidIndex,
}

impl core::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InstructionError::UnknownGate(name) => write!(f, "unknown gate: {name}"),
            InstructionError::WrongArgCount { expected, got } => {
                write!(f, "expected {expected} argument(s), got {got}")
            }
            InstructionError::InvalidIndex => write!(f, "argument is not a valid index"),
        }
    }
}

impl core::error::Error for InstructionError {}

impl TryFrom<Func> for Spanned<Instruction> {
    type Error = Spanned<InstructionError>;

    fn try_from(func: Func) -> Result<Self, Self::Error> {
        let ident_span = func.ident.span;
        let func_span = func
            .args
            .last()
            .map(|a| Span {
                start: ident_span.start,
                end: a.span.end,
            })
            .unwrap_or(ident_span);
        let args = func.args.as_slice();

        let instr = match func.ident.inner.as_str() {
            "h" => {
                check_argc(args, 1, func_span)?;
                Instruction::H {
                    target: as_index(&args[0])?,
                }
            }
            "i" => {
                check_argc(args, 1, func_span)?;
                Instruction::I {
                    target: as_index(&args[0])?,
                }
            }
            "x" => {
                check_argc(args, 1, func_span)?;
                Instruction::X {
                    target: as_index(&args[0])?,
                }
            }
            "y" => {
                check_argc(args, 1, func_span)?;
                Instruction::Y {
                    target: as_index(&args[0])?,
                }
            }
            "z" => {
                check_argc(args, 1, func_span)?;
                Instruction::Z {
                    target: as_index(&args[0])?,
                }
            }
            "s" => {
                check_argc(args, 1, func_span)?;
                Instruction::S {
                    target: as_index(&args[0])?,
                }
            }
            "sdg" => {
                check_argc(args, 1, func_span)?;
                Instruction::Sdg {
                    target: as_index(&args[0])?,
                }
            }
            "t" => {
                check_argc(args, 1, func_span)?;
                Instruction::T {
                    target: as_index(&args[0])?,
                }
            }
            "tdg" => {
                check_argc(args, 1, func_span)?;
                Instruction::Tdg {
                    target: as_index(&args[0])?,
                }
            }
            "cx" => {
                check_argc(args, 2, func_span)?;
                Instruction::CX {
                    control: as_index(&args[0])?,
                    target: as_index(&args[1])?,
                }
            }
            "cy" => {
                check_argc(args, 2, func_span)?;
                Instruction::CY {
                    control: as_index(&args[0])?,
                    target: as_index(&args[1])?,
                }
            }
            "cz" => {
                check_argc(args, 2, func_span)?;
                Instruction::CZ {
                    control: as_index(&args[0])?,
                    target: as_index(&args[1])?,
                }
            }
            "swap" => {
                check_argc(args, 2, func_span)?;
                Instruction::Swap {
                    first: as_index(&args[0])?,
                    second: as_index(&args[1])?,
                }
            }
            "rx" => {
                check_argc(args, 2, func_span)?;
                Instruction::RX {
                    theta: as_angle(&args[0]),
                    target: as_index(&args[1])?,
                }
            }
            "ry" => {
                check_argc(args, 2, func_span)?;
                Instruction::RY {
                    theta: as_angle(&args[0]),
                    target: as_index(&args[1])?,
                }
            }
            "rz" => {
                check_argc(args, 2, func_span)?;
                Instruction::RZ {
                    theta: as_angle(&args[0]),
                    target: as_index(&args[1])?,
                }
            }
            "meas" => {
                check_argc(args, 2, func_span)?;
                Instruction::Meas {
                    qbit: as_index(&args[0])?,
                    cbit: as_index(&args[1])?,
                }
            }
            name => {
                return Err(Spanned::new(
                    InstructionError::UnknownGate(name.to_string()),
                    ident_span,
                ));
            }
        };

        Ok(Spanned::new(instr, func_span))
    }
}

fn check_argc(
    args: &[Spanned<Num>],
    expected: usize,
    span: Span,
) -> Result<(), Spanned<InstructionError>> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(Spanned::new(
            InstructionError::WrongArgCount {
                expected,
                got: args.len(),
            },
            span,
        ))
    }
}

fn as_index(arg: &Spanned<Num>) -> Result<usize, Spanned<InstructionError>> {
    match arg.inner {
        Num::Int(n) if n >= 0 => Ok(n as usize),
        _ => Err(Spanned::new(InstructionError::InvalidIndex, arg.span)),
    }
}

fn as_angle(arg: &Spanned<Num>) -> f64 {
    match arg.inner {
        Num::Int(n) => n as f64,
        Num::Float(f) => f,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    fn parse_one(input: &str) -> Spanned<Instruction> {
        let mut instrs = parse(input).unwrap();
        assert_eq!(instrs.len(), 1);
        instrs.remove(0)
    }

    fn parse_err(input: &str) -> Spanned<InstructionError> {
        let err = parse(input).unwrap_err();
        let crate::parse::ParseError::Instruction(e) = err.inner else {
            panic!("expected instruction error, got {:?}", err.inner);
        };
        Spanned::new(e, err.span)
    }

    #[test]
    fn test_single_qubit() {
        assert!(matches!(
            parse_one("h 0\n").inner,
            Instruction::H { target: 0 }
        ));
        assert!(matches!(
            parse_one("sdg 2\n").inner,
            Instruction::Sdg { target: 2 }
        ));
    }

    #[test]
    fn test_two_qubit() {
        assert!(matches!(
            parse_one("cx 0 1\n").inner,
            Instruction::CX {
                control: 0,
                target: 1
            }
        ));
        assert!(matches!(
            parse_one("swap 1 3\n").inner,
            Instruction::Swap {
                first: 1,
                second: 3
            }
        ));
    }

    #[test]
    fn test_rotation() {
        let instr = parse_one("rx 1 0\n");
        let Instruction::RX { theta, target } = instr.inner else {
            panic!()
        };
        assert_eq!(theta, 1.0);
        assert_eq!(target, 0);
    }

    #[test]
    fn test_meas() {
        assert!(matches!(
            parse_one("meas 0 1\n").inner,
            Instruction::Meas { qbit: 0, cbit: 1 }
        ));
    }

    #[test]
    fn test_unknown_gate() {
        let err = parse_err("foo 0\n");
        assert!(matches!(err.inner, InstructionError::UnknownGate(_)));
    }

    #[test]
    fn test_wrong_arg_count() {
        let err = parse_err("h 0 1\n");
        assert!(matches!(
            err.inner,
            InstructionError::WrongArgCount {
                expected: 1,
                got: 2
            }
        ));
    }

    #[test]
    fn test_invalid_qubit_negative() {
        let err = parse_err("h -1\n");
        assert!(matches!(err.span, Span { start: 2, end: 4 }));
        assert!(matches!(err.inner, InstructionError::InvalidIndex));
    }

    #[test]
    fn test_invalid_qubit_float() {
        let err = parse_err("cx 0 1.5\n");
        assert!(matches!(err.span, Span { start: 5, end: 8 }));
        assert!(matches!(err.inner, InstructionError::InvalidIndex));
    }
}
