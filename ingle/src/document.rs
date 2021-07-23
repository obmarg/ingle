use crate::values::DocumentValues;

pub trait Document: Sized {
    fn to_values(&self) -> Result<DocumentValues, ()>;

    fn from_values(values: DocumentValues) -> Result<Self, ()>;
}

impl Document for DocumentValues {
    fn to_values(&self) -> Result<DocumentValues, ()> {
        Ok(self.clone())
    }

    fn from_values(values: DocumentValues) -> Result<Self, ()> {
        Ok(values)
    }
}
