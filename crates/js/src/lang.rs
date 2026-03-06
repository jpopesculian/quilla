use quilla_lang::{Instruction, ParseError, Spanned};
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParseItem {
    #[serde(flatten)]
    result: ParseResult,
    span: Span,
}

impl From<Instruction> for Operation {
    fn from(instruction: Instruction) -> Self {
        match instruction {
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum ParseResult {
    Operation(Operation),
    Error(String),
}

impl From<Spanned<Result<Instruction, ParseError>>> for ParseItem {
    fn from(spanned: Spanned<Result<Instruction, quilla_lang::parse::ParseError>>) -> Self {
        ParseItem {
            result: match spanned.inner {
                Ok(instr) => ParseResult::Operation(Operation::from(instr)),
                Err(e) => ParseResult::Error(e.to_string()),
            },
            span: Span {
                start: spanned.span.start,
                end: spanned.span.end,
            },
        }
    }
}

#[wasm_bindgen(unchecked_return_type = "ParsedItem[]")]
pub fn parse(input: &str) -> Result<Vec<JsValue>, JsError> {
    quilla_lang::parse(input)
        .into_iter()
        .map(|spanned| {
            Ok(ParseItem::from(spanned)
                .serialize(&serde_wasm_bindgen::Serializer::json_compatible())?)
        })
        .collect()
}
