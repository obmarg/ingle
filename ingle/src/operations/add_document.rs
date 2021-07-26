use std::marker::PhantomData;

use super::{IntoRequest, OperationError};
use crate::{
    document::{Document, DocumentResponse},
    executors::{BatchWriteExecutor, WriteExecutor},
    google::firestore::v1 as firestore,
    paths::CollectionPath,
    paths::ProjectPath,
    values::{DocumentValues, EncodingError},
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
#[must_use]
pub struct AddDocumentOperation<T> {
    collection_path: CollectionPath,

    document_id: Option<String>,

    document: Result<DocumentValues, EncodingError>,

    t: PhantomData<fn() -> T>,
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

    pub async fn run<E>(self, executor: E) -> Result<DocumentResponse<T>, OperationError>
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

    fn into_request(self) -> Result<Self::Request, OperationError> {
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

impl AddDocumentRequest {
    pub(crate) fn into_firestore_request(
        self,
        project_path: ProjectPath,
    ) -> firestore::CreateDocumentRequest {
        let (parent, collection_id) = self.collection_path.parent_and_collection_id(project_path);

        firestore::CreateDocumentRequest {
            parent,
            collection_id,
            document_id: self.document_id,
            document: Some(firestore::Document {
                name: String::new(),
                fields: self.document.into_firestore(),
                create_time: None,
                update_time: None,
            }),
            mask: None,
        }
    }

    pub(crate) fn into_firestore_write(self, project_path: ProjectPath) -> firestore::Write {
        firestore::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: None,
            operation: Some(firestore::write::Operation::Update(firestore::Document {
                name: self
                    .collection_path
                    .document(new_doc_id())
                    .full_path(project_path),
                fields: self.document.into_firestore(),
                create_time: None,
                update_time: None,
            })),
        }
    }
}

fn new_doc_id() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect()
}
