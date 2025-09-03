# BERT Architecture – Quick Overview

Purpose: 2–3 minute orientation for engineers. Where to put code, how data flows.

## At a Glance
- Bevy (rendering, ECS systems) ↔ Leptos (UI) ↔ Tauri (desktop)
- Entry points:
  - Bevy app init: `src/bevy_app/mod.rs` → `init_bevy_app(...)` (schedules, events, resources)
  - Systems orchestration: `src/bevy_app/systems/mod.rs` (camera, setup, ui, removal, etc.)
  - UI root + toolbar: `src/leptos_app/mod.rs` (buttons, key handlers)
  - UI components: `src/leptos_app/components/*`
  - Data model: `src/bevy_app/data_model/*` (save/load/serialize, complexity)

## Event & Data Flow (UI → Bevy → UI)
- UI emits `TriggerEvent` (e.g., “Show Tree”) from `src/leptos_app/mod.rs`
- Bevy reads it in `react_to_trigger_event` → pipes to `serialize_world` → `send_world_to_leptos`
- UI receives `TreeEvent` and updates state

Key files:
- `src/bevy_app/systems/mod.rs`: `react_to_trigger_event` and `send_world_to_leptos`
- `src/bevy_app/data_model/save.rs`, `load.rs`
- `src/leptos_app/mod.rs`: event_l2b / event_b2l wiring, toolbar buttons

ASCII sketch:
UI (Leptos)
  ↓ TriggerEvent
Bevy: react_to_trigger_event → serialize_world → send_world_to_leptos
  ↑ TreeEvent
UI (Leptos) updates Tree

## Scheduling & Registration
- Schedules:
  - PreUpdate: input handling, zoom, deselection
  - Update: core logic (spawn/update/cleanup sets)
  - PostUpdate: UI sync, label auto-spawn, geometry updates
- Registration:
  - Add new systems under `.add_systems(PreUpdate|Update|PostUpdate, (...))` in `src/bevy_app/mod.rs`
  - Group with existing sets (e.g., `AllSet`, `ZoomSet`, `CreateButtonSet`) when relevant

## Common Tasks & Where to Put Things
- Add a Bevy system: `src/bevy_app/systems/your_system.rs`, export in `systems/mod.rs`, register in `bevy_app/mod.rs`
- Add a UI button or key handler: `src/leptos_app/mod.rs` (top-left buttons), or `src/leptos_app/components/*`
- Save/Load world: `src/bevy_app/data_model/{save,load}.rs`
- Toast/notifications: `src/bevy_app/systems/toast_handler.rs` (events bridged to UI)

## Useful Pointers
- Keyboard focus/zoom: handled in Leptos (canvas container) to avoid Bevy focus quirks
- Desktop vs Web:
  - Desktop features may use Tauri (file dialogs)
  - Web features should avoid desktop-only APIs (guard with `#[cfg(not(target_arch = "wasm32"))]`)