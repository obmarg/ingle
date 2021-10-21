use std::collections::HashMap;

use super::{EncodableValue, EncodingError, Result, Value};

pub fn encode_bool(v: bool) -> Result<Value> {
    Ok(Value::Boolean(v))
}

// The protobuf definitions only have an i64 type.
pub fn encode_i8(v: i8) -> Result<Value> {
    encode_i64(i64::from(v))
}

pub fn encode_i16(v: i16) -> Result<Value> {
    encode_i64(i64::from(v))
}

pub fn encode_i32(v: i32) -> Result<Value> {
    encode_i64(i64::from(v))
}

pub fn encode_i64(v: i64) -> Result<Value> {
    Ok(Value::Integer(v))
}

pub fn encode_u8(v: u8) -> Result<Value> {
    encode_i64(i64::from(v))
}

pub fn encode_u16(v: u16) -> Result<Value> {
    encode_i64(i64::from(v))
}

pub fn encode_u32(v: u32) -> Result<Value> {
    encode_i64(i64::from(v))
}

pub fn encode_u64(v: u64) -> Result<Value> {
    if v >= (std::i64::MAX as u64) {
        Err(EncodingError::IntegerOverflow)
    } else {
        encode_i64(v as i64)
    }
}

pub fn encode_f32(v: f32) -> Result<Value> {
    encode_f64(f64::from(v))
}

pub fn encode_f64(v: f64) -> Result<Value> {
    Ok(Value::Double(v))
}

// Serialize a char as a single-character string.
pub fn encode_char(v: char) -> Result<Value> {
    Ok(Value::String(v.to_string()))
}

pub fn encode_str(v: &str) -> Result<Value> {
    Ok(Value::String(v.to_string()))
}

pub fn encode_bytes(v: &[u8]) -> Result<Value> {
    Ok(Value::Bytes(v.to_vec()))
}

// An absent optional is represented as the JSON `null`.
pub fn encode_none() -> Result<Value> {
    Ok(Value::Null)
}

// At present optional is represented as just the contained value. Note that
// this is a lossy representation. For example the values `Some(())` and
// `None` both serialize as just `null`.
pub fn encode_some<T>(value: &T) -> Result<Value>
where
    T: ?Sized + EncodableValue,
{
    value.encode()
}

// In Serde, unit means an anonymous value containing no data.
// We map this to a null value
pub fn encode_unit() -> Result<Value> {
    encode_none()
}

// Unit struct means a named value containing no data.
// We basically just treat this like null for now.
pub fn encode_unit_struct(_name: &'static str) -> Result<Value> {
    encode_unit()
}

pub fn encode_unit_variant(
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
) -> Result<Value> {
    encode_str(variant)
}

// We treat newtype structs as insignificant wrappers around the data they
// contain.
pub fn encode_newtype_struct<T>(_name: &'static str, value: &T) -> Result<Value>
where
    T: ?Sized + EncodableValue,
{
    value.encode()
}

// Note that newtype variant (and all of the other variant serialization
// methods) refer exclusively to the "externally tagged" enum
// representation.
pub fn encode_newtype_variant<T>(
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
) -> Result<Value>
where
    T: ?Sized + EncodableValue,
{
    let mut map_serializer = encode_map(Some(2))?;
    map_serializer.encode_entry(variant.to_string(), value)?;
    map_serializer.end()
}

/*
fn encode_seq(len: Option<usize>) -> Result<Self::SerializeSeq> {
    todo!()
    /*
    let vec = match len {
        None => Vec::new(),
        Some(len) => Vec::with_capacity(len),
    };
    Ok(SequenceValueSerializer { items: vec })
    */
}

fn encode_tuple(len: usize) -> Result<Self::SerializeTuple> {
    todo!()
    /*
    Ok(SequenceValueSerializer {
        items: Vec::with_capacity(len),
    })
    */
}

// We treat tuple structs exactly like tuples for now.
fn encode_tuple_struct(
    self,
    _name: &'static str,
    len: usize,
) -> Result<Self::SerializeTupleStruct> {
    encode_tuple(len)
}

// Serialize a tuple variant.
//
// This method is only responsible for the externally tagged representation.
fn encode_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    len: usize,
) -> Result<Self::SerializeTupleVariant> {
    todo!()
    /*
    Ok(NamedSequenceValueSerializer {
        named_sequence_serializer: NamedSequenceSerializer {
            name: variant.to_string(),
            items: Vec::with_capacity(len),
        },
    })
    */
}
*/

pub fn encode_map(len: Option<usize>) -> Result<ValueMapEncoder> {
    let map = len.map(HashMap::with_capacity).unwrap_or_default();

    Ok(ValueMapEncoder { map })
}

pub fn encode_struct(_name: &'static str, len: usize) -> Result<ValueMapEncoder> {
    encode_map(Some(len))
}

/*
fn encode_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    len: usize,
) -> Result<Self::SerializeStructVariant> {
    Ok(NamedMapValueSerializer {
        named_map_serializer: NamedMapSerializer {
            name: variant.to_string(),
            items: HashMap::with_capacity(len),
        },
    })
}
*/

// TODO: Now add the firestore specific types

// TODO: The ValueEncoder impl

pub struct ValueMapEncoder {
    map: HashMap<String, Value>,
}

impl ValueMapEncoder {
    pub fn encode_entry<V>(&mut self, key: String, value: &V) -> Result<()>
    where
        V: ?Sized + EncodableValue,
    {
        self.map.insert(key, value.encode()?);
        Ok(())
    }

    pub fn end(self) -> Result<Value> {
        Ok(Value::Map(self.map))
    }
}
