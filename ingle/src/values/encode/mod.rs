use super::{DocumentValues, EncodingError, Value};

pub mod document;
pub mod value;

type Result<T> = std::result::Result<T, EncodingError>;

pub trait EncodableDocument {
    fn encode(&self) -> Result<DocumentValues>;
}

// TODO: impl EncodableDocument for types which make sense (mostly HashMap etc)

pub trait EncodableValue {
    fn encode(&self) -> Result<Value>;
}

// TODO: impl this for other primitives
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

// TODO: Actually, do we even need half the encode_X functions
// when we can just use the trait impls directly?
