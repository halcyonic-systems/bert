# BERT (Bounded Entity Reasoning Toolkit)

A visual software tool that guides users through the rigorous analysis and decomposition of complex adaptive systems.

## Quick Start

### Web Version
- Visit [bert.systems](https://bert.systems/) to use BERT directly in your browser

### Desktop Applications
- [MacOS Apple Silicon](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert_0.1.0_aarch64.dmg)
- [MacOS Intel](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert_0.1.0_x64.dmg)
- [Windows](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert.exe)

## User Controls

### Navigation
- **Pan**: Right-click and drag
- **Scroll**: Use mouse wheel to scroll vertically/horizontally
- **Reset View**: Press `Ctrl+R` (both Mac and Windows)

### Zoom
- **Zoom Out**: Press `-` key (elements appear smaller)
- **Zoom In**: Press `=` key (elements appear larger)

### Selection
- **Select Element**: Left-click on an element
- **Multi-select**: Hold `Shift` while clicking elements
- **Deselect All**: Press `Escape`

### Element Management
- **Move Elements**: Click and drag selected elements
- **Delete Elements**: Select element(s) and press `Delete` or `Backspace`
- **Hide Elements**: Select element(s) and press `H`
- **Unhide Elements**: Press `U` to unhide previously hidden elements

### File Operations
- **Save**: Press `Ctrl+S` (both Mac and Windows)

### Advanced
- **Apply Sink/Source Equivalence**: Press `E`

## Learning Resources
- [Tutorial videos](https://github.com/halcyonic-systems/bert/blob/main/docs/Tutorials.md) - Visual walkthroughs of basic functionality

## Key Features

- Guides systematic decomposition of complex systems using Deep Systems Analysis methodology
- Visual system mapping that preserves critical flows, interfaces, and relationships
- Structured knowledge capture in a standardized, computable format
- Hierarchical modeling with unlimited decomposition levels

## Example Analyses

- [The Bitcoin Network](https://github.com/halcyonic-systems/bert/blob/main/btc.json)

## Why BERT?

BERT enables analysts to develop detailed understanding of complex systems while preserving crucial details and meanings often lost through abstract modeling approaches. The tool implements a rigorous methodology called Deep Systems Analysis (DSA).

## Technical Background

BERT implements ideas from [George Mobus's](https://directory.tacoma.uw.edu/employee/gmobus) [Systems Science: Theory, Analysis, Modeling and Design](https://link.springer.com/book/10.1007/978-3-030-93482-8). After an interdisciplinary career spanning naval engineering, robotics, artificial intelligence, computer science, energy systems modeling, and systems science, Mobus identified key limitations in standard systems modeling frameworks like [Stella](https://www.iseesystems.com/store/products/stella-online.aspx) and [UML](https://www.uml.org/)/[SysML](https://sysml.org/). To address these gaps, he proposed the creation of a new formal "System Language" (SL) grounded in systems science principles.

BERT represents a first step toward developing this formal systems language, built specifically for modern systems scientists.

Read more about the [various components of SL](https://github.com/halcyonic-systems/bert/blob/main/research/system%20language/system_language.md).

## Development

BERT is a Rust-based project that uses:
- [Leptos](https://leptos.dev/) for web UIs
- [Bevy](https://bevyengine.org/) for creating and interacting with systems and their components
- [Tauri](https://v2.tauri.app/) for building desktop applications

### Prerequisites
- **Rust**: Install using `rustup` ([installation instructions](https://www.rust-lang.org/tools/install))
- **Tauri**: Install using `cargo install create-tauri-app --locked` ([installation instructions](https://v2.tauri.app/start/))

### Run the Project
```bash
cargo tauri dev
```

### Project Structure
- `src-tauri/src/main.rs`: Application entry point
- `src-tauri/tauri.conf.json`: Tauri configuration
- `src/bevy_app`: Bevy application and components
- `src/leptos_app`: Leptos web UI components

## Contributing
