use bitvec::{slice::BitSlice, vec::BitVec};
use std::fmt;

pub fn bits_to_string(bits: &BitSlice) -> String {
    bits.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

pub fn string_to_bits(s: &str) -> Result<BitVec, InvalidBitString> {
    s.chars()
        .map(|c| match c {
            '1' => Ok(true),
            '0' => Ok(false),
            c => Err(InvalidBitString(c)),
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidBitString(char);

impl fmt::Display for InvalidBitString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid bit string character: {}", self.0)
    }
}

impl std::error::Error for InvalidBitString {}
