use maplit::hashmap;

use ingle::{values::Value, EncodableDocument};

#[test]
fn test_simple_document_encoder() {
    #[derive(EncodableDocument)]
    struct MyDocument {
        data: bool,
        other_data: bool,
    }

    let encoded = MyDocument {
        data: true,
        other_data: false,
    }
    .encode(ingle::values::encode::DocumentEncoder {})
    .unwrap();

    assert_eq!(
        encoded.into_hashmap(),
        hashmap! {
            "data".to_string() => Value::Boolean(true),
            "other_data".to_string() => Value::Boolean(false)
        }
    )
}
