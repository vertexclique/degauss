/// Backward compatibility: A new schema is backward compatible if it can be used to read the data written in the
/// previous schema.
#[cfg(test)]
mod forward_transitive_compat {
    use std::{path::PathBuf, str::FromStr};

    use avro_rs::Schema;
    use degauss::prelude::*;

    #[test]
    fn iteratively_removing_fields_with_defaults_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file(&PathBuf::from_str("tests/data/schema8.avsc").unwrap()).unwrap(),
            Schema::parse_file(&PathBuf::from_str("tests/data/schema2.avsc").unwrap()).unwrap(),
            Schema::parse_file(&PathBuf::from_str("tests/data/schema1.avsc").unwrap()).unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::ForwardsTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_default_to_a_field_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file(&PathBuf::from_str("tests/data/schema2.avsc").unwrap()).unwrap(),
            Schema::parse_file(&PathBuf::from_str("tests/data/schema3.avsc").unwrap()).unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::ForwardsTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_field_with_a_default_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file(&PathBuf::from_str("tests/data/schema2.avsc").unwrap()).unwrap(),
            Schema::parse_file(&PathBuf::from_str("tests/data/schema1.avsc").unwrap()).unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::ForwardsTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_default_is_not_a_transitively_compatible_change() {
        let schemas = vec![
            Schema::parse_file(&PathBuf::from_str("tests/data/schema1.avsc").unwrap()).unwrap(),
            Schema::parse_file(&PathBuf::from_str("tests/data/schema3.avsc").unwrap()).unwrap(),
            Schema::parse_file(&PathBuf::from_str("tests/data/schema2.avsc").unwrap()).unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::ForwardsTransitive);
        assert_eq!(dc.validate(&schemas), false);
    }
}