//! TypeQL schema for BERT models.
//!
//! The schema is extracted verbatim from `bert/docs/bert-typedb-schema.md`
//! (the design document) and embedded at compile time via `include_str!`.
//! Keeping both sources means edits to the design doc don't silently drift
//! from what the transpiler ships — regenerate `schema.tql` whenever the
//! doc's TypeQL code block changes.
//!
//! Regenerate via (from repo root):
//!
//! ```sh
//! awk '/^```typeql$/{flag=1; next} /^```$/{flag=0} flag' \
//!     docs/bert-typedb-schema.md > tools/bert-typedb/schema.tql
//! ```

/// The complete TypeQL schema as a single `define`-block string.
///
/// Pass this directly to a TypeDB `Schema` transaction's `query()` method.
/// All `define` statements are idempotent; re-loading the schema is safe.
pub const SCHEMA_TQL: &str = include_str!("../schema.tql");

#[cfg(test)]
mod tests {
    use super::SCHEMA_TQL;

    #[test]
    fn schema_is_nonempty_and_well_formed() {
        assert!(!SCHEMA_TQL.is_empty(), "schema.tql should not be empty");
        assert!(
            SCHEMA_TQL.trim_start().starts_with("define"),
            "schema should start with `define` keyword"
        );
    }

    #[test]
    fn schema_contains_all_expected_entity_types() {
        // Every SL primitive mapped in bert-typedb-schema.md §Entities must
        // have an `entity X,` declaration somewhere in the schema. Guards
        // against silent extraction regressions where a subsection gets
        // clipped by an inadvertent markdown edit.
        for entity in [
            "entity bert_model",
            "entity system",
            "entity external_entity",
            "entity boundary",
            "entity interface",
            "entity interaction",
            "entity agent_model",
            "entity primitive_assignment",
            "entity cognitive_parameter",
        ] {
            assert!(
                SCHEMA_TQL.contains(entity),
                "schema missing entity declaration: {entity}"
            );
        }
    }

    #[test]
    fn schema_contains_all_expected_relations() {
        for relation in [
            "relation composition",
            "relation in_environment",
            "relation has_boundary",
            "relation has_interface",
            "relation participates_in",
            "relation routes_through",
            "relation is_equivalent_to",
            "relation has_agent_config",
            "relation has_primitive",
            "relation has_cognitive_param",
        ] {
            assert!(
                SCHEMA_TQL.contains(relation),
                "schema missing relation declaration: {relation}"
            );
        }
    }

    #[test]
    fn schema_has_values_constraints_on_controlled_vocabulary_enums() {
        // Presence of @values on enum-like attributes is the mechanism that
        // enforces SL spec vocabulary at insert time — if this disappears
        // silently the transpiler will happily insert bad values.
        for attr_with_values in [
            "attribute archetype",
            "attribute substance_type",
            "attribute interaction_type",
            "attribute usability",
            "attribute interface_type",
            "attribute agent_kind",
            "attribute process_primitive",
        ] {
            let idx = SCHEMA_TQL
                .find(attr_with_values)
                .unwrap_or_else(|| panic!("attribute missing: {attr_with_values}"));
            let after = &SCHEMA_TQL[idx..idx.saturating_add(300)];
            assert!(
                after.contains("@values"),
                "{attr_with_values} should have a @values constraint"
            );
        }
    }
}
