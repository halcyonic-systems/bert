# Getting Started Developing

Purpose: 5-minute verification, first issues to try, and troubleshooting.

## Prerequisites
- Rust (stable) via rustup
- Node.js (for Tauri v2)
- wasm32 target and trunk (for web dev)

**Alternative: Dev Container**
Open folder in VS Code → "Reopen in Container" for automatic setup with Rust, Node, wasm target, and trunk pre-installed.

## 5-Minute Checks

Web (wasm):
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cd bert/bert
trunk serve
# Open the local URL; canvas renders and toolbar buttons visible
```

Desktop (Tauri v2):
```bash
cd bert/bert
npm install
cargo tauri dev
# App window launches to main UI
```

Tests:
```bash
cd bert/bert
cargo test -p bert
```

**Dev Container Usage:**
- Web: `trunk serve` (or `just web` if using justfile)
- Tests: `cargo test -p bert`
- Lint/format: `cargo clippy -- -D warnings` / `cargo fmt --all`
- Note: Desktop (Tauri) GUI is best run on host OS due to GUI requirements

## Good First Issues
- Fix Zoom Controls Mapping (map plus/equal → ZoomIn, minus/underscore → ZoomOut; update controls menu)
- Add Foundational Unit Tests (components and data model)
- Desktop Screenshot (guarded for non-wasm; emit toast on success)

## Troubleshooting

Web:
- Missing wasm target: run `rustup target add wasm32-unknown-unknown`
- Missing trunk: `cargo install trunk --locked`
- Key handlers not working: ensure the canvas container has focus and prevents browser zoom (`preventDefault()`)

Desktop:
- Tauri prerequisites (macOS): Xcode Command Line Tools, Node installed
- If file dialogs don’t work: confirm Tauri APIs are available (`__TAURI__` check in code)

Bevy/Leptos Integration:
- Keyboard focus: Leptos handles keys; see `src/leptos_app/mod.rs` for handlers
- System registration: add/update in `src/bevy_app/mod.rs` under the correct schedule
- Data flow: `react_to_trigger_event` → serialization → `TreeEvent` → UI

## Conventions
- `cargo fmt --all`, `cargo clippy -- -D warnings`, `cargo test -p bert` before commits
- Prefer adding new systems in `src/bevy_app/systems/` and exporting via `systems/mod.rs`