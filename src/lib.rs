pub mod compat;
pub mod errors;
pub mod schema;
pub mod table;

mod schema_registry;
pub use schema_registry::{ResponseExt, SchemaRegistryClient, SerdeExt};
pub mod prelude {
    pub use crate::compat::*;
    pub use crate::errors::*;
    pub use crate::schema::*;
    pub use crate::schema_registry::types::*;
}
