use ingle::{
    values::{DocumentValues, Value},
    CollectionRef, DatabaseBuilder,
};

#[tokio::test]
async fn test_adding_document() {
    let database = DatabaseBuilder::new("nandos-api-platform")
        .auth_token(std::env::var("GOOGLE_TOKEN").unwrap())
        .connect()
        .await
        .unwrap();

    let document = DocumentValues::from_hashmap(maplit::hashmap! {
        "Test".to_string() => Value::Boolean(true)
    });

    CollectionRef::new("books")
        .add_document(&document)
        .run(&database)
        .await
        .unwrap();
    //let collection = ingle::CollectionRef::new("raw")
}
