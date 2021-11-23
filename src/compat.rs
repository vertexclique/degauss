use avro_rs::{schema_compatibility::SchemaCompatibility, Schema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumIter, EnumString, EnumVariantNames};

///
/// Possible compatibility mode array between schemas
#[derive(
    EnumIter,
    EnumVariantNames,
    EnumString,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Serialize,
    Deserialize,
)]
pub enum DegaussCompatMode {
    /// Can read the data written by the most recent previous schema.
    #[strum(serialize = "backward")]
    #[serde(rename(deserialize = "backward", deserialize = "BACKWARD"))]
    Backward,

    /// Can read the data written by all earlier schemas.
    #[strum(serialize = "backward_transitive")]
    #[serde(rename(
        deserialize = "backward-transitive",
        deserialize = "backward_transitive",
        deserialize = "BACKWARD_TRANSITIVE"
    ))]
    BackwardTransitive,

    /// The data written by this schema can be read by the most recent previous schema.  
    #[strum(serialize = "forward")]
    #[serde(rename(deserialize = "forward", deserialize = "FORWARD"))]
    Forward,

    /// The data written by this schema can be read by all earlier schemas.
    #[strum(serialize = "forward_transitive")]
    #[serde(rename(
        deserialize = "forward-transitive",
        deserialize = "forward_transitive",
        deserialize = "FORWARD_TRANSITIVE"
    ))]
    ForwardTransitive,

    /// Can read the data written by, a write data readable by the most recent previous schema.
    #[strum(serialize = "full")]
    #[serde(rename(deserialize = "full", deserialize = "FULL"))]
    Full,

    /// Can read the data written by, a write data readable by all earlier schemas.
    #[strum(serialize = "full_transitive")]
    #[serde(rename(
        deserialize = "full-transitive",
        deserialize = "full_transitive",
        deserialize = "FULL_TRANSITIVE"
    ))]
    FullTransitive,
}

// /// Also known as 'backwards'. Can read the data written by the most recent previous schema.
// CanReadLatest,
// /// Also known as 'backwards transitive'. Can read the data written by all earlier schemas.
// CanReadAll,
// /// Also known as 'forwards'. The data written by this schema can be read by the most recent previous schema.
// CanBeReadByLatest,
// /// Also known as 'forwards transitive'. The data written by this schema can be read by all earlier schemas.
// CanBeReadByAll,
// /// Also known as 'full'. Can read the data written by, a write data readable by the most recent previous schema.
// MutualReadWithLatest,
// /// Also known as 'full transitive'. Can read the data written by, a write data readable by all earlier schemas.
// MutualReadWithAll,

// /** Also known as 'backwards'. Can read the data written by the most recent previous schema. */
// CAN_READ_LATEST(ChronologyType.LATEST, CheckType.CAN_READ),
// /** Also known as 'backwards transitive'. Can read the data written by all earlier schemas. */
// CAN_READ_ALL(ChronologyType.ALL, CheckType.CAN_READ),
// /** Also known as 'forwards'. The data written by this schema can be read by the most recent previous schema. */
// CAN_BE_READ_BY_LATEST(ChronologyType.LATEST, CheckType.CAN_BE_READ_BY),
// /** Also known as 'forwards transitive'. The data written by this schema can be read by all earlier schemas. */
// CAN_BE_READ_BY_ALL(ChronologyType.ALL, CheckType.CAN_BE_READ_BY),
// /** Also known as 'full'. Can read the data written by, a write data readable by the most recent previous schema. */
// MUTUAL_READ_WITH_LATEST(ChronologyType.LATEST, CheckType.MUTUAL_READ),
// /** Also known as 'full transitive'. Can read the data written by, a write data readable by all earlier schemas. */
// MUTUAL_READ_WITH_ALL(ChronologyType.ALL, CheckType.MUTUAL_READ);

#[derive(Clone, Debug)]
pub struct DegaussCheck(pub DegaussCompatMode);

impl DegaussCheck {
    ///
    /// Validate given list of the schemas with the compat mode
    pub fn validate(&self, schemas: &[Schema]) -> bool {
        match self.0 {
            DegaussCompatMode::Backward => {
                // First existing schema, second validating schema
                SchemaCompatibility::can_read(&schemas[1], &schemas[0])
            }
            DegaussCompatMode::Forward => {
                // First validating schema, second existing schema
                SchemaCompatibility::can_read(&schemas[0], &schemas[1])
            }
            DegaussCompatMode::Full => {
                // Both vice versa
                SchemaCompatibility::mutual_read(&schemas[0], &schemas[1])
            }
            DegaussCompatMode::BackwardTransitive => {
                let mut x = schemas.to_vec();
                x.reverse();
                x.windows(2)
                    .all(|c| SchemaCompatibility::can_read(&c[1], &c[0]))
            }
            DegaussCompatMode::ForwardTransitive => {
                let mut x = schemas.to_vec();
                x.reverse();
                schemas
                    .windows(2)
                    .all(|c| SchemaCompatibility::can_read(&c[0], &c[1]))
            }
            DegaussCompatMode::FullTransitive => {
                let mut x = schemas.to_vec();
                x.reverse();
                schemas
                    .windows(2)
                    .all(|c| SchemaCompatibility::mutual_read(&c[0], &c[1]))
            }
        }
    }

    pub fn tabular_validate(&self, schemas: &[Schema]) -> HashMap<DegaussCompatMode, bool> {
        [(self.0, self.validate(schemas))].iter().cloned().collect()
    }
}
