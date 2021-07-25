use async_trait::async_trait;

use crate::{document::DocumentResponse, operations, values::DocumentValues, FirestoreError};

#[async_trait]
pub trait ReadExecutor: Send + Sync {
    async fn list_documents(
        &self,
        input: operations::ListDocumentsRequest,
    ) -> Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError>;
}

#[async_trait]
impl<T> ReadExecutor for &T
where
    T: ReadExecutor,
{
    async fn list_documents(
        &self,
        input: operations::ListDocumentsRequest,
    ) -> Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError> {
        (*self).list_documents(input).await
    }
}

#[async_trait]
impl ReadExecutor for &dyn ReadExecutor {
    async fn list_documents(
        &self,
        input: operations::ListDocumentsRequest,
    ) -> Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError> {
        (*self).list_documents(input).await
    }
}

#[async_trait]
pub trait WriteExecutor: Send + Sync {
    async fn add_document(
        &self,
        input: operations::AddDocumentRequest,
    ) -> Result<DocumentResponse<DocumentValues>, FirestoreError>;
}

#[async_trait]
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
pub(crate) mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    pub struct TestExecutor {
        list_documents_result: TestExecutorField<
            Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError>,
        >,
    }

    impl Default for TestExecutor {
        fn default() -> Self {
            TestExecutor {
                list_documents_result: TestExecutorField::one(None),
            }
        }
    }

    impl TestExecutor {
        pub fn list_documents_result(
            self,
            result: Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError>,
        ) -> Self {
            TestExecutor {
                list_documents_result: TestExecutorField::one(Some(result)),
            }
        }

        pub fn list_documents_results(
            self,
            results: Vec<Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError>>,
        ) -> Self {
            TestExecutor {
                list_documents_result: TestExecutorField::many(results),
            }
        }
    }

    struct TestExecutorField<T> {
        inner: Arc<Mutex<TestExecutorFieldInner<T>>>,
    }

    impl<T> TestExecutorField<T> {
        fn one(inner: Option<T>) -> Self {
            TestExecutorField {
                inner: Arc::new(Mutex::new(TestExecutorFieldInner::One(inner))),
            }
        }

        fn many(inner: Vec<T>) -> Self {
            let mut inner = inner;
            inner.reverse();
            TestExecutorField {
                inner: Arc::new(Mutex::new(TestExecutorFieldInner::Many(inner))),
            }
        }

        fn take(&self) -> Option<T> {
            let mut guard = self.inner.lock().unwrap();
            match &mut *guard {
                TestExecutorFieldInner::One(opt) => opt.take(),
                TestExecutorFieldInner::Many(vec) => vec.pop(),
            }
        }
    }

    enum TestExecutorFieldInner<T> {
        One(Option<T>),
        Many(Vec<T>),
    }

    #[async_trait]
    impl ReadExecutor for TestExecutor {
        async fn list_documents(
            &self,
            _: operations::ListDocumentsRequest,
        ) -> Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError> {
            self.list_documents_result
                .take()
                .unwrap_or(Err(FirestoreError::UnknownError))
        }
    }

    #[test]
    fn read_executor_is_object_safe() {
        let _: Box<dyn ReadExecutor> = Box::new(TestExecutor::default());
    }

    #[async_trait]
    impl WriteExecutor for TestExecutor {
        async fn add_document(
            &self,
            _: operations::AddDocumentRequest,
        ) -> Result<DocumentResponse<DocumentValues>, FirestoreError> {
            Err(FirestoreError::UnknownError)
        }
    }

    #[test]
    fn write_executor_is_object_safe() {
        let _: Box<dyn WriteExecutor> = Box::new(TestExecutor::default());
    }
}
