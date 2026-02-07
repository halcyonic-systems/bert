# Changelog

All notable changes to BERT will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-02-07

### Added
- **E-Network Environmental Flows** - Full environmental feedback/feed-forward flow system
  - Sink→Source feedback flows (curves upward) for environmental feedback loops
  - Source→Sink feed-forward flows (curves downward) for environmental inputs
  - Dashed arc visualization for Source/Sink feedback loops
  - Save/load/drag support for all E-network flow types

- **Draggable Flow Endpoint Handles** - Precise control over flow connection points
  - Drag handles on internal flows to adjust where they connect to subsystem boundaries
  - Angle-based offsets for zoom-independent positioning
  - Boundary constraints prevent handles from leaving parent subsystem
  - Auto-offset for stacking multiple internal flows
  - Full save/load persistence for endpoint positions

- **Interface Subsystem Keyboard Shortcut** - Quick creation of interface subsystems

- **Blockchain Example Models** - Enhanced ethereum.json and bitcoin.json with interface processors and Valid Proof flows

### Changed
- **Bevy 0.15 → 0.17.3** - Major framework upgrade across the entire codebase
  - Leptos 0.7 → 0.8, lyon 0.13 → 0.15, bevy_prototype_lyon 0.15
  - leptos-bevy-canvas 0.4, bevy-inspector-egui 0.35
  - getrandom WASM backend configured for web builds
- **Model Browser** - Reduced to 5 curated example models for quality over quantity

### Fixed
- **Persistence Hardening** - Major reliability improvements to save/load
  - Pre-scan OriginalIds to prevent entity ID collisions on load
  - Preserve entity IDs across save/load cycles (no more position drift)
  - Register all interfaces including those without flows
  - Register interface subsystem IDs to prevent collision
  - Prevent duplicate interfaces on save/reload
  - Prevent duplicate flows and NaN crash on internal flow save/reload
  - Despawn orphaned flow endpoint handles on model reload
- **Flow Rendering** - Resolved endpoint handle and curve rendering at wrong positions for nested flows
- **Selection** - Z-order respected when selecting overlapping entities; subsystem clicking correctly switches panel focus
- **Interface Subsystem Sizing** - Correct 4% scaling factor applied from creation
- **G-Network Flows** - Consistent flow thickness

## [0.2.5] - 2026-01-05

### Added
- **Subsystem Archetypes** - Classify subsystems by HCGS role (Mobus framework)
  - Governance (blue stroke), Economy (green), Agent (orange), Unspecified (black)
  - RadioGroup in subsystem details panel
  - Backward-compatible serialization (older models load without archetype)
  - See [System Archetypes](https://bert.gitbook.io/for-researchers/system-archetypes) for theory

### Changed
- **Repository Cleanup** - Major reduction in repository size and complexity
  - Root: 23 → 18 files
  - Deleted `archive/` (31 obsolete docs) and `gitbook/_book/` (62 build artifacts)
  - Consolidated `docs/` from 8 files + 3 subdirs → 5 files
  - Reorganized `assets/models/` into `examples/` (tracked) + `local/` (gitignored)
  - README: 74 → 36 lines

- **Developer Experience** - Improved onboarding for new contributors
  - CONTRIBUTING.md: 685 → 149 lines (removed intimidating templates)
  - Restored DOCUMENTATION_GUIDELINES.md with practical templates
  - Fixed dead links in gitbook
  - Added MOBUS_REFERENCE.md quick reference

### Removed
- `private-dev/` folder (1.5GB of Python venvs, node_modules, PDFs)
- Stale exploration branches (kept 3 as architectural reference)

## [0.2.4] - 2025-12-17

### Added
- **Click-to-Place Palette** - Redesigned element creation workflow
  - Floating palette with ghost preview
  - Click canvas to place elements
  - Mobus-aligned 3-icon layout

- **Interface as Subsystem** - Optional interface behavior per Mobus I ⊆ C
  - Interfaces can be marked as subsystems
  - Enables proper boundary handler modeling

- **Interface Connections** - Connect interfaces directly
  - Interface ↔ Interface flows now supported
  - G network cross-level connections per Mobus boundary theory

- **Undo/Redo** - Command pattern implementation
  - Ctrl+Z / Ctrl+Y support
  - Event-based execution

- **Screenshot Export** - Save canvas as PNG (Ctrl+P)

### Changed
- Improved visual hierarchy with size adjustments
- Auto-zoom on focus for nested subsystems
- Connection mode UX improvements (auto-deselect, exit after creation)

### Fixed
- Flow curve rendering on first load in web WASM
- N-network flow positioning with correct parenting
- Interface picking at nested levels

## [0.2.3] - 2025-09-08

### Added
- **Smart Parameters Deletion** - Added delete functionality for all Smart Parameters types
  - Clean "×" delete buttons with hover states for all parameter types
  - Safe UUID-based deletion preventing accidental removals
  - Consistent UI across Numeric, Ordinal, Categorical, and Boolean parameters

### Fixed
- **Windows Build Compatibility** - Resolved CI build failures on Windows
  - Removed files with invalid Windows filename characters
  - Improved cross-platform filename handling
  - All CI builds now pass on Windows, macOS, and Linux

### Changed
- Enhanced parameter management UX with intuitive deletion workflow
- Improved overall Smart Parameters system usability

## [0.2.2] - 2025-09-08

### Added
- **Smart Parameters System** - Context-aware parameter system supporting multiple data types:
  - Numeric parameters with units (e.g., temperature, flow rate, shipment value)
  - Ordinal parameters with ordered options (e.g., efficiency levels, priority)
  - Categorical parameters with discrete choices (e.g., commodity types, transport modes)
  - Boolean parameters with custom labels (e.g., active/inactive, international/domestic)
  - Intelligent autocomplete suggestions based on substance type context
  - Economic flow parameters for interstate commerce analysis (SCTG2 commodity codes, transport modes)
  - Automatic Flow.amount synchronization with Shipment Value parameter

### Changed
- **Improved Element Clickability** - Enhanced user interaction with visual elements:
  - Increased flow curve widths by 100% for easier selection
  - Expanded entity clickable areas by ~40%
  - Expanded interface clickable areas by ~40%
  - Better visual feedback and interaction responsiveness

### Fixed
- **Intuitive Zoom Controls** (contributed by @Aaravthk) - Fixed issue #3:
  - Plus (+) key now correctly zooms in
  - Minus (-) key now correctly zooms out
  - Added support for numpad keys (NumpadAdd, NumpadSubtract)
  - Aligned with standard zoom control conventions

- **Subsystem Minimum Size Calculation** - Corrected sizing logic:
  - Reduced subsystem sizes by 73% for more efficient space usage
  - Fixed interface subsystem handling to properly account for contained interfaces
  - Improved visual hierarchy and layout consistency

### Contributors
- @Aaravthk - Fixed zoom control issue #3

### Notes
This release focuses on usability improvements and introduces the Smart Parameters MVP for enhanced system modeling capabilities. The release is ready for collaborator review and testing.

## [0.2.1] - Previous releases...