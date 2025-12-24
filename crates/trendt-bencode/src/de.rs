use serde::Deserialize;
use serde::de::{self, Visitor};

use crate::error::{Error, Result};

/// A deserializer for bencode data
pub struct Deserializer<'de> {
    input: &'de [u8],
    position: usize,
}

impl<'de> Deserializer<'de> {
    pub fn new(input: &'de [u8]) -> Self {
        Deserializer { input, position: 0 }
    }

    fn peek(&self) -> Result<u8> {
        self.input
            .get(self.position)
            .copied()
            .ok_or(Error::UnexpectedEof)
    }

    fn next(&mut self) -> Result<u8> {
        let byte = self.peek()?;
        self.position += 1;
        Ok(byte)
    }

    fn expect(&mut self, expected: u8) -> Result<()> {
        let byte = self.next()?;
        if byte != expected {
            return Err(Error::InvalidCharacter(byte));
        }
        Ok(())
    }

    fn parse_integer(&mut self) -> Result<i64> {
        self.expect(b'i')?;
        let start = self.position;
        while self.peek()? != b'e' {
            self.next()?;
        }
        let end = self.position;
        let bytes = &self.input[start..end];
        let s = std::str::from_utf8(bytes).map_err(|_| Error::InvalidInteger)?;
        let n: i64 = s.parse().map_err(|_| Error::InvalidInteger)?;
        self.expect(b'e')?;
        Ok(n)
    }

    fn parse_byte_string(&mut self) -> Result<&'de [u8]> {
        let start = self.position;
        while self.peek()? != b':' {
            self.next()?;
        }
        let end = self.position;
        let len_bytes = &self.input[start..end];
        let len_str = std::str::from_utf8(len_bytes).map_err(|_| Error::InvalidCharacter(0))?;
        let len: usize = len_str.parse().map_err(|_| Error::InvalidCharacter(0))?;
        self.expect(b':')?;
        if self.position + len > self.input.len() {
            return Err(Error::UnexpectedEof);
        }
        let data = &self.input[self.position..self.position + len];
        self.position += len;
        Ok(data)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.peek()? {
            b'i' => self.deserialize_i64(visitor),
            b'l' => self.deserialize_seq(visitor),
            b'd' => self.deserialize_map(visitor),
            b'0'..=b'9' => self.deserialize_bytes(visitor),
            byte => Err(Error::InvalidCharacter(byte)),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.parse_integer()?;
        visitor.visit_bool(n != 0)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(self.parse_integer()? as i8)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(self.parse_integer()? as i16)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(self.parse_integer()? as i32)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.parse_integer()?)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.parse_integer()? as u8)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(self.parse_integer()? as u16)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(self.parse_integer()? as u32)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(self.parse_integer()? as u64)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Message("bencode does not support floats".into()))
    }

    fn deserialize_f64<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Message("bencode does not support floats".into()))
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let bytes = self.parse_byte_string()?;
        let s = std::str::from_utf8(bytes).map_err(|_| Error::InvalidCharacter(0))?;
        let mut chars = s.chars();
        let c = chars.next().ok_or(Error::UnexpectedEof)?;
        if chars.next().is_some() {
            return Err(Error::Message("expected single char".into()));
        }
        visitor.visit_char(c)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let bytes = self.parse_byte_string()?;
        let s = std::str::from_utf8(bytes).map_err(|_| Error::InvalidCharacter(0))?;
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let bytes = self.parse_byte_string()?;
        visitor.visit_borrowed_bytes(bytes)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.expect(b'l')?;
        let value = visitor.visit_seq(SeqAccess::new(self))?;
        self.expect(b'e')?;
        Ok(value)
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.expect(b'd')?;
        let value = visitor.visit_map(MapAccess::new(self))?;
        self.expect(b'e')?;
        Ok(value)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value> {
        Err(Error::Message("bencode does not support enums".into()))
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }
}

struct SeqAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        SeqAccess { de }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Check if we've reached the end of the list
        if self.de.peek()? == b'e' {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct MapAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        MapAccess { de }
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        // Check if we've reached the end of the dict
        if self.de.peek()? == b'e' {
            return Ok(None);
        }
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

/// Deserialize a value from bencode bytes
pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(input);
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn deserialize_integer() {
        let n: i64 = from_bytes(b"i42e").unwrap();
        assert_eq!(n, 42);
    }

    #[test]
    fn deserialize_string() {
        let s: String = from_bytes(b"4:spam").unwrap();
        assert_eq!(s, "spam");
    }

    #[test]
    fn deserialize_vec() {
        let v: Vec<i64> = from_bytes(b"li1ei2ei3ee").unwrap();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn deserialize_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Person {
            name: String,
            age: i64,
        }

        let p: Person = from_bytes(b"d3:agei25e4:name5:Alicee").unwrap();
        assert_eq!(
            p,
            Person {
                name: "Alice".into(),
                age: 25
            }
        );
    }
}
