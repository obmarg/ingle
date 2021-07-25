use std::future::Future;
use std::time::SystemTime;

use async_trait::async_trait;

use crate::{google::firestore::v1 as firestore, Database, FirestoreError};

mod executors;

pub use executors::{ReadOnlyExecutor, ReadPhaseExecutor};

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
    type Ok;
    type Err;

    async fn run(&self, executor: ReadPhaseExecutor) -> Result<Self::Ok, Self::Err>;
}

#[async_trait]
pub trait ReadOnlyTransaction {
    type Result;

    async fn run(&self, executor: ReadOnlyExecutor) -> Self::Result;
}

#[async_trait]
impl<Func, Fut, Res> ReadOnlyTransaction for Func
where
    for<'a> &'a Func: Send,
    Func: (Fn(ReadOnlyExecutor) -> Fut) + Send + 'static,
    Fut: Future<Output = Res> + Send,
{
    type Result = Res;

    async fn run(&self, executor: ReadOnlyExecutor) -> Res {
        self(executor).await
    }
}

#[derive(Debug, Default)]
pub struct ReadOnly {
    read_time: Option<SystemTime>,
}

#[derive(Debug, Default)]
pub struct ReadWrite {
    max_retries: Option<u8>,
}

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

    pub fn run<T>(self, transaction: T) -> Result<T::Ok, T::Err>
    where
        T: Transaction,
    {
        todo!()
    }
}

impl TransactionBuilder<ReadOnly> {
    pub async fn run<T>(self, transaction: T) -> Result<T::Result, TransactionError<()>>
    where
        T: ReadOnlyTransaction,
    {
        let mut client = self.database.client.clone();
        let response = client
            .begin_transaction(firestore::BeginTransactionRequest {
                database: self.database.project_path.database_path().to_string(),
                options: Some(firestore::TransactionOptions {
                    mode: Some(firestore::transaction_options::Mode::ReadOnly(
                        firestore::transaction_options::ReadOnly {
                            consistency_selector: None,
                        },
                    )),
                }),
            })
            .await
            .map_err(FirestoreError::from)
            .map_err(TransactionError::CouldNotStartTransaction)?
            .into_inner();

        let transaction_id = response.transaction.clone();

        let executor = ReadOnlyExecutor {
            database: self.database.clone(),
            transaction_id: response.transaction,
        };

        let result = transaction.run(executor).await;

        client
            .commit(firestore::CommitRequest {
                database: self.database.project_path.database_path().to_string(),
                transaction: transaction_id,
                writes: Vec::new(),
            })
            .await
            .map_err(FirestoreError::from)
            .map_err(TransactionError::CouldNotFinishTransaction)?;

        Ok(result)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TransactionError<E>
where
    E: std::fmt::Debug,
{
    #[error("Could not start transaction: {0}")]
    CouldNotStartTransaction(FirestoreError),
    #[error("Error while running transaction")]
    InnerError(E),
    #[error("Could not finish transaction: {0}")]
    CouldNotFinishTransaction(FirestoreError),
}

impl<T> TransactionBuilder<T> where T: Transaction {}
