use async_trait::async_trait;

use crate::{
    document::DocumentResponse,
    executors::{ReadExecutor, WriteExecutor},
    operations,
    values::DocumentValues,
    FirestoreError,
};

pub struct ReadPhaseExecutor {
    pub(super) database: super::Database,
    pub(super) transaction_id: Vec<u8>,
}

impl ReadPhaseExecutor {
    pub fn finish_reads(self) -> WritePhaseExecutor {
        WritePhaseExecutor {
            database: self.database,
            transaction_id: self.transaction_id,
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
    database: super::Database,
    transaction_id: Vec<u8>,
}

#[async_trait]
impl WriteExecutor for WritePhaseExecutor {
    async fn add_document(
        &self,
        input: operations::AddDocumentRequest,
    ) -> Result<DocumentResponse<DocumentValues>, FirestoreError> {
        todo!()
    }
}
