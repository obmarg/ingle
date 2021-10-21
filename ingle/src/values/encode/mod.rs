use std::collections::HashMap;

use super::{DocumentValues, EncodingError, Value};

pub mod document;
pub mod value;

type Result<T> = std::result::Result<T, EncodingError>;

pub trait EncodableDocument {
    fn encode(&self) -> Result<DocumentValues>;
}

pub trait EncodableValue {
    fn encode(&self) -> Result<Value>;
}

impl EncodableValue for bool {
    fn encode(&self) -> Result<Value> {
        value::encode_bool(*self)
    }
}

impl<T: EncodableValue> EncodableValue for &T {
    fn encode(&self) -> Result<Value> {
        (*self).encode()
    }
}
