# Leptos App - User Interface

The user interface layer built with the Leptos web framework. This directory contains all UI components and user interaction logic.

## Architecture Overview

**Leptos App** handles:
- User interface rendering and interaction
- Component library and UI patterns
- Integration with embedded Bevy canvas
- File operations and browser APIs

## Key Directories

- **`components/`** - Reusable UI component library
- **`tree/`** - Tree view components for system hierarchy

## Integration with Bevy Engine

The Leptos UI embeds and controls the Bevy systems modeling engine:
- Leptos renders UI controls and menus
- Bevy canvas embedded for system visualization
- Event communication between UI and engine
- Shared state management across frameworks

## Key Files

- **`mod.rs`** - Main Leptos app setup and Bevy integration
- **`details.rs`** - Detailed view components and logic
- **`use_file_dialog.rs`** - File operation utilities and hooks

## Component Architecture

BERT uses a component-based UI architecture:
- **Functional components** with reactive signals
- **Event-driven interactions** with Bevy systems
- **Modular design** for maintainability and testing

## Development Patterns

- **Signals**: Reactive state management
- **Components**: Reusable UI elements
- **Actions**: User interaction handlers
- **Hooks**: Shared logic and utilities

## Browser Integration

Leptos provides web platform integration:
- File system access via browser APIs
- Canvas rendering for Bevy integration
- Responsive design for different screen sizes