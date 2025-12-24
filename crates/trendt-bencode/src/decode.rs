use crate::error::{Error, Result};
use crate::value::Value;

/// A decoder for bencode data
pub struct Decoder<'a> {
    /// The input bytes
    input: &'a [u8],
    /// Current position in input
    position: usize,
}

impl<'a> Decoder<'a> {
    /// Create a new decoder from input bytes
    pub fn new(input: &'a [u8]) -> Self {
        Decoder { input, position: 0 }
    }

    /// Peek at the current byte without advancing
    fn peek(&self) -> Result<u8> {
        self.input
            .get(self.position)
            .copied()
            .ok_or(Error::UnexpectedEof)
    }

    /// Consume and return the current byte
    fn next(&mut self) -> Result<u8> {
        let byte = self.peek()?;
        self.position += 1;
        Ok(byte)
    }

    /// Expect a specific byte, error if mismatch
    fn expect(&mut self, expected: u8) -> Result<()> {
        let byte = self.next()?;
        if byte != expected {
            return Err(Error::InvalidCharacter(byte));
        }
        Ok(())
    }

    /// Decode an integer: i<number>e
    fn decode_integer(&mut self) -> Result<i64> {
        // Expect opening 'i'
        self.expect(b'i')?;

        // Collect digits (and optional leading '-')
        let start = self.position;
        loop {
            let byte = self.peek()?;
            if byte == b'e' {
                break;
            }
            self.next()?;
        }
        let end = self.position;

        // Parse the number
        let bytes = &self.input[start..end];
        let string = std::str::from_utf8(bytes).map_err(|_| Error::InvalidInteger)?;
        let number: i64 = string.parse().map_err(|_| Error::InvalidInteger)?;

        // Validate: no leading zeros (except "0" itself), no "-0"
        if bytes.len() > 1 && bytes[0] == b'0' {
            return Err(Error::InvalidInteger);
        }
        if bytes == b"-0" {
            return Err(Error::InvalidInteger);
        }

        // Expect closing 'e'
        self.expect(b'e')?;

        Ok(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_positive_integer() {
        let mut decoder = Decoder::new(b"i42e");
        assert_eq!(decoder.decode_integer().unwrap(), 42);
    }

    #[test]
    fn decode_negative_integer() {
        let mut decoder = Decoder::new(b"i-3e");
        assert_eq!(decoder.decode_integer().unwrap(), -3);
    }

    #[test]
    fn decode_zero() {
        let mut decoder = Decoder::new(b"i0e");
        assert_eq!(decoder.decode_integer().unwrap(), 0);
    }

    #[test]
    fn reject_leading_zero() {
        let mut decoder = Decoder::new(b"i03e");
        assert!(decoder.decode_integer().is_err());
    }

    #[test]
    fn reject_negative_zero() {
        let mut decoder = Decoder::new(b"i-0e");
        assert!(decoder.decode_integer().is_err());
    }
}
