use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::fmt::{Display, Formatter};
use anyhow::{anyhow, Context};

use super::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

fn valid_chunk_character(value: u8) -> bool {
    (value >= b'A' && value <= b'Z') ||
        (value >= b'a' && value <= b'z')
}

fn is_lowercase(value: u8) -> bool {
    value >= b'a' && value <= b'z'
}

impl ChunkType {
    pub fn new(value: [u8; 4]) -> Result<Self> {
        if !value.iter().all(|&c| valid_chunk_character(c)) {
            return Err(anyhow!("chunk types must be ascii characters, {:?} rejected", value));
        }
        Ok(Self(value))
    }

    pub fn bytes(&self) -> &[u8; 4] {
        &self.0
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        is_lowercase(self.0[0]) == false
    }

    pub fn is_public(&self) -> bool {
        is_lowercase(self.0[1]) == false
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        is_lowercase(self.0[2]) == false
    }

    pub fn is_safe_to_copy(&self) -> bool {
        is_lowercase(self.0[3]) == true
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d] = self.bytes();
        let (a, b, c, d) = (char::from(*a), char::from(*b), char::from(*c), char::from(*d));
        write!(f, "{}{}{}{}", a, b, c, d)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        Self::new(value)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes: std::result::Result<[u8; 4], _> = s.as_bytes().try_into();
        let bytes = bytes.context("Chunk names must have length 4")?;
        bytes.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = &[82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}
