use tonic::{codegen::InterceptedService, transport::Channel};

use crate::{
    document::DocumentResponse,
    executors::WriteExecutor,
    google::firestore::v1::firestore_client::FirestoreClient,
    operations::{self, IntoRequest},
    paths::ProjectPath,
    values::DocumentValues,
};

mod auth;
mod builder;

pub use builder::{ConnectError, DatabaseBuilder};

pub struct Database {
    client: FirestoreClient<InterceptedService<Channel, Interceptor>>,
    project_path: ProjectPath,
}

type Interceptor =
    Box<dyn FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> + Send + Sync>;

#[async_trait::async_trait]
impl WriteExecutor for Database {
    async fn add_document(
        &self,
        input: operations::AddDocumentRequest,
    ) -> Result<DocumentResponse<DocumentValues>, ()> {
        let mut client = self.client.clone();

        Ok(client
            .create_document(input.into_firestore_request(self.project_path.clone()))
            .await
            .expect("TODO: errors")
            .into_inner()
            .try_into_document_response()?)
    }
}
