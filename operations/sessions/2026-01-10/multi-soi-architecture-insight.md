# Multi-SOI Architecture Insight

**Date:** 2026-01-10
**Context:** E-network environmental flows implementation

## Vision

Allow expanding Sources/Sinks into full SOIs on the same canvas, enabling recursive system decomposition per Mobus theory. The flow would be:
- Click Source → "Expand" → becomes a peer SOI
- The E-network flow connects the original SOI's Sink to the new SOI's import interface
- "Collapse" reverses this back to a Source/Sink

## Current Foundation

The E-network flow feature (Sink→Source feedback, Source→Sink feed-forward) establishes that Sources and Sinks can be connected, which is the conceptual link between SOIs.

## Architectural Barriers

| Barrier | Current State | Required Change |
|---------|--------------|-----------------|
| `FocusedSystem` singleton | Single SOI assumption | Support multiple root systems or remove requirement |
| Nesting level math | Relative to single focused system | Per-SOI nesting contexts |
| Canvas rendering | Single SOI boundary circle | Multiple SOI boundaries |
| Navigation model | "Dive into" swaps FocusedSystem | Side-by-side SOI view |
| Save/Load | Single root system | Multiple root systems |
| Connection mode | No SOI context awareness | Know which SOI you're connecting within/between |

## Implementation Path

1. **WorldCanvas** concept with multiple `System` entities at root level
2. Each "expanded Source/Sink" becomes a peer System entity
3. E-network flows become the visual/data links between SOIs
4. Refactor FocusedSystem → optional or multi-select
5. New UX for "expand/collapse" on external entities

## Estimate

Major architectural evolution (BERT 3.0 level) - approximately 2-4 weeks of focused work with significant refactoring.

## Related Files

- `src/bevy_app/resources.rs` - FocusedSystem resource
- `src/bevy_app/components/system_elements.rs` - System component
- `src/bevy_app/systems/connection_mode.rs` - E-network validation
- `src/bevy_app/data_model/save.rs` / `load.rs` - Serialization
