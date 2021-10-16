use avro_rs::{Schema, schema_compatibility::SchemaCompatibility};

/// Chronological order of satisfaction of check for schemas
pub enum DegaussChronologyType {
    Latest,
    All
}

/// Check level types between schemas
pub enum DegaussCheckType {
    CanRead,
    CanBeReadBy,
    MutualRead
}

///
/// Possible compatiblity mode array between schemas
pub enum DegaussCompatMode {
    /// Also known as 'backwards'. Can read the data written by the most recent previous schema.
    CanReadLatest,
    /// Also known as 'backwards transitive'. Can read the data written by all earlier schemas.
    CanReadAll,
    /// Also known as 'forwards'. The data written by this schema can be read by the most recent previous schema.    
    CanBeReadByLatest,
    /// Also known as 'forwards transitive'. The data written by this schema can be read by all earlier schemas.
    CanBeReadByAll,
    /// Also known as 'full'. Can read the data written by, a write data readable by the most recent previous schema.
    MutualReadWithLatest,
    /// Also known as 'full transitive'. Can read the data written by, a write data readable by all earlier schemas.
    MutualReadWithAll
}


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
    fn validate(&self, validate: &Schema, existing: &Schema) -> bool {
        match self.0 {
            DegaussCheckType::CanRead => SchemaCompatibility::can_read(existing, validate),
            DegaussCheckType::CanBeReadBy => SchemaCompatibility::can_read(validate, existing),
            DegaussCheckType::MutualRead => SchemaCompatibility::mutual_read(validate, existing),
            _ => false
        }
    }
}