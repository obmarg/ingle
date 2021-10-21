use std::collections::HashMap;

use super::{DocumentValues, EncodingError, Value};

pub mod document;
mod value_encoder;

pub use self::value_encoder::ValueEncoder;

type Result<T> = std::result::Result<T, EncodingError>;

pub trait EncodableDocument {
    fn encode(&self) -> Result<DocumentValues>;
}

pub trait EncodableValue {
    fn encode(&self, encoder: ValueEncoder) -> Result<Value>;
}

impl EncodableValue for bool {
    fn encode(&self, encoder: ValueEncoder) -> Result<Value> {
        encoder.encode_bool(*self)
    }
}

impl<T: EncodableValue> EncodableValue for &T {
    fn encode(&self, encoder: ValueEncoder) -> Result<Value> {
        (*self).encode(encoder)
    }
}
