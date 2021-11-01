use crate::errors::*;
use avro_rs::Schema;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Parse a given file and convert it to Schema object
pub trait FromFile {
    /// Parses a given file path into a a valid Schema object
    fn parse_file<P: AsRef<Path>>(path: P) -> Result<Schema, DegaussError>;
}

/// Implements the FromFile trait for reading Schema from a given file
impl FromFile for Schema {
    /// Parses a given file into a a valid Schema object
    fn parse_file<P: AsRef<Path>>(path: P) -> Result<Schema, DegaussError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let schema = Schema::parse_str(&contents)?;
        Ok(schema)
    }
}
