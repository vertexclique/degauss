use avro_rs::{schema_compatibility::SchemaCompatibility, Schema};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::{fmt, panic};
use strum::IntoEnumIterator;

use crate::errors::DegaussError;

/// Chronological order of satisfaction of check for schemas
pub enum DegaussChronologyType {
    Latest,
    All,
}

#[derive(Debug, strum_macros::EnumIter, PartialEq, Eq, Hash, strum_macros::Display)]
/// Check level types between schemas
pub enum DegaussCheckType {
    CanRead,
    CanBeReadBy,
    MutualRead,
}

///
/// Possible compatiblity mode array between schemas
pub enum DegaussCompatMode {
    /// Also known as 'backwards'. Can read the data written by the most recent previous schema.
    Backwards,
    /// Also known as 'backwards transitive'. Can read the data written by all earlier schemas.
    BackwardsTransitive,
    /// Also known as 'forwards'. The data written by this schema can be read by the most recent previous schema.    
    Forwards,
    /// Also known as 'forwards transitive'. The data written by this schema can be read by all earlier schemas.
    ForwardsTransitive,
    /// Also known as 'full'. Can read the data written by, a write data readable by the most recent previous schema.
    Full,
    /// Also known as 'full transitive'. Can read the data written by, a write data readable by all earlier schemas.
    FullTransitive,
}

impl fmt::Debug for DegaussCompatMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backwards => write!(f, "Backwards"),
            Self::BackwardsTransitive => write!(f, "BackwardsTransitive"),
            Self::Forwards => write!(f, "Forwards"),
            Self::ForwardsTransitive => write!(f, "ForwardsTransitive"),
            Self::Full => write!(f, "Full"),
            Self::FullTransitive => write!(f, "FullTransitive"),
        }
    }
}

impl TryFrom<&str> for DegaussCompatMode
{
    type Error = DegaussError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s.to_lowercase().as_str() {
            "backwards" => Self::Backwards,
            "backwards-transitive" | "backwardstransitive" => Self::BackwardsTransitive,
            "forwards" => Self::Forwards,
            "forwards-transitive" | "forwardstransitive" => Self::ForwardsTransitive,
            "full" => Self::Full,
            "full-transitive" | "fulltransitive" => Self::FullTransitive,
            _ => return Err(DegaussError::ParseFailure)
        })
    }
}

impl From<&OsStr> for DegaussCompatMode
{
    fn from(s: &OsStr) -> Self {
        s
        .to_owned()
        .into_string()
        .unwrap_or_else(|op| panic!("Failed to decode compatibility: {:#?}", op))
        .as_str()
        .try_into()
        .unwrap()
    }
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

pub struct DegaussCheck(DegaussCheckType);

impl DegaussCheck {
    pub fn validate(&self, validate: &Schema, existing: &Schema) -> bool {
        match self.0 {
            DegaussCheckType::CanRead => SchemaCompatibility::can_read(existing, validate),
            DegaussCheckType::CanBeReadBy => SchemaCompatibility::can_read(validate, existing),
            DegaussCheckType::MutualRead => SchemaCompatibility::mutual_read(validate, existing),
            _ => false,
        }
    }

    pub fn validate_all(validate: &Schema, existing: &Schema) -> HashMap<DegaussCheckType, bool> {
        let mut results: HashMap<DegaussCheckType, bool> = HashMap::with_capacity(3);

        for check in DegaussCheckType::iter() {
            let result = match check {
                DegaussCheckType::CanRead => SchemaCompatibility::can_read(existing, validate),
                DegaussCheckType::CanBeReadBy => SchemaCompatibility::can_read(validate, existing),
                DegaussCheckType::MutualRead => {
                    SchemaCompatibility::mutual_read(validate, existing)
                }
                _ => false,
            };
            results.insert(check, result);
        }
        results
    }
}
