/// Full transitive compatibility: A new schema is fully compatible if it’s both transitively backward and transitively
/// forward compatible with the entire schema history.
#[cfg(test)]
mod full_transitive_compat {

    use avro_rs::Schema;
    use degauss::prelude::*;

    #[test]
    fn iteratively_adding_fields_with_defaults_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema8.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn iteratively_removing_fields_with_defaults_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema8.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_default_to_a_field_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_field_with_a_default_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_a_field_with_default_is_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_default_from_a_field_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn transitively_adding_a_field_without_a_default_is_not_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), false);
    }

    #[test]
    fn transitively_removing_a_field_without_a_default_is_not_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::FullTransitive);
        assert_eq!(dc.validate(&schemas), false);
    }
}
