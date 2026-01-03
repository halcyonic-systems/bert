# BERT - Claude Code Context

## Critical Architecture Knowledge

### BERT is Tauri + WASM, NOT Native Rust

**This is the most important thing to understand:**

```
Desktop app = Tauri wrapping a WASM webview
           = target_arch is ALWAYS wasm32, even on desktop
           = #[cfg(not(target_arch = "wasm32"))] code NEVER runs
```

**Implications:**
- Use `js_sys` and `web_sys` for browser APIs, not native Rust equivalents
- Use `js_sys::Date` instead of `chrono` for timestamps
- Use `web_sys::Blob` + anchor download instead of native file I/O
- Don't use `#[cfg(not(target_arch = "wasm32"))]` gates for "desktop" features

**Testing contexts:**
- `trunk serve` → Browser at localhost (WASM)
- `cargo tauri dev` → Desktop window (also WASM in webview)
- `cargo run` → Native binary (works differently, NOT how users run BERT)

### Keyboard Shortcuts Use Ctrl, Not Cmd

The `MODIFIER` constant in `constants.rs` maps to `Ctrl`, not platform-aware `Cmd` on Mac.
- Save: `Ctrl+S` (not `Cmd+S`)
- Screenshot: `Ctrl+P`
- All shortcuts use `Ctrl` regardless of platform

### Downloads Location (Screenshots, Exports)

Browser downloads via `Blob` + anchor go to the system's default **Downloads folder**.
- No native "Save As" dialog in WASM context
- Users should check `~/Downloads` for exported files
- Toast messages should say "saved to Downloads" (not a custom path)

## Tech Stack

| Layer | Technology | Location |
|-------|------------|----------|
| UI Components | Leptos 0.7 | `src/leptos_app/` |
| Visualization | Bevy 0.15 | `src/bevy_app/` |
| Desktop Wrapper | Tauri v2 | `src-tauri/` |
| Styling | Tailwind CSS | `styles.css`, `tailwind.config.js` |

## Development Commands

```bash
# Web development
trunk serve                    # Browser at http://localhost:1320

# Desktop development
cargo tauri dev                # Tauri desktop window

# Testing
cargo test -p bert             # Run tests
cargo check --target wasm32-unknown-unknown  # Verify WASM compilation

# Linting
cargo fmt --all                # Format code
cargo clippy -- -D warnings    # Lint
```

---

## Entry Points & Event Flow

### Key Entry Points
- **Bevy app init**: `src/bevy_app/mod.rs` → `init_bevy_app(...)` (schedules, events, resources)
- **Systems orchestration**: `src/bevy_app/systems/mod.rs` (camera, setup, ui, removal)
- **UI root + toolbar**: `src/leptos_app/mod.rs` (buttons, key handlers)
- **UI components**: `src/leptos_app/components/*`
- **Data model**: `src/bevy_app/data_model/*` (save/load/serialize)

### Event Flow (UI → Bevy → UI)
```
UI (Leptos)
  ↓ TriggerEvent
Bevy: react_to_trigger_event → serialize_world → send_world_to_leptos
  ↑ TreeEvent
UI (Leptos) updates Tree
```

**Key files for event flow:**
- `src/bevy_app/systems/mod.rs`: `react_to_trigger_event`, `send_world_to_leptos`
- `src/bevy_app/data_model/save.rs`, `load.rs`
- `src/leptos_app/mod.rs`: event_l2b / event_b2l wiring

### Scheduling
- **PreUpdate**: Input handling, zoom, deselection
- **Update**: Core logic (spawn/update/cleanup sets)
- **PostUpdate**: UI sync, label auto-spawn, geometry updates

---

## Where to Put Things

### Add a Bevy System
1. Create `src/bevy_app/systems/your_system.rs`
2. Export in `systems/mod.rs`: `mod your_system; pub use your_system::*;`
3. Register in `bevy_app/mod.rs` under `.add_systems(Update|PostUpdate, ...)`
4. Group with existing sets (`AllSet`, `ZoomSet`, `CreateButtonSet`) when relevant

### Add a UI Button or Key Handler
- Toolbar buttons: `src/leptos_app/mod.rs`
- Reusable components: `src/leptos_app/components/*`

### Add a Leptos Component
1. Create `src/leptos_app/components/my_component.rs`
2. Export in `components/mod.rs`: `mod my_component; pub use my_component::*;`
3. Import in parent: `use crate::leptos_app::components::MyComponent;`

### Other Locations
- **Save/Load**: `src/bevy_app/data_model/{save,load}.rs`
- **Toast notifications**: `src/bevy_app/systems/toast_handler.rs`
- **Constants**: `src/bevy_app/constants.rs`

---

## Code Patterns

