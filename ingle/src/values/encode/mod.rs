use std::collections::HashMap;

use super::{DocumentValues, EncodingError, Value};

mod document_encoder;
mod value_encoder;

pub use document_encoder::DocumentEncoder;
use value_encoder::ValueEncoder;

type Result<T> = std::result::Result<T, EncodingError>;

pub trait EncodableDocument {
    fn encode(&self, encoder: DocumentEncoder) -> Result<DocumentValues>;
}

pub trait EncodableValue {
    fn encode(&self, encoder: ValueEncoder) -> Result<Value>;
}

impl EncodableValue for bool {
    fn encode(&self, encoder: ValueEncoder) -> Result<Value> {
        encoder.encode_bool(*self)
    }
}
