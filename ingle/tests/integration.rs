use ingle::{
    transactions::ReadOnlyExecutor,
    values::{DocumentValues, Value},
    CollectionRef, DatabaseBuilder,
};

#[tokio::test]
async fn test_adding_and_listing_documents() {
    let database = DatabaseBuilder::new(std::env::var("GOOGLE_PROJECT").unwrap())
        .auth_token(std::env::var("GOOGLE_TOKEN").unwrap())
        .connect()
        .await
        .unwrap();

    let document = DocumentValues::from_hashmap(maplit::hashmap! {
        "Test".to_string() => Value::Boolean(true)
    });

    let collection = CollectionRef::new("books");

    collection
        .add_document(&document)
        .run(&database)
        .await
        .unwrap();

    let documents = collection
        .list_documents::<DocumentValues>()
        .fetch_all(&database)
        .await
        .unwrap();

    assert!(!documents.is_empty());
}

#[tokio::test]
async fn test_read_only_transactions() {
    let database = DatabaseBuilder::new(std::env::var("GOOGLE_PROJECT").unwrap())
        .auth_token(std::env::var("GOOGLE_TOKEN").unwrap())
        .connect()
        .await
        .unwrap();

    database
        .transaction()
        .read_only()
        .run(|tx: ReadOnlyExecutor| async move {
            let collection = CollectionRef::new("books");

            let documents = collection
                .list_documents::<DocumentValues>()
                .fetch_all(&tx)
                .await
                .unwrap();

            assert!(!documents.is_empty());
        })
        .await
        .unwrap();
}