### Modal Component Pattern
```rust
#[component]
pub fn MyModal(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-30 flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-xl max-w-4xl max-h-[90vh] overflow-y-auto m-4">
                    <div class="p-6">
                        <div class="flex justify-between items-center mb-6">
                            <h2 class="text-2xl font-bold text-gray-900">"Modal Title"</h2>
                            <button
                                class="text-gray-400 hover:text-gray-600 text-2xl font-bold"
                                on:click=move |_| on_close.run(())
                            >
                                "×"
                            </button>
                        </div>
                        // Modal content here
                    </div>
                </div>
            </div>
        </Show>
    }
}
```

### Button Group Pattern
```rust
<div class="absolute top-4 left-4 z-20 flex gap-2">
    <button
        class="px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow"
        on:click=move |_| { /* action */ }
    >
        {"Button Text"}
    </button>
</div>
```

### Signal Management Pattern
```rust
// In App component
let (feature_visible, set_feature_visible) = signal(false);

// In component usage
<MyComponent
    visible=feature_visible
    on_close=Callback::new(move |_| set_feature_visible.set(false))
/>
```

### Module Integration Pattern
**Both lines required in `mod.rs`:**
```rust
mod my_component;
pub use my_component::*;
```

---

## Styling Conventions

### Z-Index Hierarchy
- `z-10`: Background elements
- `z-20`: UI controls (buttons, toolbars)
- `z-30`: Modals and overlays
- `z-40`: Tooltips and dropdowns

### Color Scheme
- **Headers**: `text-gray-900`
- **Body text**: `text-gray-700`
- **Cards/backgrounds**: `bg-white`
- **Code blocks**: `bg-gray-100`
- **Borders**: `border-gray-200`

### Button Classes
```
px-4 py-2 rounded-lg bg-white shadow-md hover:shadow-lg transition-shadow
```

### UI Control Positioning
```
absolute top-4 left-4 z-20
```

---

## Debugging Patterns

### Compilation Errors

**Leptos callback error**: `expected function, found Callback<()>`
```rust
// Wrong (Leptos 0.6 style)
on:click=move |_| on_close(())

// Correct (Leptos 0.7+)
on:click=move |_| on_close.run(())
```

**Module not found**: `unresolved import`
```rust
// Missing - need BOTH lines in mod.rs
mod my_component;
pub use my_component::*;
```

**Import path error**: `use of undeclared crate or module`
```rust
// Wrong (relative)
use super::components::MyComponent;

// Correct (absolute)
use crate::leptos_app::components::MyComponent;
```

### Runtime Errors

**Signal not updating**: UI not reacting to state changes
```rust
// Wrong (non-reactive)
let visible = my_signal.get_untracked();

// Correct (reactive)
<Show when=move || my_signal.get()>
```

**Props not updating**: Component doesn't respond to prop changes
```rust
// Wrong (static)
#[component]
pub fn MyComponent(visible: bool) -> impl IntoView

// Correct (reactive)
#[component]
pub fn MyComponent(#[prop(into)] visible: Signal<bool>) -> impl IntoView
```

### Build Errors

**Port conflict**: `Address already in use (os error 48)`
```bash
pkill -f "tauri dev"
pkill -f "trunk serve"
# Or: lsof -ti:1320 | xargs kill -9
```

---

## Quality Gates

### Before Any PR
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo doc --no-deps --quiet
```

### New Feature Checklist
- [ ] Generate docs: `./scripts/bert.sh feature "Feature Name"`
- [ ] Test compilation after each significant change
- [ ] Manual test: `trunk serve` or `cargo tauri dev`
- [ ] No debug code remains
- [ ] Documentation updated

---

## Common Gotchas

### Cargo/Dependencies
- Keep dependencies in main `[dependencies]`, not platform-specific sections
- `web-sys`: Must explicitly enable each DOM API feature used

### Bevy 0.15
- Uses observer pattern for some features (e.g., screenshots)
- Asset paths should NOT include "assets/" prefix (use `"icons/foo.png"` not `"assets/icons/foo.png"`)
- No native SVG support - convert to PNG first
- `FocusedSystem` resource has many hidden dependents - don't disable without mapping dependencies

### Leptos 0.7
- Callbacks use `.run()` method, not direct invocation
- Use `Memo::new()` not `Signal::derive()` - latter causes disposal issues with `Show` components

### WASM/WebGL
- Stricter texture format requirements than native
- PNG 16-bit causes `Rgba16Unorm` errors - use 8-bit PNGs
- Test asset changes incrementally, not bulk replacement

### Architectural Guidance
- Before deleting systems, map the full dependency graph
- Test in isolation before integrating
- "Slow down, understand deeply, then implement incrementally"

---

## Related Documentation

- **Contributing guide**: `CONTRIBUTING.md` (root)
- **Build & release**: `docs/BUILD.md`
- **User documentation**: [bert.gitbook.io](https://bert.gitbook.io)
- **Deep architecture**: `gitbook/for-developers/architecture/`
