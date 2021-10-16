use thiserror::Error;

#[derive(Error, Debug)]
pub enum DegaussError {
    #[error("Schema read failed")]
    SchemaRead,
    #[error("Schema compatibility error")]
    SchemaCompat,
}