use crate::values::DocumentValues;

use crate::{operations, requests};

enum Error {}

trait ReadExecutor {}

#[async_trait::async_trait]
pub trait WriteExecutor {
    async fn add_document(
        &self,
        input: operations::AddDocumentRequest,
    ) -> Result<requests::DocumentResponse<DocumentValues>, ()>;
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
