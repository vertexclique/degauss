use thiserror::Error;

#[derive(Error, Debug)]
pub enum DegaussError {
    #[error("Schema compatibility error")]
    SchemaCompat,

    #[error("Parse failure")]
    ParseFailure,

    #[error("File read error")]
    IO(#[from] std::io::Error),

    #[error("Schema parsing error")]
    Schema(#[from] avro_rs::Error),
}
