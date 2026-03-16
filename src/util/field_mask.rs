#![allow(dead_code)]

/// Build a FieldMask string from a list of field paths.
/// Used for partial updates in mutate operations.
pub fn build_field_mask(fields: &[&str]) -> String {
    fields.join(",")
}

/// Build a field mask from an object's non-None fields.
/// Takes a list of (field_name, is_set) pairs and returns the mask for set fields.
pub fn build_field_mask_from_options(fields: &[(&str, bool)]) -> String {
    fields
        .iter()
        .filter(|(_, is_set)| *is_set)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_field_mask() {
        assert_eq!(build_field_mask(&["name", "status"]), "name,status");
    }

    #[test]
    fn test_build_from_options() {
        let mask = build_field_mask_from_options(&[
            ("name", true),
            ("status", false),
            ("budget", true),
        ]);
        assert_eq!(mask, "name,budget");
    }
}
