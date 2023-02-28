use async_trait::async_trait;
use futures_channel::mpsc::UnboundedSender;

use crate::{
    executors::{BatchWriteExecutor, ReadExecutor},
    google::firestore::v1 as firestore,
    operations,
    paths::ProjectPath,
    values::DocumentValues,
    FirestoreError,
};

pub struct ReadPhaseExecutor {
    pub(super) database: super::Database,
    pub(super) transaction_id: Vec<u8>,
    pub(super) writes: UnboundedSender<WriteRequest>,
}

impl ReadPhaseExecutor {
    pub fn finish_reads(self) -> WritePhaseExecutor {
        WritePhaseExecutor {
            writes: self.writes,
        }
    }
}

pub struct ReadOnlyExecutor {
    pub(super) database: super::Database,
    pub(super) transaction_id: Vec<u8>,
}

macro_rules! impl_read_executor {
    ($target:ty) => {
        #[async_trait]
        impl ReadExecutor for $target {
            async fn list_documents(
                &self,
                input: operations::ListDocumentsRequest,
            ) -> Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError> {
                self.database
                    .list_documents(input.in_transaction(self.transaction_id.clone()))
                    .await
            }
        }
    };
}

impl_read_executor!(ReadPhaseExecutor);
impl_read_executor!(ReadOnlyExecutor);

// TODO: is this a good name?
pub struct WritePhaseExecutor {
    writes: UnboundedSender<WriteRequest>,
}

// Ok, so ideal semantics:
// - Rollback can be called, will abort transaction and trigger a retry.
// - Commit can be called to commit early (failure will trigger a retry when control returns).
//
// We commit on return if no rollback/commit was called (retry on fail)
//
// Transaction functions are assumed to be succesful unless one of the above retry paths
// happens.
//

impl WritePhaseExecutor {
    pub async fn rollback(self) {
        self.writes.unbounded_send(WriteRequest::Rollback).ok();
    }

    pub async fn commit(self) {
        self.writes.unbounded_send(WriteRequest::Commit).ok();
    }
}

#[async_trait]
impl BatchWriteExecutor for WritePhaseExecutor {
    async fn add_document(&self, input: operations::AddDocumentRequest) {
        self.writes
            .unbounded_send(WriteRequest::AddDocument(input))
            .expect("unbounded_send failed in add_document");
    }
}

pub(super) enum WriteRequest {
    Commit,
    Rollback,
    AddDocument(operations::AddDocumentRequest),
}

impl WriteRequest {
    pub fn into_firestore_write(self, project_path: &ProjectPath) -> Option<firestore::Write> {
        // TODO: If random IDs are needed do something like this:
        // https://github.com/googleapis/python-firestore/blob/8703b48c45e7bb742a794cad9597740c44182f81/google/cloud/firestore_v1/base_collection.py#L465
        match self {
            WriteRequest::Rollback => {
                panic!("Trying to write a rollback request.  This shouldn't happen")
            }
            WriteRequest::Commit => None,
            WriteRequest::AddDocument(request) => {
                Some(request.into_firestore_write(project_path.clone()))
            }
        }
    }
}
