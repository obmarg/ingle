use std::marker::PhantomData;

use super::{IntoRequest, OperationError};
use crate::{
    document::{Document, DocumentResponse},
    executors::{BatchWriteExecutor, WriteExecutor},
    google::firestore::v1 as firestore,
    paths::ProjectPath,
    paths::{CollectionPath, DocumentPath},
    values::{DocumentValues, EncodingError},
};

impl crate::DocumentRef {
    pub fn set<T>(&self, document: &T) -> SetDocumentOperation<T>
    where
        T: Document,
    {
        SetDocumentOperation::new(self.path.clone(), document)
    }
}

#[derive(Debug)]
#[must_use]
pub struct SetDocumentOperation<T> {
    document_path: DocumentPath,

    document: Result<DocumentValues, EncodingError>,

    t: PhantomData<fn() -> T>,
}

impl<T> SetDocumentOperation<T>
where
    T: Document,
{
    fn new(document_path: DocumentPath, document: &T) -> Self {
        Self {
            document_path,
            document: document.to_values(),
            t: PhantomData,
        }
    }

    pub async fn run<E>(self, executor: E) -> Result<DocumentResponse<T>, OperationError>
    where
        E: WriteExecutor,
    {
        let response = executor.set_document(self.into_request()?).await?;

        Ok(DocumentResponse {
            name: response.name,
            document: T::from_values(response.document)?,
        })
    }
}

impl<T> IntoRequest for SetDocumentOperation<T> {
    type Request = SetDocumentRequest;

    fn into_request(self) -> Result<Self::Request, OperationError> {
        Ok(SetDocumentRequest {
            document_path: self.document_path,
            document: self.document?,
        })
    }
}

pub struct SetDocumentRequest {
    document_path: DocumentPath,
    document: DocumentValues,
}

impl SetDocumentRequest {
    pub(crate) fn into_firestore_request(
        self,
        project_path: ProjectPath,
    ) -> firestore::UpdateDocumentRequest {
        firestore::UpdateDocumentRequest {
            document: Some(firestore::Document {
                name: self.document_path.full_path(project_path),
                fields: self.document.into_firestore(),
                create_time: None,
                update_time: None,
            }),
            mask: None,
            update_mask: None,
            current_document: None,
        }
    }

    pub(crate) fn into_firestore_write(self, project_path: ProjectPath) -> firestore::Write {
        firestore::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: None,
            operation: Some(firestore::write::Operation::Update(firestore::Document {
                name: self.document_path.full_path(project_path),
                fields: self.document.into_firestore(),
                create_time: None,
                update_time: None,
            })),
        }
    }
}

