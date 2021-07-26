use std::future::Future;
use std::time::SystemTime;

use async_trait::async_trait;
use futures_channel::mpsc;
use futures_util::StreamExt;

use crate::{google::firestore::v1 as firestore, paths::ProjectPath, Database, FirestoreError};

mod executors;

pub use executors::{ReadOnlyExecutor, ReadPhaseExecutor};

use self::executors::WriteRequest;

impl super::Database {
    pub fn transaction(&self) -> TransactionBuilder<ReadWrite> {
        TransactionBuilder {
            database: self.clone(),
            mode: ReadWrite::default(),
        }
    }
}

#[async_trait]
pub trait Transaction {
    type Result;

    async fn run(&self, executor: ReadPhaseExecutor) -> Self::Result;
}

#[async_trait]
pub trait ReadOnlyTransaction {
    type Result;

    async fn run(self, executor: ReadOnlyExecutor) -> Self::Result;
}

#[async_trait]
impl<Func, Fut, Res> ReadOnlyTransaction for Func
where
    Func: (FnOnce(ReadOnlyExecutor) -> Fut) + Send + 'static,
    Fut: Future<Output = Res> + Send,
{
    type Result = Res;

    async fn run(self, executor: ReadOnlyExecutor) -> Res {
        self(executor).await
    }
}

#[async_trait]
impl<Func, Fut, Res> Transaction for Func
where
    for<'a> &'a Func: Send,
    Func: (Fn(ReadPhaseExecutor) -> Fut) + Send + 'static,
    Fut: Future<Output = Res> + Send,
{
    type Result = Res;

    async fn run(&self, executor: ReadPhaseExecutor) -> Res {
        self(executor).await
    }
}

#[must_use]
pub struct TransactionBuilder<Mode = ReadWrite> {
    database: Database,
    mode: Mode,
}

impl<Mode> TransactionBuilder<Mode> {
    pub fn read_only(self) -> TransactionBuilder<ReadOnly> {
        TransactionBuilder {
            mode: ReadOnly::default(),
            database: self.database,
        }
    }

    pub fn read_write(self) -> TransactionBuilder<ReadWrite> {
        TransactionBuilder {
            mode: ReadWrite::default(),
            database: self.database,
        }
    }
}

impl TransactionBuilder<ReadWrite> {
    pub fn max_retries(self, retries: u8) -> Self {
        TransactionBuilder {
            mode: ReadWrite {
                max_retries: Some(retries),
            },
            ..self
        }
    }

    pub async fn run<T>(self, transaction: T) -> Result<T::Result, TransactionError>
    where
        T: Transaction,
    {
        let mut client = self.database.client.clone();
        let mut tries = self.mode.tries();
        let mut transaction_id = vec![];
        let mut last_error = None;
        let database_path = self.database.project_path.database_path().to_string();

        while tries > 0 {
            tries -= 1;

            let begin_request = firestore::BeginTransactionRequest {
                database: database_path.clone(),
                options: Some(firestore::TransactionOptions {
                    mode: Some(firestore::transaction_options::Mode::ReadWrite(
                        firestore::transaction_options::ReadWrite {
                            retry_transaction: transaction_id.clone(),
                        },
                    )),
                }),
            };

            let response = begin_transaction(&mut client, begin_request).await?;

            transaction_id = response.transaction.clone();

            let (write_sender, write_receiver) = mpsc::unbounded();

            let executor = ReadPhaseExecutor {
                database: self.database.clone(),
                transaction_id: response.transaction,
                writes: write_sender,
            };

            let result = transaction.run(executor).await;

            let writes = write_receiver.collect::<Vec<_>>().await;

            if writes
                .iter()
                .any(|w| matches!(w, executors::WriteRequest::Rollback))
            {
                client
                    .rollback(firestore::RollbackRequest {
                        database: database_path.clone(),
                        transaction: transaction_id.clone(),
                    })
                    .await
                    .ok();

                last_error = Some(TransactionError::RollbackRequested);
                continue;
            }

            let commit = commit_request(&self.database.project_path, &transaction_id, writes);

            match commit_transaction(&mut client, commit).await {
                Ok(_) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    continue;
                }
            }
        }

        Err(last_error.expect("Transaction failed without error.  Probably a bug in ingle"))
    }
}

impl TransactionBuilder<ReadOnly> {
    pub async fn run<T>(self, transaction: T) -> Result<T::Result, TransactionError>
    where
        T: ReadOnlyTransaction,
    {
        let mut client = self.database.client.clone();
        let request = firestore::BeginTransactionRequest {
            database: self.database.project_path.database_path().to_string(),
            options: Some(firestore::TransactionOptions {
                mode: Some(firestore::transaction_options::Mode::ReadOnly(
                    firestore::transaction_options::ReadOnly {
                        consistency_selector: None,
                    },
                )),
            }),
        };

        let response = begin_transaction(&mut client, request).await?;

        let transaction_id = response.transaction.clone();

        let executor = ReadOnlyExecutor {
            database: self.database.clone(),
            transaction_id: response.transaction,
        };

        let result = transaction.run(executor).await;

        commit_transaction(
            &mut client,
            firestore::CommitRequest {
                database: self.database.project_path.database_path().to_string(),
                transaction: transaction_id,
                writes: Vec::new(),
            },
        )
        .await?;

        Ok(result)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TransactionError {
    #[error("Could not start transaction: {0}")]
    CouldNotStartTransaction(FirestoreError),
    #[error("Could not finish transaction: {0}")]
    CouldNotCommitTransaction(FirestoreError),
    #[error("A rollback was requested in the transaction")]
    RollbackRequested,
}

#[derive(Debug, Default)]
pub struct ReadOnly {
    read_time: Option<SystemTime>,
}

#[derive(Debug, Default)]
pub struct ReadWrite {
    max_retries: Option<u8>,
}

impl ReadWrite {
    fn tries(&self) -> u8 {
        self.max_retries.unwrap_or(5) + 1
    }
}

async fn begin_transaction(
    client: &mut super::FirestoreClient,
    request: firestore::BeginTransactionRequest,
) -> Result<firestore::BeginTransactionResponse, TransactionError> {
    Ok(client
        .begin_transaction(request)
        .await
        .map_err(FirestoreError::from)
        .map_err(TransactionError::CouldNotStartTransaction)?
        .into_inner())
}

async fn commit_transaction(
    client: &mut super::FirestoreClient,
    request: firestore::CommitRequest,
) -> Result<firestore::CommitResponse, TransactionError> {
    Ok(client
        .commit(request)
        .await
        .map_err(FirestoreError::from)
        .map_err(TransactionError::CouldNotCommitTransaction)?
        .into_inner())
}

fn commit_request(
    project_path: &ProjectPath,
    transaction_id: &[u8],
    writes: Vec<WriteRequest>,
) -> firestore::CommitRequest {
    firestore::CommitRequest {
        database: project_path.database_path().to_string(),
        transaction: transaction_id.to_vec(),
        writes: writes
            .into_iter()
            .filter_map(|write| write.into_firestore_write(project_path))
            .collect(),
    }
}
