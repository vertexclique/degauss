/// Backward compatibility: A new schema is backward compatible if it can be used to read the data written in the
/// previous schema.
#[cfg(test)]
mod backward_compat {

    use avro_rs::Schema;
    use degauss::prelude::*;

    #[test]
    fn adding_a_field_with_default_is_a_backward_compatible() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Backwards);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_a_field_wo_default_is_not_a_backward_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Backwards);
        assert_eq!(dc.validate(&schemas), false);
    }

    #[test]
    fn evolving_a_field_type_to_a_union_is_a_backward_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema6.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Backwards);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_type_from_a_union_is_not_a_backward_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema6.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Backwards);
        assert_eq!(dc.validate(&schemas), false);
    }

    #[test]
    fn adding_a_new_type_in_union_is_a_backward_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema7.avsc").unwrap(),
            Schema::parse_file("tests/data/schema6.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Backwards);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_type_from_a_union_is_not_a_backward_compatible_change_second_try() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema6.avsc").unwrap(),
            Schema::parse_file("tests/data/schema7.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Backwards);
        assert_eq!(dc.validate(&schemas), false);
    }

    #[test]
    fn removing_a_default_is_not_a_transitively_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::BackwardsTransitive);
        assert_eq!(dc.validate(&schemas), true);
    }
}
