use serde::ser::{self, Serialize};

use crate::error::{Error, Result};

/// A serializer for bencode data
pub struct Serializer {
    output: Vec<u8>,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer { output: Vec::new() }
    }
}

/// Serialize a value to bencode bytes
pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = SortedMapSerializer<'a>;
    type SerializeStruct = SortedMapSerializer<'a>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_i64(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes());
        self.output.push(b'e');
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::Message("bencode does not support floats".into()))
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::Message("bencode does not support floats".into()))
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.extend(v.len().to_string().as_bytes());
        self.output.push(b':');
        self.output.extend(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::Message("bencode does not support enums".into()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()> {
        Err(Error::Message("bencode does not support enums".into()))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output.push(b'l');
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Message("bencode does not support enums".into()))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SortedMapSerializer::new(self))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Message("bencode does not support enums".into()))
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        Err(Error::Message("bencode does not support enums".into()))
    }

    fn end(self) -> Result<()> {
        Err(Error::Message("bencode does not support enums".into()))
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()> {
        Err(Error::Message("bencode does not support enums".into()))
    }

    fn end(self) -> Result<()> {
        Err(Error::Message("bencode does not support enums".into()))
    }
}

/// A serializer that collects map entries and sorts them by key
pub struct SortedMapSerializer<'a> {
    ser: &'a mut Serializer,
    entries: Vec<(Vec<u8>, Vec<u8>)>,
    current_key: Option<Vec<u8>>,
}

impl<'a> SortedMapSerializer<'a> {
    fn new(ser: &'a mut Serializer) -> Self {
        SortedMapSerializer {
            ser,
            entries: Vec::new(),
            current_key: None,
        }
    }
}

impl<'a> ser::SerializeMap for SortedMapSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        let mut key_serializer = Serializer::new();
        key.serialize(&mut key_serializer)?;
        self.current_key = Some(key_serializer.output);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        let key = self
            .current_key
            .take()
            .ok_or_else(|| Error::Message("serialize_value called before serialize_key".into()))?;
        let mut value_serializer = Serializer::new();
        value.serialize(&mut value_serializer)?;
        self.entries.push((key, value_serializer.output));
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.ser.output.push(b'd');
        let mut entries = self.entries;
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        for (key, value) in entries {
            self.ser.output.extend(key);
            self.ser.output.extend(value);
        }
        self.ser.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for SortedMapSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        ser::SerializeMap::serialize_key(self, key)?;
        ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeMap::end(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn serialize_integer() {
        assert_eq!(to_bytes(&42i64).unwrap(), b"i42e");
        assert_eq!(to_bytes(&-3i64).unwrap(), b"i-3e");
    }

    #[test]
    fn serialize_string() {
        assert_eq!(to_bytes(&"spam").unwrap(), b"4:spam");
        assert_eq!(to_bytes(&String::from("hello")).unwrap(), b"5:hello");
    }

    #[test]
    fn serialize_vec() {
        let v = vec![1i64, 2, 3];
        assert_eq!(to_bytes(&v).unwrap(), b"li1ei2ei3ee");
    }

    #[test]
    fn serialize_struct() {
        #[derive(Serialize)]
        struct Person {
            age: i64,
            name: String,
        }

        let p = Person {
            age: 25,
            name: "Alice".into(),
        };
        // Keys should be sorted: "age" before "name"
        assert_eq!(to_bytes(&p).unwrap(), b"d3:agei25e4:name5:Alicee");
    }

    #[test]
    fn serialize_struct_unsorted_fields() {
        #[derive(Serialize)]
        struct Data {
            zebra: i64,
            apple: i64,
        }

        let d = Data { zebra: 1, apple: 2 };
        // Keys should be sorted: "apple" before "zebra"
        assert_eq!(to_bytes(&d).unwrap(), b"d5:applei2e5:zebrai1ee");
    }
}
