use crate::paths::{CollectionPath, DocumentPath};

pub struct CollectionRef {
    pub(crate) path: CollectionPath,
}

impl CollectionRef {
    pub fn new(collection_id: impl Into<String>) -> CollectionRef {
        CollectionRef {
            path: CollectionPath::new(collection_id.into()),
        }
    }

    pub fn document(&self, id: impl Into<String>) -> DocumentRef {
        DocumentRef {
            path: self.path.document(id.into()),
        }
    }
}

pub struct DocumentRef {
    path: DocumentPath,
}

impl DocumentRef {
    pub fn sub_collection(&self, id: impl Into<String>) -> CollectionRef {
        CollectionRef {
            path: self.path.collection(id.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use crate::values::{DocumentValues, Value};

    use super::CollectionRef;

    #[test]
    fn test_add_document() {
        let document = DocumentValues::from_hashmap(hashmap! {
            "Hello".to_string() => Value::Null
        });

        let op = CollectionRef::new("books").add_document(&document);

        insta::assert_debug_snapshot!(op, @r###"
        AddDocumentOperation {
            collection_path: CollectionPath {
                parent: None,
                id: "books",
            },
            document_id: None,
            document: Ok(
                DocumentValues(
                    {
                        "Hello": Null,
                    },
                ),
            ),
            t: PhantomData,
        }
        "###)
    }

    #[test]
    fn add_nested_document() {
        let document = DocumentValues::from_hashmap(hashmap! {
            "Name".to_string() => Value::String("Lyra Belacqua".to_string())
        });

        let op = CollectionRef::new("books")
            .document("Northern Lights")
            .sub_collection("characters")
            .add_document(&document);

        insta::assert_debug_snapshot!(op, @r###"
        AddDocumentOperation {
            collection_path: CollectionPath {
                parent: Some(
                    "books/Northern Lights",
                ),
                id: "characters",
            },
            document_id: None,
            document: Ok(
                DocumentValues(
                    {
                        "Name": String(
                            "Lyra Belacqua",
                        ),
                    },
                ),
            ),
            t: PhantomData,
        }
        "###);
    }
}
