/// Backward compatibility: A new schema is backward compatible if it can be used to read the data written in the
/// previous schema.
#[cfg(test)]
/// Forward compatibility: A new schema is forward compatible if the previous schema can read data written in this
/// schema.
mod forward_compat {

    use apache_avro::Schema;
    use degauss::prelude::*;

    #[test]
    fn adding_a_field_is_a_forward_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Forward);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_a_field_is_a_forward_compatible_change_second_try() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Forward);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_a_field_is_a_forward_compatible_change_third_try() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Forward);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn adding_a_field_is_a_forward_compatible_change_fourth_try() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Forward);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn removing_a_default_is_not_a_transitively_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Forward);
        assert_eq!(dc.validate(&schemas), true);
    }
}
