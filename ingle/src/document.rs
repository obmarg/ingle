use crate::{
    google::firestore::v1 as firestore,
    values::{DecodingError, DocumentValues, EncodingError},
};

pub trait Document: Sized {
    fn to_values(&self) -> Result<DocumentValues, EncodingError>;

    fn from_values(values: DocumentValues) -> Result<Self, DecodingError>;
}

impl Document for DocumentValues {
    fn to_values(&self) -> Result<DocumentValues, EncodingError> {
        Ok(self.clone())
    }

    fn from_values(values: DocumentValues) -> Result<Self, DecodingError> {
        Ok(values)
    }
}

pub struct DocumentResponse<D> {
    pub name: String,

    pub document: D,
}

impl<D> PartialEq for DocumentResponse<D>
where
    D: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.document.eq(&other.document)
    }
}

impl<D> Eq for DocumentResponse<D> where D: Eq {}

impl<D> std::fmt::Debug for DocumentResponse<D>
where
    D: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocumentResponse")
            .field("name", &self.name)
            .field("document", &self.document)
            .finish()
    }
}

impl DocumentResponse<DocumentValues> {
    pub(crate) fn try_from_firestore(
        doc: firestore::Document,
    ) -> Result<DocumentResponse<DocumentValues>, DecodingError> {
        Ok(DocumentResponse {
            name: doc.name,
            document: DocumentValues::try_from_firestore(doc.fields)?,
        })
    }
}

impl firestore::Document {
    pub(crate) fn try_into_document_response(
        self,
    ) -> Result<DocumentResponse<DocumentValues>, DecodingError> {
        DocumentResponse::try_from_firestore(self)
    }
}
