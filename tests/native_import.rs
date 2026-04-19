//! Regression test: ensures external native Rust crates can import and use
//! `bert::bevy_app::data_model::WorldModel` for JSON deserialization.
//!
//! This test guards the lib+bin split introduced in issue #42. It is run
//! on the default (native) cargo target, exercising the native compile path
//! for the bert library — distinct from the WASM path exercised by
//! `cargo check --target wasm32-unknown-unknown`.
//!
//! Downstream consumers (e.g. the BERT→TypeDB transpiler, #37) depend on
//! being able to read a BERT JSON model natively without pulling in a full
//! Bevy App. If this test ever fails, investigate the lib crate structure
//! before modifying the test.

use bert::bevy_app::data_model::WorldModel;

#[test]
fn bitcoin_json_deserializes_as_worldmodel() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/models/examples/bitcoin.json"
    );
    let bytes = std::fs::read(path).expect("bitcoin.json should exist");
    let model: WorldModel =
        serde_json::from_slice(&bytes).expect("bitcoin.json should deserialize");

    // Smoke: some plausible sizes so a silent empty parse doesn't pass.
    assert!(
        !model.systems.is_empty(),
        "WorldModel.systems should not be empty"
    );
    assert!(
        !model.interactions.is_empty(),
        "WorldModel.interactions should not be empty"
    );
    assert_eq!(model.version, 1);
}
