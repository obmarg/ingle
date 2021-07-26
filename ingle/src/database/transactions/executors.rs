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

pub struct WritePhaseExecutor {
    writes: UnboundedSender<WriteRequest>,
}

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
