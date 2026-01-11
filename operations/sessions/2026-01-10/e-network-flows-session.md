# E-Network Environmental Flows Session

**Date:** 2026-01-10
**Branch:** `feature/feedback-loop-visualization`

## Accomplishments

- Implemented E-network curve direction rule: feedback (Sink→Source) curves UP, feed-forward (Source→Sink) curves DOWN
- Fixed thin outflow rendering by using `min(source, dest)` nesting level for G-network flows
- Fixed zoom stroke width to use correct constant per element type (FLOW_LINE_WIDTH vs EXTERNAL_ENTITY_LINE_WIDTH)
- Removed verbose per-frame logging from feedback_arc.rs

## Commits

- `57e921e` - fix(e-network): feedback curves UP, feed-forward curves DOWN
- `6ff51ea` - fix(flows): consistent G-network flow thickness and reduce logging

## Technical Details

### G-Network Flow Thickness Issue
The root cause was that connection_mode.rs used `source_nesting_level` for flow scale, but:
- Interfaces have nesting level 1
- External entities have nesting level 0

When creating an outflow (Interface clicked first), scale used level 1 (smaller).
When creating an inflow (External entity clicked first), scale used level 0 (larger).

Fix: Use `min(source_nesting_level, dest_nesting_level)` for G-network flows.

### Zoom Stroke Width Issue
`apply_zoom_to_strokes` in zoom.rs was applying `EXTERNAL_ENTITY_LINE_WIDTH` (5.0) to ALL entities with HighlightBundles, including flows that should use `FLOW_LINE_WIDTH` (6.0).

Fix: Check `SystemElement` type and use appropriate constant.

## Related Files

- `src/bevy_app/systems/connection_mode.rs` - G-network nesting level fix
- `src/bevy_app/systems/ui/zoom.rs` - Stroke width per element type
- `src/bevy_app/systems/ui/feedback_arc.rs` - Logging cleanup
