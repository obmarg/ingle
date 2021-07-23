use std::collections::HashMap;

use crate::path::DocumentPath;

#[derive(Clone, Debug)]
pub struct DocumentValues(HashMap<String, Value>);

impl DocumentValues {
    pub fn into_hashmap(self) -> HashMap<String, Value> {
        self.0
    }

    pub fn from_hashmap(values: HashMap<String, Value>) -> Self {
        DocumentValues(values)
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

