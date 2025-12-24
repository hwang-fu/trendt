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

    /// Decode a byte string: <length>:<data>
    fn decode_byte_string(&mut self) -> Result<Vec<u8>> {
        // Read length digits until ':'
        let start = self.position;
        loop {
            let byte = self.peek()?;
            if byte == b':' {
                break;
            }
            if !byte.is_ascii_digit() {
                return Err(Error::InvalidCharacter(byte));
            }
            self.next()?;
        }
        let end = self.position;

        // Parse length
        let length_bytes = &self.input[start..end];
        let length_str =
            std::str::from_utf8(length_bytes).map_err(|_| Error::InvalidCharacter(0))?;
        let length: usize = length_str.parse().map_err(|_| Error::InvalidCharacter(0))?;

        // Skip ':'
        self.expect(b':')?;

        // Read exactly `length` bytes
        if self.position + length > self.input.len() {
            return Err(Error::UnexpectedEof);
        }
        let data = self.input[self.position..self.position + length].to_vec();
        self.position += length;

        Ok(data)
    }

    /// Decode any bencode value
    pub fn decode_value(&mut self) -> Result<Value> {
        match self.peek()? {
            b'i' => Ok(Value::Integer(self.decode_integer()?)),
            b'l' => self.decode_list(),
            b'd' => self.decode_dict(),
            b'0'..=b'9' => Ok(Value::ByteString(self.decode_byte_string()?)),
            byte => Err(Error::InvalidCharacter(byte)),
        }
    }

    /// Decode a list: l<items>e
    fn decode_list(&mut self) -> Result<Value> {
        // Expect opening 'l'
        self.expect(b'l')?;

        let mut items = Vec::new();

        // Read items until 'e'
        while self.peek()? != b'e' {
            items.push(self.decode_value()?);
        }

        // Expect closing 'e'
        self.expect(b'e')?;

        Ok(Value::List(items))
    }

    /// Decode a dictionary: d<pairs>e
    fn decode_dict(&mut self) -> Result<Value> {
        todo!()
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

    #[test]
    fn decode_byte_string_simple() {
        let mut decoder = Decoder::new(b"4:spam");
        assert_eq!(decoder.decode_byte_string().unwrap(), b"spam");
    }

    #[test]
    fn decode_byte_string_empty() {
        let mut decoder = Decoder::new(b"0:");
        assert_eq!(decoder.decode_byte_string().unwrap(), b"");
    }

    #[test]
    fn decode_byte_string_with_binary() {
        let mut decoder = Decoder::new(b"3:\x00\x01\x02");
        assert_eq!(decoder.decode_byte_string().unwrap(), vec![0, 1, 2]);
    }

    #[test]
    fn decode_empty_list() {
        let mut decoder = Decoder::new(b"le");
        assert_eq!(decoder.decode_value().unwrap(), Value::List(vec![]));
    }

    #[test]
    fn decode_list_of_integers() {
        let mut decoder = Decoder::new(b"li1ei2ei3ee");
        let expected = Value::List(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        assert_eq!(decoder.decode_value().unwrap(), expected);
    }

    #[test]
    fn decode_list_mixed() {
        let mut decoder = Decoder::new(b"l4:spami42ee");
        let expected = Value::List(vec![
            Value::ByteString(b"spam".to_vec()),
            Value::Integer(42),
        ]);
        assert_eq!(decoder.decode_value().unwrap(), expected);
    }

    #[test]
    fn decode_nested_list() {
        let mut decoder = Decoder::new(b"lli1eee");
        let expected = Value::List(vec![Value::List(vec![Value::Integer(1)])]);
        assert_eq!(decoder.decode_value().unwrap(), expected);
    }
}
