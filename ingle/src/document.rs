use crate::{google::firestore::v1 as firestore, values::DocumentValues};

pub trait Document: Sized {
    fn to_values(&self) -> Result<DocumentValues, ()>;

    fn from_values(values: DocumentValues) -> Result<Self, ()>;
}

impl Document for DocumentValues {
    fn to_values(&self) -> Result<DocumentValues, ()> {
        Ok(self.clone())
    }

    fn from_values(values: DocumentValues) -> Result<Self, ()> {
        Ok(values)
    }
}

pub struct DocumentResponse<D> {
    pub document: D,
}

impl DocumentResponse<DocumentValues> {
    pub(crate) fn try_from_firestore(
        doc: firestore::Document,
    ) -> Result<DocumentResponse<DocumentValues>, ()> {
        Ok(DocumentResponse {
            name: doc.name,
            document: DocumentValues::try_from_firestore(doc.fields)?,
        })
    }
}

impl firestore::Document {
    pub(crate) fn try_into_document_response(self) -> Result<DocumentResponse<DocumentValues>, ()> {
        DocumentResponse::try_from_firestore(self)
    }
}
