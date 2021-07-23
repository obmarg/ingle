/// A firebase database, collection or document path

#[derive(Clone, Debug)]
pub struct CollectionPath {
    parent: Option<String>,
    id: String,
}

impl CollectionPath {
    pub fn new(id: String) -> CollectionPath {
        CollectionPath { parent: None, id }
    }

    pub fn document(&self, id: String) -> DocumentPath {
        let mut path = String::with_capacity(
            self.parent.as_ref().map(String::len).unwrap_or_default()
                + self.id.len()
                + id.len()
                + 2,
        );

        if let Some(parent) = &self.parent {
            path.push_str(&parent);
            path.push('/');
        }
        path.push_str(&self.id);
        path.push('/');
        path.push_str(&id);

        DocumentPath { path }
    }
}

#[derive(Clone, Debug)]
pub struct DocumentPath {
    path: String,
}

impl DocumentPath {
    pub fn collection(&self, id: String) -> CollectionPath {
        CollectionPath {
            parent: Some(self.path.clone()),
            id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_document_path() {
        insta::assert_snapshot!(
            CollectionPath::new("books".to_string())
                .document("Northern Lights".to_string())
                .collection("characters".into())
                .document("Lyra Belacqua".into())
                .collection("talents".into())
                .document("reading the alethiometer".into()).path,
            @"books/Northern Lights/characters/Lyra Belacqua/talents/reading the alethiometer"
        )
    }

    #[test]
    fn test_nested_collection_path() {
        insta::assert_debug_snapshot!(
            CollectionPath::new("books".to_string())
                .document("Northern Lights".to_string())
                .collection("characters".into())
                .document("Lyra Belacqua".into())
                .collection("talents".into()),
            @r###"
        CollectionPath {
            parent: Some(
                "books/Northern Lights/characters/Lyra Belacqua",
            ),
            id: "talents",
        }
        "###
        )
    }
}
