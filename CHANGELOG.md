# Changelog

All notable changes to BERT will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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