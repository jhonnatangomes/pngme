use std::{
    fmt::Display,
    str::{from_utf8, FromStr},
};

fn is_valid_ascii_string(value: [u8; 4]) -> bool {
    for byte in value {
        if !byte.is_ascii_alphabetic() {
            return false;
        }
    }
    true
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if is_valid_ascii_string(value) {
            Ok(ChunkType { bytes: value })
        } else {
            Err("Chunk type must contain only ASCII letters".into())
        }
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes().try_into() {
            Ok(bytes) => {
                if is_valid_ascii_string(bytes) {
                    Ok(ChunkType { bytes })
                } else {
                    Err("Chunk type must contain only ASCII letters".into())
                }
            }
            Err(_) => Err("Chunk type must be 4 characters long".into()),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.bytes).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.bytes;
    }

    fn is_valid(&self) -> bool {
        return is_valid_ascii_string(self.bytes) && self.is_reserved_bit_valid();
    }

    fn is_critical(&self) -> bool {
        let first_byte = self.bytes[0];
        return (first_byte >> 5 & 1) == 0;
    }

    fn is_public(&self) -> bool {
        let second_byte = self.bytes[1];
        return (second_byte >> 5 & 1) == 0;
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let third_byte = self.bytes[2];
        return (third_byte >> 5 & 1) == 0;
    }

    fn is_safe_to_copy(&self) -> bool {
        let fourth_byte = self.bytes[3];
        return (fourth_byte >> 5 & 1) == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
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

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
