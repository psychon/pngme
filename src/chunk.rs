use std::convert::{TryFrom, TryInto};

use anyhow::anyhow;

use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crate::crc::Crc;

#[derive(Debug, Eq, PartialEq)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self { chunk_type, data }
    }

    pub fn length(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_as_string(&self) -> Result<String> {
        let str = std::str::from_utf8(&self.data)?;
        Ok(str.to_string())
    }

    pub fn crc(&self) -> u32 {
        let crc = Crc::new();
        let crc = crc.update(self.chunk_type.bytes());
        let crc = crc.update(&self.data);
        crc.get()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(&self.length().to_be_bytes());
        result.extend(self.chunk_type.bytes());
        result.extend(&self.data);
        result.extend(&self.crc().to_be_bytes());
        result
    }

    pub fn parse_next(value: &[u8]) -> Result<(Self, &[u8])> {
        // Get the individual parts as byte slices
        if value.len() < 12 {
            return Err(anyhow!("Too short chunk data"));
        }
        let length = &value[0..4];
        let chunk_type = &value[4..8];
        let remaining = &value[8..];

        // Parse the values
        let length = u32::from_be_bytes(length.try_into()?);
        let chunk_type = ChunkType::new(chunk_type.try_into()?)?;

        // Check that the length is valid
        if length > u32::max_value() - 4 {
            return Err(anyhow!("Too large length"));
        }
        let length = usize::try_from(length)?;
        if length + 4 < remaining.len() {
            return Err(anyhow!("Too large length, larger than remaining data"));
        }

        // Get the data and the CRC
        let data = &remaining[..length];
        let crc = &remaining[length..length + 4];
        let remaining = &remaining[length + 4..];
        let crc = u32::from_be_bytes(crc.try_into()?);

        // Check that the CRC is valid
        let chunk = Chunk::new(chunk_type, data.to_vec());
        let chunk_crc = chunk.crc();
        if chunk_crc != crc {
            Err(anyhow!("Incorrect chunk CRC: {} != {}", crc, chunk_crc))
        } else {
            Ok((chunk, remaining))
        }
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        let (chunk, remaining) = Self::parse_next(value)?;
        if !remaining.is_empty() {
            return Err(anyhow!("Trailing data left after chunk"));
        }
        Ok(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!"
            .bytes()
            .collect();
        Chunk::new(chunk_type, data)
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
}
