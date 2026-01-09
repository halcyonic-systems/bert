# Feature: Flow Endpoint Handles

## Overview

**Feature Name**: Flow Endpoint Handles
**Branch**: feature/flow-endpoint-handles
**Status**: Design Phase
**Contributors**: rsthornton, Claude
**Date**: 2026-01-09

## Problem Statement

Multiple flows between the same subsystem pair (without interfaces) visually stack on top of each other. Users cannot:
- Distinguish individual flows
- Select specific flows
- Reposition flow endpoints independently

Current workaround requires creating full interfaces at every layer, which users report feels "forced" and heavyweight for simple models.

## Description

Mini interface handles - lightweight, toggle-able endpoints for flows that:
1. Allow dragging flow endpoints independently
2. Can optionally expand to full interfaces when needed
3. Don't require the full interface creation workflow

## Design Options

### Option A: Invisible Anchors
- Flows get invisible anchor points at each end
- Anchors are draggable when flow is selected
- **Pro**: No visual clutter
- **Con**: Discoverability issue

### Option B: Mini Interface Buttons
- Small circular handles at flow endpoints
- Visible on hover or when flow selected
- Click to drag, double-click to upgrade to interface
- **Pro**: Discoverable, intuitive
- **Con**: Some visual complexity

### Option C: Auto-Offset
- Multiple flows between same pair automatically offset
- No user interaction needed
- **Pro**: Zero friction
- **Con**: Less user control

## Technical Implementation

### Components to Add

- `FlowEndpointHandle` - Component marking draggable endpoint
- Handle spawn bundle in `bundles/spawn/`

### Components to Modify

- `src/bevy_app/systems/ui/drag.rs` - Handle dragging logic
- `src/bevy_app/data_model/save.rs` / `load.rs` - Serialization
- `src/leptos_app/` - UI for handle interaction

### Architecture Decisions

[To be determined after design option selected]

## Success Criteria

- [ ] Multiple flows between same subsystems are visually distinguishable
- [ ] Users can reposition flow endpoints without creating full interfaces
- [ ] Existing models with interfaces continue to work
- [ ] Save/reload preserves endpoint positions

## Testing Strategy

- Unit tests for handle serialization
- Manual testing with multi-flow models
- Regression test: existing models load correctly

## Future Improvements

- Handle → Interface upgrade path
- Keyboard shortcuts for handle manipulation
- Snap-to-grid for handle positioning

## Related Documentation

- bert-old: Had rigid flow positioning (reference implementation)
- Issue context: #10 (internal flow rendering bug)

---

_This documentation was automatically generated for the Flow Endpoint Handles feature on 2026-01-09._
