# Model Browser Feature Implementation Plan

## Overview
This document outlines the implementation plan for adding a model browser feature to BERT (Bounded Entity Reasoning Toolkit). The model browser will allow users to easily navigate and load pre-existing JSON models from a library directly within the desktop GUI or web interface.

## Feature Description
A user-friendly interface that presents a curated library of JSON models, allowing users to:
- Browse available models in a grid/list view
- Search and filter models by category
- Preview model descriptions
- Load models with a single click
- Access recently used models

## Architecture Analysis

### Current State
- **Model Format**: JSON-based `WorldModel` structure with systems, interactions, and environment
- **Loading Mechanism**: Existing `LoadFileEvent` system handles both desktop (Tauri) and web environments
- **UI Framework**: Leptos reactive components with real-time Bevy ECS synchronization
- **File Handling**: Platform-specific implementations using Tauri file dialog (desktop) and FileReader API (web)

### Integration Points
1. **Controls Menu** (`src/leptos_app/controls.rs`) - Add model browser button
2. **Load System** (`src/bevy_app/data_model/load.rs`) - Extend to handle library models
3. **Component System** (`src/leptos_app/components/`) - New model browser component
4. **Asset Pipeline** - Bundle example models with application

## Implementation Plan

### Phase 1: Core MVP (2-4 hours)

#### 1. Create Model Browser Component
**File**: `src/leptos_app/components/model_browser.rs`

```rust
// Basic structure:
- Modal/panel container
- Grid layout for model cards
- Model card component (thumbnail, title, description)
- Close button and basic navigation
```

#### 2. Add UI Integration
**File**: `src/leptos_app/controls.rs`
- Add "Model Browser" button to controls menu
- Wire up click handler to show/hide browser
- Add keyboard shortcut (Ctrl+B / Cmd+B)

#### 3. Bundle Example Models
**Directory**: `assets/models/`
```
assets/
└── models/
    ├── manifest.json         # Model metadata and categories
    ├── examples/
    │   ├── circuit.json      # Electrical system example
    │   ├── ecosystem.json    # Biological system example
    │   ├── mechanical.json   # Mechanical system example
    │   ├── network.json      # Information system example
    │   └── hybrid.json       # Multi-domain example
    └── thumbnails/           # Optional preview images
```

#### 4. Implement Loading Logic
**Modify**: `src/bevy_app/data_model/load.rs`
- Add `ModelSource` enum for different loading contexts
- Extend load system to handle bundled assets
- Platform-specific loading strategies

### Phase 2: Platform-Specific Handling

#### Desktop (Tauri)
```rust
enum DesktopModelSource {
    Bundled(String),      // Ships with app
    UserLibrary(PathBuf), // ~/Documents/BERT/models/
    Downloaded(PathBuf),  // Models fetched from remote
}
```

#### Web Browser
```rust
enum WebModelSource {
    Bundled(String),     // Statically served
    Remote(String),      // Fetch from CDN/GitHub
    Cached(String),      // IndexedDB storage
}
```

### Phase 3: Enhanced Features (30 min each)

1. **Search Functionality**
   - Add search input to filter models by name/description
   - Implement fuzzy search for better UX

2. **Category Filters**
   - Parse categories from manifest.json
   - Add filter buttons/dropdown
   - Show category badges on model cards

3. **Recent Models**
   - Track last 5-10 loaded models
   - Persist to localStorage/preferences
   - Quick access section at top

4. **Model Metadata**
   - Extended descriptions
   - Author information
   - Complexity indicators
   - Domain tags

## Technical Considerations

### Model Manifest Format
```json
{
  "version": 1,
  "models": [
    {
      "id": "circuit-basic",
      "name": "Basic Circuit",
      "description": "Simple electrical circuit with battery, resistor, and LED",
      "category": "electrical",
      "tags": ["beginner", "tutorial"],
      "file": "examples/circuit.json",
      "thumbnail": "thumbnails/circuit.png"
    }
  ],
  "categories": [
    { "id": "electrical", "name": "Electrical Systems" },
    { "id": "mechanical", "name": "Mechanical Systems" },
    { "id": "biological", "name": "Biological Systems" }
  ]
}
```

### State Management
```rust
// Add to app state
#[derive(Clone, Debug)]
struct ModelLibrary {
    models: Vec<ModelInfo>,
    categories: Vec<Category>,
    search_query: RwSignal<String>,
    selected_category: RwSignal<Option<String>>,
    is_open: RwSignal<bool>,
}
```

### Loading Flow
1. User opens model browser (Ctrl+B)
2. Component reads manifest and displays available models
3. User clicks on a model card
4. System loads model data based on platform:
   - Desktop: Read from bundled assets or user library
   - Web: Fetch from static assets or remote URL
5. Model data passed to existing `LoadFileEvent` system
6. Browser closes and model loads in main view

## File Structure Changes
```
bert/
├── assets/
│   └── models/              # NEW: Model library
├── src/
│   └── leptos_app/
│       └── components/
│           └── model_browser.rs  # NEW: Browser component
```

## Benefits
- **Discoverability**: New users can immediately explore BERT's capabilities
- **Education**: Example models demonstrate best practices
- **Efficiency**: Quick access to common model templates
- **Sharing**: Foundation for community model sharing

## Implementation Order
1. Create basic UI component with static model list
2. Wire up to controls menu
3. Implement click-to-load with bundled examples
4. Add search/filter functionality
5. Implement platform-specific optimizations
6. Add metadata and enhanced features

## Success Metrics
- Model browser loads in < 100ms
- Models load with single click
- Works identically on desktop and web
- No disruption to existing load functionality

## Future Enhancements
- User model library management
- Model sharing/export
- Remote model repositories
- Model versioning and updates
- Collaborative model editing