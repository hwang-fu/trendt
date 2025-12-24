use crate::value::Value;

/// Encode a bencode value to bytes
pub fn encode(value: &Value) -> Vec<u8> {
    let mut output = Vec::new();
    encode_value(value, &mut output);
    output
}

fn encode_value(value: &Value, output: &mut Vec<u8>) {
    match value {
        Value::Integer(n) => encode_integer(*n, output),
        Value::ByteString(bytes) => encode_byte_string(bytes, output),
        Value::List(items) => encode_list(items, output),
        Value::Dict(map) => encode_dict(map, output),
    }
}

fn encode_integer(n: i64, output: &mut Vec<u8>) {
    output.push(b'i');
    output.extend(n.to_string().as_bytes());
    output.push(b'e');
}

fn encode_byte_string(bytes: &[u8], output: &mut Vec<u8>) {
    output.extend(bytes.len().to_string().as_bytes());
    output.push(b':');
    output.extend(bytes);
}

fn encode_list(items: &[Value], output: &mut Vec<u8>) {
    output.push(b'l');
    for item in items {
        encode_value(item, output);
    }
    output.push(b'e');
}

fn encode_dict(map: &std::collections::BTreeMap<Vec<u8>, Value>, output: &mut Vec<u8>) {
    output.push(b'd');
    // BTreeMap iterates in sorted order, so keys are already sorted
    for (key, value) in map {
        encode_byte_string(key, output);
        encode_value(value, output);
    }
    output.push(b'e');
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn encode_integer() {
        assert_eq!(encode(&Value::Integer(42)), b"i42e");
        assert_eq!(encode(&Value::Integer(-3)), b"i-3e");
        assert_eq!(encode(&Value::Integer(0)), b"i0e");
    }

    #[test]
    fn encode_byte_string() {
        assert_eq!(encode(&Value::ByteString(b"spam".to_vec())), b"4:spam");
        assert_eq!(encode(&Value::ByteString(vec![])), b"0:");
    }

    #[test]
    fn encode_list() {
        let list = Value::List(vec![Value::Integer(1), Value::Integer(2)]);
        assert_eq!(encode(&list), b"li1ei2ee");
    }

    #[test]
    fn encode_empty_list() {
        assert_eq!(encode(&Value::List(vec![])), b"le");
    }

    #[test]
    fn encode_dict() {
        let mut map = BTreeMap::new();
        map.insert(b"foo".to_vec(), Value::Integer(1));
        assert_eq!(encode(&Value::Dict(map)), b"d3:fooi1ee");
    }

    #[test]
    fn encode_empty_dict() {
        assert_eq!(encode(&Value::Dict(BTreeMap::new())), b"de");
    }

    #[test]
    fn round_trip() {
        use crate::decode::Decoder;

        let original = b"d3:bar4:spam3:fooli1ei2eee";
        let mut decoder = Decoder::new(original);
        let value = decoder.decode_value().unwrap();
        let encoded = encode(&value);
        assert_eq!(encoded, original);
    }
}
