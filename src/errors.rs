use thiserror::Error;

#[derive(Error, Debug)]
pub enum DegaussError {
    #[error("File read error")]
    IO(#[from] std::io::Error),

    #[error("Schema parsing error")]
    Schema(#[from] avro_rs::Error),
}
