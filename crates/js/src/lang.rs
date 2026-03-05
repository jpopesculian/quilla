use quilla_lang::instruction::Instruction;
use quilla_lang::span::Spanned;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use wasm_bindgen::prelude::*;

use crate::operation::Operation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParsedOperation {
    operation: Operation,
    span: Span,
}

impl From<Spanned<Instruction>> for ParsedOperation {
    fn from(spanned: Spanned<Instruction>) -> Self {
        let operation = match spanned.inner {
            Instruction::H { target } => Operation::H { target },
            Instruction::I { target } => Operation::I { target },
            Instruction::X { target } => Operation::X { target },
            Instruction::Y { target } => Operation::Y { target },
            Instruction::Z { target } => Operation::Z { target },
            Instruction::S { target } => Operation::S { target },
            Instruction::Sdg { target } => Operation::Sdg { target },
            Instruction::T { target } => Operation::T { target },
            Instruction::Tdg { target } => Operation::Tdg { target },
            Instruction::CX { control, target } => Operation::CX { control, target },
            Instruction::CY { control, target } => Operation::CY { control, target },
            Instruction::CZ { control, target } => Operation::CZ { control, target },
            Instruction::Swap { first, second } => Operation::Swap { first, second },
            Instruction::RX { theta, target } => Operation::RX { theta, target },
            Instruction::RY { theta, target } => Operation::RY { theta, target },
            Instruction::RZ { theta, target } => Operation::RZ { theta, target },
            Instruction::Meas { qbit, cbit } => Operation::Meas { qbit, cbit },
        };
        let span = Span {
            start: spanned.span.start,
            end: spanned.span.end,
        };
        ParsedOperation { operation, span }
    }
}

#[wasm_bindgen(unchecked_return_type = "ParsedOperation[]")]
pub fn parse(input: &str) -> Result<Vec<JsValue>, JsValue> {
    let instrs = quilla_lang::parse(input).map_err(|e| JsValue::from_str(&e.to_string()))?;
    instrs
        .into_iter()
        .map(|spanned| {
            ParsedOperation::from(spanned)
                .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
                .map_err(|e| JsValue::from_str(&e.to_string()))
        })
        .collect()
}
