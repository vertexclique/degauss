#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

//!
//!
//! <div align="center">
//! <p><h1>DeGauss</h1> </p>
//! <p><strong>Your friendly neighborhood Avro schema compatibility checker.</strong> </p>
//! <p>
//!
//! [![cicd](https://github.com/vertexclique/degauss/actions/workflows/cicd.yml/badge.svg)](https://github.com/vertexclique/degauss/actions/workflows/cicd.yml)
//! [![Crates.io](https://img.shields.io/crates/v/degauss)](https://crates.io/crates/degauss)
//! [![Docs.rs](https://docs.rs/degauss/badge.svg)](https://docs.rs/degauss)
//! [![codecov](https://codecov.io/gh/vertexclique/degauss/branch/master/graph/badge.svg)](https://codecov.io/gh/vertexclique/degauss)
//! </p>
//! </div>
//! </br>
//!
//!
//! ## Usage
//!
//! `degauss` can be used to check the compatibility of schemas and help with schema evolution.
//!
//! Let's consider a scenario, we have a schema which is registered on kafka.
//! ```bash
//! cat schema_v1.avsc
//!
//! {
//! "type": "record",
//! "name": "movie",
//! "fields": [
//! {
//!         "name": "movie_id",
//!         "type": "long"
//!     },
//!     {
//!         "name": "title",
//!         "type": "string"
//!     },
//!     {
//!         "name": "release_year",
//!         "type": "long"
//!     }
//! ]
//! }
//! ```
//! Another user wants to evolve this schema but wants to make sure if the new schema adheres to the compatibility
//! guarantees for the given topic. The resulting v2 of this schema could look like:
//!
//!```bash
//! cat schema_v2.avsc
//!
//! {
//! "type": "record",
//! "name": "movie",
//! "fields": [
//! {
//!         "name": "movie_id",
//!         "type": "long"
//!     },
//!     {
//!         "name": "title",
//!         "type": "string"
//!     }
//! ]
//! }
//! ```
//!
//! The person can simply use degauss to check this.
//! ```bash
//! degauss -s older_schema.avsc current_schema.avsc --compat forward # or choose from several compatibilities
//! ```

pub mod compat;
pub mod errors;
pub mod schema;
pub mod table;

pub mod schema_registry;

pub mod prelude {
    pub use crate::compat::*;
    pub use crate::errors::*;
    pub use crate::schema::*;
    pub use crate::schema_registry::types::*;
    pub use crate::schema_registry::*;
}
