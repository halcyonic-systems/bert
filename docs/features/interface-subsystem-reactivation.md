# Feature: Interface Subsystem Reactivation

## Overview

**Feature Name**: Interface Subsystem Reactivation
**Branch**: feature/interface-subsystem-reactivation
**Status**: Complete
**Contributors**: Philip Hua, Claude
**Date**: 2026-01-10

## Description

This feature reactivates the interface subsystem functionality, providing a keyboard shortcut ('I' key) to create small circular subsystems at interfaces. This implements the Mobus 8-tuple principle that interfaces are a subset of subsystems (I ⊆ C), making the theoretical relationship explicit and visual.

**Theoretical Foundation**: Per Mobus's systems science formalization:
- `S = ⟨B, I, O_in, O_out, C, N, G, H⟩` where `I ⊆ C`
- Interfaces are not just "holes" in the boundary—they ARE subsystems with internal structure
- Interface subsystems process, validate, or regulate what crosses the boundary

## Implemented Functionality

- **Keyboard shortcut**: Press 'I' when an interface is selected to create an interface subsystem
- **Auto-detection**: Interface type (Import/Export) determined from connected flows
- **Size control**: Interface subsystems remain small (4% of parent) via `INTERFACE_SUBSYSTEM_SCALING_FACTOR`
- **Skip protection**: Interfaces with existing subsystems are skipped (no duplicates)
- **Visual placement**: Subsystem positioned at the interface boundary

## Technical Implementation

### Components Added

- `src/bevy_app/systems/interface_subsystem.rs`: New system for keyboard shortcut handling

### Components Modified

- `src/bevy_app/systems/mod.rs`: Added module export
- `src/bevy_app/mod.rs`: Registered system in Update schedule
- `src/bevy_app/systems/palette.rs`: Auto-select interfaces after placement
- `src/bevy_app/bundles/spawn/subsystem.rs`: Fixed z-level for interface subsystem children
- `src/bevy_app/constants.rs`: Added `INTERFACE_SUBSYSTEM_SCALING_FACTOR` (4%)
- `src/bevy_app/systems/subsystem.rs`: Use dedicated scaling factor for interface subsystems

### Bug Fixes During Implementation

1. **Newly placed interfaces not working**: Interfaces placed via palette were not auto-selected, so 'I' key didn't work. Fixed by setting `is_selected: true` in `palette.rs`.

2. **Interface subsystems invisible (z=-90)**: Child z-position was calculated relative to parent interface. `SUBSYSTEM_Z - INTERFACE_Z = 10 - 100 = -90` put them behind everything. Fixed to use `z = 5.0` (small positive offset).

3. **Wrong size (14% instead of 4%)**: Code comment promised 4% but used `SUBSYSTEM_MIN_SCALING_FACTOR` (14%). Created dedicated `INTERFACE_SUBSYSTEM_SCALING_FACTOR = 0.04` constant.

### Architecture Decisions

**Keyboard Shortcut Approach**: Chosen over context menu or palette drag-and-drop because:
1. Simplest implementation path (lowest complexity)
2. Consistent with existing keyboard-driven patterns (F for connection mode)
3. Discoverable through tooltip/documentation
4. Leverages existing `spawn_interface_subsystem()` function

**Reusing Existing Spawn Function**: The `spawn_interface_subsystem()` in `bundles/spawn/subsystem.rs` was already fully implemented but button-gated. This feature simply provides a new trigger pathway.

## Usage Examples

```
1. Create or load a model with interfaces
2. Click on an interface to select it
3. Press 'I' key
4. Interface subsystem appears as small circle at the interface
5. Use connection mode ('F') to create internal flows to/from the interface subsystem
```

## Testing Strategy

### Manual Testing Sequence

```
1. Run: trunk serve
2. Open http://localhost:1320
3. Load model with interfaces (e.g., bitcoin-4.1.json)
4. Select an interface (click on it)
5. Press 'I' key
6. Verify: Small subsystem appears at interface
7. Test flow attachment:
   - Press 'F' to enter connection mode
   - Click interface subsystem, then internal subsystem
   - Verify flow connects properly
8. Test handle dragging on the new flow
9. Save model (Ctrl+S)
10. Reload model (Ctrl+O)
11. Verify interface subsystem and flow persist correctly
```

### Automated Tests

Unit tests pending—covered by existing `spawn_interface_subsystem` tests.

## Future Improvements

- **Context menu option**: Right-click interface → "Add Interface Subsystem"
- **Visual feedback**: Toast notification on successful creation
- **Batch creation**: Keyboard shortcut when multiple interfaces selected
- **Remove/toggle**: Keyboard shortcut to remove interface subsystem

## Related Documentation

- Session plan: `operations/sessions/2026-01-10/interface-subsystem-reactivation-session.md`
- Mobus 8-tuple: Chapter 6 of *Principles of Systems Science*
- BERT Architecture: `docs/CLAUDE.md`

---

_This documentation was updated for the Interface Subsystem Reactivation feature on 2026-01-10._
