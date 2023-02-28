use ingle::{
    transactions::{ReadOnlyExecutor, ReadPhaseExecutor},
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

    let added_doc = collection
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
            // TODO: Ideally do a non transactional write here to make sure it doesn't
            // show in the list
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

#[tokio::test]
async fn test_read_write_transactions() {
    let database = DatabaseBuilder::new(std::env::var("GOOGLE_PROJECT").unwrap())
        .auth_token(std::env::var("GOOGLE_TOKEN").unwrap())
        .connect()
        .await
        .unwrap();

    println!("Connected");

    database
        .transaction()
        .read_write()
        .run(|tx: ReadPhaseExecutor| async move {
            println!("In transaction");
            let collection = CollectionRef::new("books");

            let documents = collection
                .list_documents::<DocumentValues>()
                .fetch_all(&tx)
                .await
                .unwrap();

            println!("Got documents: {:?}", documents);

            //assert!(!documents.is_empty());

            let tx = tx.finish_reads();

            let document = DocumentValues::from_hashmap(maplit::hashmap! {
                "Test".to_string() => Value::Boolean(true)
            });

            println!("Adding Document");
            collection.add_document(&document).run_in(&tx).await;
            println!("Added Document");

            // TODO: Ideally cause clashes here somehow (maybe w/ non-transactional writes)
        })
        .await
        .unwrap();

    println!("Done transaction");
}
