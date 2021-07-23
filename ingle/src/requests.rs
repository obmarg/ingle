use crate::{path::CollectionPath, values::DocumentValues};


pub struct DocumentResponse<D> {
    pub name: String,
    pub document: D,
}
