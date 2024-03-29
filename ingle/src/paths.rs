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
        }
        path.push('/');
        path.push_str(&self.id);
        path.push('/');
        path.push_str(&id);

        DocumentPath { path }
    }

    pub fn parent_and_collection_id(self, project_path: ProjectPath) -> (String, String) {
        let documents_part = "/documents";
        let parent_len = self.parent.as_ref().map(String::len).unwrap_or_default();
        let mut parent =
            String::with_capacity(parent_len + project_path.path.len() + documents_part.len());
        parent.push_str(&project_path.path);
        parent.push_str(documents_part);
        if let Some(current_parent) = self.parent {
            parent.push_str(&current_parent);
        }

        (parent, self.id)
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

    pub(crate) fn full_path(self, project_path: ProjectPath) -> String {
        let documents_part = "/documents";

        let mut document_id =
            String::with_capacity(project_path.path.len() + documents_part.len() + self.path.len());
        document_id.push_str(&project_path.path);
        document_id.push_str(documents_part);
        document_id.push_str(&self.path);

        document_id
    }
}

#[derive(Clone)]
pub struct ProjectPath {
    path: String,
}

impl ProjectPath {
    pub fn new(project_id: String, database_id: String) -> ProjectPath {
        ProjectPath {
            path: format!("projects/{}/databases/{}", project_id, database_id),
        }
    }

    pub(crate) fn database_path(&self) -> &str {
        &self.path
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
            @"/books/Northern Lights/characters/Lyra Belacqua/talents/reading the alethiometer"
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
                "/books/Northern Lights/characters/Lyra Belacqua",
            ),
            id: "talents",
        }
        "###
        )
    }
}
