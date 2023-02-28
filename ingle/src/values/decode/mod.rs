use super::{DecodingError, DocumentValues, Value};

pub mod document;
//pub mod value;

type Result<T> = std::result::Result<T, DecodingError>;

pub trait DecodableDocument {
    fn decode(document: DocumentValues) -> Result<Self>;
}

pub trait DecodableValue {
    fn decode(value: Value) -> Result<Self>;
}
