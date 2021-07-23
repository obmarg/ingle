mod add_document;

pub use add_document::{AddDocumentOperation, AddDocumentRequest};

pub(crate) trait IntoRequest {
    type Request;

    fn into_request(self) -> Result<Self::Request, ()>;
}
