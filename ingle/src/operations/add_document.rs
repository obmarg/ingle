use std::marker::PhantomData;

use super::IntoRequest;
use crate::{
    document::Document, executors::WriteExecutor, path::CollectionPath, requests::DocumentResponse,
    values::DocumentValues,
};

impl crate::CollectionRef {
    pub fn add_document<T>(&self, document: &T) -> AddDocumentOperation<T>
    where
        T: Document,
    {
        AddDocumentOperation::new(self.path.clone(), document)
    }
}

#[derive(Debug)]
pub struct AddDocumentOperation<T> {
    collection_path: CollectionPath,

    document_id: Option<String>,

    document: Result<DocumentValues, ()>,

    t: PhantomData<T>,
}

impl<T> AddDocumentOperation<T>
where
    T: Document,
{
    fn new(collection_path: CollectionPath, document: &T) -> Self {
        Self {
            collection_path,
            document: document.to_values(),
            document_id: None,
            t: PhantomData,
        }
    }

    pub fn with_id(self, id: impl Into<String>) -> Self {
        Self {
            document_id: Some(id.into()),
            ..self
        }
    }

    pub async fn run<E>(self, executor: E) -> Result<DocumentResponse<T>, ()>
    where
        E: WriteExecutor,
    {
        let response = executor.add_document(self.into_request()?).await?;

        Ok(DocumentResponse {
            name: response.name,
            document: T::from_values(response.document)?,
        })
    }
}

impl<T> IntoRequest for AddDocumentOperation<T> {
    type Request = AddDocumentRequest;

    fn into_request(self) -> Result<Self::Request, ()> {
        Ok(AddDocumentRequest {
            collection_path: self.collection_path,
            document_id: self.document_id.unwrap_or_default(),
            document: self.document?,
        })
    }
}

pub struct AddDocumentRequest {
    collection_path: CollectionPath,
    document_id: String,
    document: DocumentValues,
}