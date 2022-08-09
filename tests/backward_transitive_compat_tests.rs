/// Backward transitive compatibility: A new schema is backward compatible if it can be used to read the data written
/// in all previous schemas.
#[cfg(test)]
mod backward_transitive_compat {

    use apache_avro::Schema;
    use degauss::prelude::*;

    #[test]
    fn iteratively_adding_fields_with_defaults_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema8.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::BackwardTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_a_field_with_default_is_a_backward_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::BackwardTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_default_is_a_compatible_change_but_not_transitively() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::BackwardTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_default_is_not_a_transitively_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::BackwardTransitive);
        assert_eq!(dc.validate(&schemas), false);
    }
}
