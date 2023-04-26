use apache_avro::{schema_compatibility::SchemaCompatibility, Schema};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
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
    // [previous schemas..., old1, old2, old3, newest schema]
    pub fn validate(&self, schemas: &[Schema]) -> bool {
        let schemas: Vec<&Schema> = schemas.iter().rev().collect();
        // [newest schema, old3, old2, old1, previous schemas]
        match self.0 {
            DegaussCompatMode::Backward => {
                // Backward compatibility: A new schema is backward compatible if it can be used to read the data
                // written in the previous schema.
                SchemaCompatibility::can_read(&schemas[1], &schemas[0])
            }
            DegaussCompatMode::Forward => {
                // Forward compatibility: A new schema is forward compatible if the previous schema can read data written in this
                // schema.
                SchemaCompatibility::can_read(&schemas[0], &schemas[1])
            }
            DegaussCompatMode::Full => {
                // Both vice versa
                // Full compatibility: A new schema is fully compatible if it’s both backward and forward compatible.
                SchemaCompatibility::mutual_read(&schemas[0], &schemas[1])
            }
            DegaussCompatMode::BackwardTransitive => {
                // Backward transitive compatibility: A new schema is backward compatible if it can be used to read the data
                // written in all previous schemas.
                let mut x = VecDeque::from(schemas);
                // [newest schema, old3, old2, old1, previous schemas]
                match x.pop_front() {
                    Some(s) => x.iter().all(|e| SchemaCompatibility::can_read(e, &s)),
                    _ => false,
                }
            }
            DegaussCompatMode::ForwardTransitive => {
                // Forward transitive compatibility: A new schema is forward compatible if all previous schemas can read data written
                // in this schema.
                let mut x = VecDeque::from(schemas);
                // [newest schema, old3, old2, old1, previous schemas]
                match x.pop_front() {
                    Some(s) => x.iter().all(|e| SchemaCompatibility::can_read(&s, e)),
                    _ => false,
                }
            }
            DegaussCompatMode::FullTransitive => {
                // Full transitive compatibility: A new schema is fully compatible if it’s both transitively backward
                // and transitively forward compatible with the entire schema history.
                // [newest schema, old3, old2, old1, previous schemas]
                let mut x = VecDeque::from(schemas);
                // [newest schema, old3, old2, old1, previous schemas]
                match x.pop_front() {
                    Some(s) => x.iter().all(|e| SchemaCompatibility::mutual_read(&s, e)),
                    _ => false,
                }
            }
        }
    }

    pub fn tabular_validate(&self, schemas: &[Schema]) -> HashMap<DegaussCompatMode, bool> {
        [(self.0, self.validate(schemas))].iter().cloned().collect()
    }
}
