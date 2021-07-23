use crate::{path::CollectionPath, values::DocumentValues};

pub struct AddDocumentRequest {
    collection_path: CollectionPath,
    document_id: String,
    document: DocumentValues,
}

impl AddDocumentRequest {
    pub(crate) fn new(
        collection_path: CollectionPath,
        document_id: String,
        document: DocumentValues,
    ) -> Self {
        Self {
            collection_path,
            document_id,
            document,
        }
    }
}

pub struct DocumentResponse<D> {
    pub name: String,
    pub document: D,
}
