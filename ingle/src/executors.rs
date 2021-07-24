use crate::{document::DocumentResponse, operations, values::DocumentValues, FirestoreError};

trait ReadExecutor {}

#[async_trait::async_trait]
pub trait WriteExecutor: Send + Sync {
    async fn add_document(
        &self,
        input: operations::AddDocumentRequest,
    ) -> Result<DocumentResponse<DocumentValues>, FirestoreError>;
}

#[async_trait::async_trait]
impl<T> WriteExecutor for &T
where
    T: WriteExecutor,
{
    async fn add_document(
        &self,
        input: operations::AddDocumentRequest,
    ) -> Result<DocumentResponse<DocumentValues>, FirestoreError> {
        (*self).add_document(input).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestExecutor {}

    impl ReadExecutor for TestExecutor {}

    #[test]
    fn executor_is_object_safe() {
        let _: Box<dyn ReadExecutor> = Box::new(TestExecutor {});
    }
}
