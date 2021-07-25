use std::{marker::PhantomData, task::Poll};

use futures_util::future::BoxFuture;

use super::{IntoRequest, OperationError};
use crate::{
    document::{Document, DocumentResponse},
    executors::ReadExecutor,
    google::firestore::v1::{self as firestore, write::Operation},
    paths::CollectionPath,
    paths::ProjectPath,
    values::{DecodingError, DocumentValues, EncodingError},
    FirestoreError,
};

impl crate::CollectionRef {
    pub fn list_documents<T>(&self) -> ListDocumentsOperation<T>
    where
        T: Document,
    {
        ListDocumentsOperation::new(self.path.clone())
    }
}

#[derive(Debug)]
pub struct ListDocumentsOperation<T> {
    collection_path: CollectionPath,
    page_size: Option<i32>,
    page_token: Option<String>,
    max_results: Option<i32>,
    phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for ListDocumentsOperation<T> {
    fn clone(&self) -> Self {
        ListDocumentsOperation {
            collection_path: self.collection_path.clone(),
            page_size: self.page_size.clone(),
            page_token: self.page_token.clone(),
            max_results: self.max_results.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> ListDocumentsOperation<T>
where
    T: Document,
{
    fn new(collection_path: CollectionPath) -> Self {
        Self {
            collection_path,
            page_size: None,
            page_token: None,
            max_results: None,
            phantom: PhantomData,
        }
    }

    fn page_size(self, page_size: i32) -> Self {
        Self {
            page_size: Some(page_size),
            ..self
        }
    }

    fn page_token(self, page_token: String) -> Self {
        Self {
            page_token: Some(page_token),
            ..self
        }
    }

    fn maximum_results(self, max_results: i32) -> Self {
        Self {
            max_results: Some(max_results),
            ..self
        }
    }

    pub async fn fetch_page<E>(
        self,
        executor: E,
    ) -> Result<ListDocumentsResponse<T>, OperationError>
    where
        E: ReadExecutor,
    {
        let response = executor.list_documents(self.into_request()?).await?;

        Ok(ListDocumentsResponse {
            next_page_token: response.next_page_token,
            documents: response
                .documents
                .into_iter()
                .map(|r| {
                    r.and_then(|d| {
                        Ok(DocumentResponse {
                            name: d.name,
                            document: T::from_values(d.document)?,
                        })
                    })
                })
                .collect::<Vec<_>>(),
        })
    }

    pub fn stream_pages<E>(self, executor: &'_ E) -> ListDocumentsPageStream<'_, T, E>
    where
        E: ReadExecutor,
    {
        ListDocumentsPageStream {
            executor: &executor,
            future: None,
            op: self,
            state: PagingState::Unstarted,
            phantom: PhantomData,
        }
    }
}

enum PagingState {
    Unstarted,
    WaitingForNextFetch(String),
    FetchingPage,
    Finished,
}

pub struct ListDocumentsResponse<T> {
    pub next_page_token: Option<String>,
    pub documents: Vec<Result<DocumentResponse<T>, DecodingError>>,
}

#[pin_project::pin_project]
pub struct ListDocumentsPageStream<'a, T, E>
where
    E: ReadExecutor,
{
    executor: &'a E,
    future: Option<BoxFuture<'a, Result<ListDocumentsResponse<T>, OperationError>>>,
    op: ListDocumentsOperation<T>,
    state: PagingState,
    phantom: PhantomData<&'a E>,
}

impl<'a, T, E> futures_core::stream::Stream for ListDocumentsPageStream<'a, T, E>
where
    T: Document + 'a,
    E: ReadExecutor,
{
    type Item = Result<Vec<DocumentResponse<T>>, OperationError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match &this.state {
            PagingState::Unstarted => {
                *this.future = Some(Box::pin(this.op.clone().fetch_page(*this.executor)));
                *this.state = PagingState::FetchingPage;
            }
            PagingState::WaitingForNextFetch(next_page_token) => {
                let op = this.op.clone().page_token(next_page_token.clone());
                *this.future = Some(Box::pin(op.fetch_page(*this.executor)));
                *this.state = PagingState::FetchingPage;
            }
            PagingState::FetchingPage => {}
            PagingState::Finished => return Poll::Ready(None),
        }

        match this.future.as_mut().unwrap().as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                *this.future = None;
                if let Err(e) = result {
                    *this.state = PagingState::Finished;
                    return Poll::Ready(Some(Err(e)));
                }
                let page = result.unwrap();

                if let Some(next_token) = page.next_page_token {
                    *this.state = PagingState::WaitingForNextFetch(next_token);
                } else {
                    *this.state = PagingState::Finished;
                }

                let result = page
                    .documents
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(OperationError::from);

                Poll::Ready(Some(result))
            }
        }
    }
}

impl<T> IntoRequest for ListDocumentsOperation<T> {
    type Request = ListDocumentsRequest;

    fn into_request(self) -> Result<Self::Request, OperationError> {
        Ok(ListDocumentsRequest {
            collection_path: self.collection_path,
            page_size: self.page_size.unwrap_or_default(),
            page_token: self.page_token.unwrap_or_default(),
        })
    }
}

pub struct ListDocumentsRequest {
    collection_path: CollectionPath,
    page_size: i32,
    page_token: String,
}

impl ListDocumentsRequest {
    pub(crate) fn into_firestore_request(
        self,
        project_path: ProjectPath,
    ) -> firestore::ListDocumentsRequest {
        let (parent, collection_id) = self.collection_path.parent_and_collection_id(project_path);

        firestore::ListDocumentsRequest {
            parent,
            collection_id,
            page_size: self.page_size,
            page_token: self.page_token,
            order_by: String::new(),
            show_missing: false,
            mask: None,
            consistency_selector: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;

    use super::*;

    use crate::{executors::tests::TestExecutor, values::Value, CollectionRef};

    #[tokio::test]
    async fn test_fetch_page() {
        let executor = TestExecutor::default().list_documents_result(Ok(ListDocumentsResponse {
            next_page_token: None,
            documents: vec![],
        }));

        let page = CollectionRef::new("hello")
            .list_documents::<DocumentValues>()
            .fetch_page(&executor)
            .await
            .unwrap();

        assert_eq!(page.next_page_token, None);
        assert_eq!(page.documents, vec![]);
    }

    #[tokio::test]
    async fn test_stream_pages() {
        let executor = TestExecutor::default().list_documents_results(vec![
            Ok(ListDocumentsResponse {
                next_page_token: Some("next_page".into()),
                documents: vec![Ok(DocumentResponse {
                    name: "doc 1".into(),
                    document: DocumentValues::from_hashmap(maplit::hashmap! {
                        "Hello".to_string() => Value::Null
                    }),
                })],
            }),
            Ok(ListDocumentsResponse {
                next_page_token: Some("next_page_2".into()),
                documents: vec![Ok(DocumentResponse {
                    name: "doc 2".into(),
                    document: DocumentValues::from_hashmap(maplit::hashmap! {
                        "Hello 2".to_string() => Value::Null
                    }),
                })],
            }),
            Ok(ListDocumentsResponse {
                next_page_token: None,
                documents: vec![Ok(DocumentResponse {
                    name: "doc 3".into(),
                    document: DocumentValues::from_hashmap(maplit::hashmap! {
                        "Hello 3".to_string() => Value::Null
                    }),
                })],
            }),
        ]);

        let docs = CollectionRef::new("hello")
            .list_documents::<DocumentValues>()
            .stream_pages(&executor)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        assert_eq!(docs[0].name, "doc 1");
        assert_eq!(docs[1].name, "doc 2");
        assert_eq!(docs[2].name, "doc 3");
    }

    #[tokio::test]
    async fn stream_pages_takes_boxed_executor() {
        let executor: Box<dyn ReadExecutor> = Box::new(
            TestExecutor::default().list_documents_result(Ok(ListDocumentsResponse {
                next_page_token: None,
                documents: vec![],
            })),
        );

        let docs = CollectionRef::new("hello")
            .list_documents::<DocumentValues>()
            .stream_pages(&executor.as_ref())
            .collect::<Vec<_>>()
            .await;

        assert_eq!(docs, vec![]);
    }
}
