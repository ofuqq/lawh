/// Integration tests: parse and round-trip the sample `.ruqa` study boards in `examples/`.
///
/// These tests guarantee that the bundled example files:
/// 1. Deserialize from JSON without errors.
/// 2. Contain at least one element (they are non-empty boards).
/// 3. Carry the expected schema version and format magic string.
/// 4. Re-serialize to JSON without loss (round-trip identity).
use ruqa_core::document::{RuqaDocument, CURRENT_SCHEMA_VERSION, FORMAT_MAGIC};

/// Path to the examples directory relative to the workspace root.
/// `env!("CARGO_MANIFEST_DIR")` resolves to `crates/ruqa-core` at compile-time,
/// so we step up two levels to reach the workspace root.
fn examples_dir() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent() // crates/
        .expect("crates dir")
        .parent() // workspace root
        .expect("workspace root")
        .join("examples")
}

/// Parses a `.ruqa` JSON file and returns the deserialized document.
fn load(filename: &str) -> RuqaDocument {
    let path = examples_dir().join(filename);
    let json =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {filename}: {e}"));
    serde_json::from_str(&json).unwrap_or_else(|e| panic!("failed to parse {filename}: {e}"))
}

#[test]
fn example_dijkstra_parses_correctly() {
    let doc = load("dijkstra-shortest-path.ruqa");
    assert_eq!(doc.format, FORMAT_MAGIC, "format magic mismatch");
    assert_eq!(
        doc.schema_version, CURRENT_SCHEMA_VERSION,
        "schema version mismatch"
    );
    assert!(
        !doc.elements.is_empty(),
        "board must contain at least one element"
    );
}

#[test]
fn example_distributed_systems_parses_correctly() {
    let doc = load("distributed-systems-cache.ruqa");
    assert_eq!(doc.format, FORMAT_MAGIC);
    assert_eq!(doc.schema_version, CURRENT_SCHEMA_VERSION);
    assert!(!doc.elements.is_empty());
}

#[test]
fn example_binary_search_tree_parses_correctly() {
    let doc = load("binary-search-tree.ruqa");
    assert_eq!(doc.format, FORMAT_MAGIC);
    assert_eq!(doc.schema_version, CURRENT_SCHEMA_VERSION);
    assert!(!doc.elements.is_empty());
}

/// Serializing and re-deserializing a document must produce the same element count
/// and the same element IDs — a basic round-trip sanity check.
#[test]
fn example_boards_round_trip() {
    for filename in &[
        "dijkstra-shortest-path.ruqa",
        "distributed-systems-cache.ruqa",
        "binary-search-tree.ruqa",
    ] {
        let doc = load(filename);
        let element_count = doc.elements.len();
        let ids: Vec<u64> = doc.elements.iter().map(|e| e.id()).collect();

        let json = serde_json::to_string_pretty(&doc)
            .unwrap_or_else(|e| panic!("serialize failed for {filename}: {e}"));
        let doc2: RuqaDocument = serde_json::from_str(&json)
            .unwrap_or_else(|e| panic!("re-parse failed for {filename}: {e}"));

        assert_eq!(
            doc2.elements.len(),
            element_count,
            "{filename}: element count changed after round-trip"
        );
        let ids2: Vec<u64> = doc2.elements.iter().map(|e| e.id()).collect();
        assert_eq!(
            ids2, ids,
            "{filename}: element IDs changed after round-trip"
        );
    }
}
