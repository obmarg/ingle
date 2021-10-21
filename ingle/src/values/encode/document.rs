use std::collections::HashMap;

use super::{DocumentValues, EncodableDocument, EncodableValue, Result, Value, ValueEncoder};

pub fn encode_newtype_struct<T>(_name: &'static str, value: &T) -> Result<DocumentValues>
where
    T: ?Sized + EncodableDocument,
{
    // We treat newtype structs as insignificant wrappers around the data they
    // contain.
    value.encode()
}

// Note that newtype variant (and all of the other variant methods) refers
// to the "externally tagged" enum representation.
pub fn encode_newtype_variant<T>(
    // TODO: Do we need these params?  Maybe not
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
) -> Result<DocumentValues>
where
    T: ?Sized + EncodableValue,
{
    let mut serializer = encode_map(Some(2))?;
    serializer.encode_entry(variant.to_string(), value)?;
    serializer.end()
}

pub fn encode_map(len: Option<usize>) -> Result<DocumentMapEncoder> {
    let map = len.map(HashMap::with_capacity).unwrap_or_default();

    Ok(DocumentMapEncoder { map })
}

pub fn encode_struct(_name: &'static str, len: usize) -> Result<DocumentMapEncoder> {
    encode_map(Some(len))
}

// TODO: encode_tuple_variant (handles externally tagged representation)
//       encode_struct_variant

pub struct DocumentMapEncoder {
    map: HashMap<String, Value>,
}

impl DocumentMapEncoder {
    pub fn encode_entry<V>(&mut self, key: String, value: &V) -> Result<()>
    where
        V: ?Sized + EncodableValue,
    {
        self.map.insert(key, value.encode(ValueEncoder {})?);
        Ok(())
    }

    pub fn end(self) -> Result<DocumentValues> {
        Ok(DocumentValues::from_hashmap(self.map))
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::*;

    #[test]
    fn encode_document_newtype_variant() {
        let document = encode_newtype_variant("", 0, "UserId", &true).unwrap();

        assert_eq!(
            document.0,
            hashmap! { "UserId".to_string() => Value::Boolean(true)}
        );
    }

    #[test]
    fn encode_document_map() {
        let mut map_encoder = encode_map(None).unwrap();

        map_encoder.encode_entry("1".to_string(), &true).unwrap();
        map_encoder.encode_entry("2".to_string(), &false).unwrap();

        let document = map_encoder.end().unwrap();

        assert_eq!(
            document.0,
            hashmap! {
                "1".to_string() => Value::Boolean(true),
                "2".to_string() => Value::Boolean(false),
            }
        );
    }
}
