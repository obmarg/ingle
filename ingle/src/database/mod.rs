use async_trait::async_trait;
use tonic::transport::Channel;

use crate::{
    document::DocumentResponse,
    executors::{ReadExecutor, WriteExecutor},
    google::firestore::v1 as firestore,
    operations,
    paths::ProjectPath,
    values::{DecodingError, DocumentValues},
};

mod auth;
mod builder;
pub mod transactions;

pub use builder::{ConnectError, DatabaseBuilder};

use self::auth::AuthService;

type FirestoreClient = firestore::firestore_client::FirestoreClient<AuthService<Channel>>;

#[derive(Clone)]
pub struct Database {
    client: FirestoreClient,
    project_path: ProjectPath,
}

// Ok, so the transaction API.
// Need a Transaction executor.
// Probably split in two for the read phase and the write phase.
//
// Also need to implement the retry mechanisms. Probably via a Transaction trait
// or similar (with a default impl for closures)

#[async_trait]
impl ReadExecutor for Database {
    async fn list_documents(
        &self,
        input: operations::ListDocumentsRequest,
    ) -> Result<operations::ListDocumentsResponse<DocumentValues>, FirestoreError> {
        let mut client = self.client.clone();

        let response = client
            .list_documents(input.into_firestore_request(self.project_path.clone()))
            .await?
            .into_inner();

        let next_page_token = if response.next_page_token.is_empty() {
            None
        } else {
            Some(response.next_page_token)
        };

        Ok(operations::ListDocumentsResponse {
            next_page_token,
            documents: response
                .documents
                .into_iter()
                .map(DocumentResponse::try_from_firestore)
                .collect(),
        })
    }
}

#[async_trait]
impl WriteExecutor for Database {
    async fn add_document(
        &self,
        input: operations::AddDocumentRequest,
    ) -> Result<DocumentResponse<DocumentValues>, FirestoreError> {
        let mut client = self.client.clone();

        Ok(client
            .create_document(input.into_firestore_request(self.project_path.clone()))
            .await?
            .into_inner()
            .try_into_document_response()?)
    }

    async fn set_document(
        &self,
        input: operations::SetDocumentRequest,
    ) -> Result<DocumentResponse<DocumentValues>, FirestoreError> {
        let mut client = self.client.clone();

        Ok(client
            .update_document(input.into_firestore_request(self.project_path.clone()))
            .await?
            .into_inner()
            .try_into_document_response()?)
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum FirestoreError {
    #[error("unknown error")]
    UnknownError,
    #[error("operation was cancelled")]
    Cancelled,
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("deadling exceeded")]
    DeadlineExceeded,
    #[error("not found")]
    NotFound,
    #[error("already exists")]
    AlreadyExists,
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("resource exhausted: {0}")]
    ResourceExhausted(String),
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),
    #[error("aborted: {0}")]
    Aborted(String),
    #[error("out of range: {0}")]
    OutOfRange(String),
    #[error("unimplemented")]
    Unimplemented,
    #[error("internal error. try again")]
    Internal,
    #[error("service unavailable. try again")]
    Unavailable,
    #[error("unrecoverable data loss or corruption")]
    DataLoss,
    #[error("unauthenticated: {0}")]
    Unauthenticated(String),
    #[error("Malformed response: {0}")]
    MalformedResponse(#[from] DecodingError),
}

impl From<tonic::Status> for FirestoreError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::Ok => FirestoreError::UnknownError,
            tonic::Code::Cancelled => FirestoreError::Cancelled,
            tonic::Code::Unknown => FirestoreError::UnknownError,
            tonic::Code::InvalidArgument => {
                // TODO: Would be good to parse out the details on this one.
                // See https://github.com/googleapis/googleapis/blob/master/google/rpc/error_details.proto for details
                FirestoreError::InvalidArgument(status.message().to_string())
            }
            tonic::Code::DeadlineExceeded => FirestoreError::DeadlineExceeded,
            tonic::Code::NotFound => FirestoreError::NotFound,
            tonic::Code::AlreadyExists => FirestoreError::AlreadyExists,
            tonic::Code::PermissionDenied => {
                // TODO: might be good to get the details on this one too.
                FirestoreError::PermissionDenied(status.message().to_string())
            }
            tonic::Code::ResourceExhausted => {
                FirestoreError::ResourceExhausted(status.message().to_string())
            }
            tonic::Code::FailedPrecondition => {
                FirestoreError::FailedPrecondition(status.message().to_string())
            }
            tonic::Code::Aborted => FirestoreError::Aborted(status.message().to_string()),
            tonic::Code::OutOfRange => FirestoreError::OutOfRange(status.message().to_string()),
            tonic::Code::Unimplemented => FirestoreError::Unimplemented,
            tonic::Code::Internal => FirestoreError::Internal,
            tonic::Code::Unavailable => FirestoreError::Unavailable,
            tonic::Code::DataLoss => FirestoreError::DataLoss,
            tonic::Code::Unauthenticated => {
                // TODO: Details would be good on this
                FirestoreError::Unauthenticated(status.message().to_string())
            }
        }
    }
}

// TODO: Ok, so sometimes there's a google.rpc.ErrorInfo embedded in details
