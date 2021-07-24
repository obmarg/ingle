use std::collections::HashMap;

use crate::google::firestore::v1 as firestore;

#[derive(Clone, Debug)]
pub struct DocumentValues(HashMap<String, Value>);

impl DocumentValues {
    pub fn into_hashmap(self) -> HashMap<String, Value> {
        self.0
    }

    pub fn from_hashmap(values: HashMap<String, Value>) -> Self {
        DocumentValues(values)
    }

    pub(crate) fn into_firestore(self) -> HashMap<String, firestore::Value> {
        self.0
            .into_iter()
            .map(|(k, v)| (k, v.into_firestore()))
            .collect()
    }

    pub(crate) fn try_from_firestore(
        fields: HashMap<String, firestore::Value>,
    ) -> Result<Self, DecodingError> {
        Ok(DocumentValues(
            fields
                .into_iter()
                .map(|(k, v)| Ok((k, Value::try_from_firestore(v)?)))
                .collect::<Result<HashMap<_, _>, _>>()?,
        ))
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Double(f64),
    Timestamp(Timestamp),
    String(String),
    Bytes(Vec<u8>),
    DocumentReference(String),
    GeoPoint(LatLng),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

#[derive(Clone, Debug)]
pub struct Timestamp {
    pub seconds: i64,
    pub nandos: i32,
}

#[derive(Clone, Debug)]
pub struct LatLng {
    pub latitude: f64,
    pub longitude: f64,
}

impl Value {
    pub(crate) fn into_firestore(self) -> firestore::Value {
        use firestore::value::ValueType;

        let value_type = match self {
            Value::Null => ValueType::NullValue(0),
            Value::Boolean(b) => ValueType::BooleanValue(b),
            Value::Integer(v) => ValueType::IntegerValue(v),
            Value::Double(v) => ValueType::DoubleValue(v),
            Value::Timestamp(_) => todo!(),
            Value::String(s) => ValueType::StringValue(s),
            Value::Bytes(b) => ValueType::BytesValue(b),
            Value::DocumentReference(_) => todo!(),
            Value::GeoPoint(_) => todo!(),
            Value::Array(v) => ValueType::ArrayValue(firestore::ArrayValue {
                values: v.into_iter().map(Value::into_firestore).collect(),
            }),
            Value::Map(hashmap) => ValueType::MapValue(firestore::MapValue {
                fields: hashmap
                    .into_iter()
                    .map(|(k, v)| (k, v.into_firestore()))
                    .collect(),
            }),
        };

        firestore::Value {
            value_type: Some(value_type),
        }
    }

    pub fn try_from_firestore(value: firestore::Value) -> Result<Self, DecodingError> {
        use firestore::value::ValueType;

        let value_type = value.value_type.ok_or(DecodingError::NoValuePresent)?;

        Ok(match value_type {
            ValueType::NullValue(_) => Value::Null,
            ValueType::BooleanValue(b) => Value::Boolean(b),
            ValueType::IntegerValue(v) => Value::Integer(v),
            ValueType::DoubleValue(v) => Value::Double(v),
            ValueType::TimestampValue(_) => todo!(),
            ValueType::StringValue(s) => Value::String(s),
            ValueType::BytesValue(b) => Value::Bytes(b),
            ValueType::ReferenceValue(_) => todo!(),
            ValueType::GeoPointValue(_) => todo!(),
            ValueType::ArrayValue(firestore::ArrayValue { values }) => Value::Array(
                values
                    .into_iter()
                    .map(Value::try_from_firestore)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            ValueType::MapValue(firestore::MapValue { fields }) => Value::Map(
                fields
                    .into_iter()
                    .map(|(k, v)| Ok((k, Value::try_from_firestore(v)?)))
                    .collect::<Result<HashMap<_, _>, _>>()?,
            ),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DecodingError {
    #[error("No value was present in the response")]
    NoValuePresent,
}

#[derive(thiserror::Error, Debug)]
pub enum EncodingError {}
