use ingle::{
    values::{DocumentValues, Value},
    CollectionRef, DatabaseBuilder,
};

#[tokio::test]
async fn test_adding_and_listing_documents() {
    let database = DatabaseBuilder::new("nandos-api-platform")
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
