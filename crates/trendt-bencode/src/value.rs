use std::collections::BTreeMap;

/// Represents a bencode value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// Integer: i<number>e (e.g., i42e)
    Integer(i64),
    /// Byte string: <length>:<data> (e.g., 4:spam)
    ByteString(Vec<u8>),
    /// List: l<items>e (e.g., li1ei2ee)
    List(Vec<Value>),
    /// Dictionary: d<pairs>e - keys must be sorted byte strings
    Dict(BTreeMap<Vec<u8>, Value>),
}
