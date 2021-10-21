mod database;
mod document;
mod executors;
mod google;
mod paths;
mod refs;

pub mod operations;
pub mod values;

pub use self::{
    database::{Database, DatabaseBuilder, FirestoreError},
    document::Document,
    refs::{CollectionRef, DocumentRef},
    values::{EncodableDocument, EncodableValue},
};

pub mod transactions {
    pub use super::database::transactions::*;
}

// Re-export the proc macros
pub use ingle_proc_macros::EncodableDocument;
