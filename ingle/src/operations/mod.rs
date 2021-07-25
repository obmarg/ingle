mod add_document;
mod list_documents;

pub use self::{
    add_document::{AddDocumentOperation, AddDocumentRequest},
    list_documents::{ListDocumentsOperation, ListDocumentsRequest, ListDocumentsResponse},
};

use crate::{
    values::{DecodingError, EncodingError},
    FirestoreError,
};

pub(crate) trait IntoRequest {
    type Request;

    fn into_request(self) -> Result<Self::Request, OperationError>;
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum OperationError {
    #[error("Error from firestore: {0}")]
    FirestoreError(#[from] FirestoreError),
    #[error("Error encoding request: {0}")]
    EncodingError(#[from] EncodingError),
    #[error("Error decoding response: {0}")]
    DecodingError(#[from] DecodingError),
}
