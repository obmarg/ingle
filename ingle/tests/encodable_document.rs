use maplit::hashmap;

use ingle::{values::Value, EncodableDocument};

#[test]
fn test_simple_document_encoder() {
    #[derive(EncodableDocument)]
    struct MyDocument<'a> {
        data: bool,
        other_data: &'a bool,
    }

    let encoded = MyDocument {
        data: true,
        other_data: &false,
    }
    .encode()
    .unwrap();

    assert_eq!(
        encoded.into_hashmap(),
        hashmap! {
            "data".to_string() => Value::Boolean(true),
            "other_data".to_string() => Value::Boolean(false)
        }
    )
}

#[test]
fn test_nested_documents() {
    #[derive(EncodableDocument)]
    struct Parent {
        child: Child,
    }

    #[derive(EncodableDocument)]
    struct Child {
        data: bool,
    }

    let encoded = Parent {
        child: Child { data: true },
    }
    .encode()
    .unwrap();

    assert_eq!(
        encoded.into_hashmap(),
        hashmap! {
            "child".to_string() => Value::Map(
                hashmap! { "data".to_string() => Value::Boolean(true) }
            )
        }
    )
}
