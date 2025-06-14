# BERT Settings Infrastructure Research Guide

## Executive Summary

This document provides comprehensive research guidance for implementing a generalized settings infrastructure in BERT, triggered by a user request for black & white theme support. Through architectural analysis, we determined that a **settings modal approach** is significantly safer and more practical than implementing full menu systems.

## Background Context

### Original User Request
- **Request**: "Add ability for user to change GUI skin/background color" (specifically black & white for screenshots)
- **Classification**: Theme/appearance customization
- **Strategic Insight**: This represents the first of likely many user preference requests

### Architectural Discovery
- **Current State**: BERT uses keyboard shortcuts exclusively (no menu system)
- **Technical Reality**: Complex Bevy-Leptos integration with 15+ event channels
- **Risk Assessment**: Full menu implementation = 78-117 hours with high breaking-change risk
- **Recommended Approach**: Settings modal overlay (4-7 hours with AI assistance, low risk)

## Research Objectives

### Primary Goal
Implement a **generalized settings infrastructure** that:
1. **Solves immediate need**: Black & white theme switching
2. **Provides foundation**: For future user preference requests
3. **Minimizes risk**: No changes to core Bevy-Leptos integration
4. **Follows patterns**: Leverages existing serialization and UI components

### Secondary Goals
- Establish settings persistence patterns
- Create extensible preference categories
- Maintain professional software standards
- Enable rapid addition of future settings

## Technical Architecture Analysis

### Current System Complexity
```rust
// Existing Bevy-Leptos Integration (DO NOT MODIFY)
pub fn init_bevy_app(
    selected_details_query: BevyQueryDuplex<(SystemElement,), With<SelectedHighlightHelperAdded>>,
    interface_details_query: BevyQueryDuplex<InterfaceQuery, SelectionFilter>,
    // ... 15+ complex query synchronizations
    tree_event_sender: BevyEventSender<TreeEvent>,
    trigger_event_receiver: BevyEventReceiver<TriggerEvent>,
) -> App
```

**Key Insight**: This integration is **highly specialized** and **tightly coupled**. Avoid modifications.

### Existing Infrastructure to Leverage

#### 1. Serialization Patterns
- **File**: `src/bevy_app/data_model/mod.rs` (1,184+ lines, 100% documented)
- **Pattern**: Proven `serde` serialization with `WorldModel`
- **Usage**: Extend for `UserPreferences` serialization

#### 2. File Operations
- **File**: `src/bevy_app/data_model/save.rs` & `load.rs`
- **Pattern**: Tauri file dialog integration
- **Usage**: Settings persistence to local storage/file

#### 3. UI Components
- **File**: `src/leptos_app/components/` (comprehensive component library)
- **Components**: `SelectGroup`, `Slider`, `Checkbox`, `Button`, etc.
- **Usage**: Settings form construction

#### 4. Resource Management
- **File**: `src/bevy_app/resources/`
- **Pattern**: Bevy resource system for global state
- **Usage**: `UserPreferences` as Bevy resource

## Recommended Implementation Strategy

### Phase 1: Settings Modal Infrastructure (2-4 hours with AI)

#### 1.1 Settings Data Structure
```rust
// src/bevy_app/resources/user_preferences.rs
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct UserPreferences {
    pub theme: ThemeSettings,
    pub interface: InterfaceSettings,
    pub performance: PerformanceSettings,
    pub accessibility: AccessibilitySettings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeSettings {
    pub scheme: ColorScheme,
    pub background_color: [f32; 3], // RGB
    pub contrast_mode: ContrastMode,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ColorScheme {
    Default,
    BlackAndWhite,
    HighContrast,    // Enhanced accessibility support
    Custom(CustomColors),
}
```

#### 1.2 Modal Component Architecture
```rust
// src/leptos_app/components/settings_modal.rs
#[component]
pub fn SettingsModal(
    show: RwSignal<bool>,
    preferences: RwSignal<UserPreferences>,
) -> impl IntoView {
    view! {
        <Show when=move || show.get()>
            // Modal overlay approach (NO layout changes to main app)
            <div class="fixed inset-0 z-50">
                <div class="absolute inset-0 bg-black bg-opacity-50"></div>
                <div class="relative flex items-center justify-center h-full p-4">
                    // Settings content using existing components
                </div>
            </div>
        </Show>
    }
}
```

#### 1.3 Integration Points
- **Trigger**: Settings button (⚙️) + `Ctrl+,` keyboard shortcut
- **State**: Leptos signals for UI reactivity
- **Persistence**: Extend existing file operations
- **Application**: Bevy resource updates

### Phase 2: Theme System Integration (1-2 hours with AI)

#### 2.1 Color Constants Extension
```rust
// src/bevy_app/constants.rs (EXTEND, don't break existing)
pub const CLEAR_COLOR_DEFAULT: Color = Color::srgb(0.98, 0.92, 0.84); // Current
pub const CLEAR_COLOR_BW: Color = Color::srgb(1.0, 1.0, 1.0); // White background

// Theme application system
fn apply_theme_settings(
    preferences: Res<UserPreferences>,
    mut clear_color: ResMut<ClearColor>,
) {
    if preferences.is_changed() {
        match preferences.theme.scheme {
            ColorScheme::BlackAndWhite => {
                clear_color.0 = CLEAR_COLOR_BW;
            },
            ColorScheme::Default => {
                clear_color.0 = CLEAR_COLOR_DEFAULT;
            },
        }
    }
}
```

#### 2.2 CSS Theme Integration
```css
/* styles.css - Extend existing patterns */
:root {
  --theme-background: #f6f6f6;
  --theme-text: #0f0f0f;
}

[data-theme="blackandwhite"] {
  --theme-background: #ffffff;
  --theme-text: #000000;
}
```

### Phase 3: Persistence Layer (30-60 minutes with AI)

#### 3.1 Settings File Operations
```rust
// Extend existing FileDialogPlugin pattern
FileDialogPlugin::new()
    .with_save_file::<JsonWorldData>()
    .with_load_file::<JsonWorldData>()
    .with_save_file::<JsonUserPreferences>()  // Add settings
    .with_load_file::<JsonUserPreferences>()
```

#### 3.2 Local Storage Integration
- **Web**: IndexedDB for robust settings persistence (preferred over localStorage)
- **Desktop**: Tauri app data directory
- **Pattern**: Follow existing file operation patterns
- **Versioning**: Include schema version for future migrations

## AI-Accelerated Development Timeline

### Revised Timeline Rationale

The original timeline estimates (14-21 hours total) were based on traditional development approaches. With modern AI tools like Cursor and Claude, these estimates can be significantly reduced due to:

#### AI Advantages for This Project
1. **Pattern Recognition**: BERT's well-structured codebase provides excellent patterns for AI to follow
2. **Boilerplate Generation**: AI excels at generating Rust struct definitions, component templates, and serialization code
3. **Code Extension**: Adding to existing files rather than architecting from scratch
4. **Documentation Standards**: AI can follow BERT's established documentation patterns

#### Updated Timeline Breakdown

| Phase | Traditional Estimate | AI-Assisted Estimate | Speedup Factor |
|-------|---------------------|---------------------|----------------|
| **Settings Modal Infrastructure** | 8-12 hours | **2-4 hours** | 3-4x faster |
| **Theme System Integration** | 4-6 hours | **1-2 hours** | 3-4x faster |
| **Persistence Layer** | 2-3 hours | **30-60 minutes** | 3-4x faster |
| **Total Project** | 14-21 hours | **4-7 hours** | **~3x faster overall** |

#### What AI Accelerates
- **Struct and enum definitions**: Instant generation following BERT patterns
- **Component boilerplate**: Modal components, form layouts, styling
- **Serialization code**: Extending existing save/load patterns
- **CSS and styling**: Theme variables and responsive design
- **Documentation**: Following BERT's comprehensive templates

#### What Still Requires Human Time
- **Visual design decisions**: UX refinement and aesthetic choices
- **Integration testing**: Ensuring cross-system compatibility
- **Cross-platform validation**: Web vs. desktop behavior
- **Performance optimization**: Fine-tuning and edge cases

#### Optimized Implementation Strategy

**Hours 1-2: Core Infrastructure (AI-Heavy)**
- AI generates complete `UserPreferences` structure
- AI creates `SettingsModal` component following BERT patterns
- Human: Integration testing and basic validation

**Hours 3-4: Theme System (AI-Assisted)**
- AI generates color constants and theme application systems
- AI creates CSS theme variables and responsive design
- Human: Visual testing and theme coordination

**Hours 5-6: Polish & Integration (Human-Led)**
- Human: Cross-platform testing and UX refinement
- AI assists: Bug fixes and optimizations
- Human: Edge case handling and performance validation

**Hour 7: Documentation (AI-Generated, Human-Reviewed)**
- AI generates documentation following BERT's templates
- Human: Final review and technical accuracy verification

### Success Criteria (Revised)

**4-Hour MVP Target**:
- ✅ Functional settings modal with theme switching
- ✅ Settings persistence across app restarts
- ✅ No breaking changes to existing functionality
- ✅ Basic documentation

**6-7 Hour Complete Implementation**:
- ✅ Multiple theme options (Default, B&W, High Contrast)
- ✅ Keyboard shortcuts integration (`Ctrl+,`)
- ✅ Comprehensive documentation following BERT standards
- ✅ Cross-platform testing and validation
- ✅ Performance optimization and edge case handling

## Research Focus Areas

### 1. Existing Component Analysis
**Objective**: Understand current UI component capabilities
**Files to Research**:
- `src/leptos_app/components/*.rs` (all component files)
- Focus on: form components, styling patterns, event handling

**Key Questions**:
- What form components exist and their capabilities?
- How do existing components handle state management?
- What styling patterns are established?

### 2. Bevy Resource System Integration
**Objective**: Understand how to add new global resources
**Files to Research**:
- `src/bevy_app/resources/` (all resource files)
- `src/bevy_app/mod.rs` (resource initialization patterns)

**Key Questions**:
- How are resources initialized and managed?
- What's the pattern for resource change detection?
- How do resources communicate with Leptos?

### 3. Event System Patterns
**Objective**: Understand Leptos ↔ Bevy communication
**Files to Research**:
- `src/bevy_app/events.rs`
- `src/leptos_app/mod.rs` (event channel setup)
- `src/bevy_app/mod.rs` (event system integration)

**Key Questions**:
- How are new event types added?
- What's the pattern for Leptos → Bevy communication?
- How are event receivers/senders created?

### 4. Serialization Patterns
**Objective**: Understand data persistence approaches
**Files to Research**:
- `src/bevy_app/data_model/save.rs`
- `src/bevy_app/data_model/load.rs`
- `src/bevy_app/data_model/mod.rs`

**Key Questions**:
- How is the current serialization system structured?
- What's the pattern for adding new serializable types?
- How does file persistence work across web/desktop?

### 5. Styling and Theme Infrastructure
**Objective**: Understand current styling approach
**Files to Research**:
- `styles.css`
- `src/leptos_app/components/` (component styling)
- `src/bevy_app/constants.rs` (color constants)

**Key Questions**:
- How are colors currently managed?
- What CSS patterns are established?
- How do Bevy and Leptos styling coordinate?

## Implementation Constraints

### What NOT to Modify
1. **Core Bevy-Leptos Integration**: `src/bevy_app/mod.rs` initialization
2. **Canvas Layout**: Main app layout in `src/leptos_app/mod.rs`
3. **Existing Event Channels**: Don't break current communication
4. **System Scheduling**: Don't modify existing SystemSets

### Safe Modification Areas
1. **Add new resources**: Extend resource initialization
2. **Add new components**: Create settings UI components
3. **Extend constants**: Add theme-related constants
4. **Add new events**: Create settings-specific events

## Success Criteria

### Minimum Viable Product
- [ ] Settings modal opens/closes correctly
- [ ] Black & white theme toggles background color
- [ ] Settings persist across app restarts
- [ ] No breaking changes to existing functionality

### Extended Success
- [ ] Multiple theme options (Default, B&W, High Contrast)
- [ ] Interface scaling options
- [ ] Performance settings
- [ ] Accessibility options
- [ ] Keyboard shortcut (`Ctrl+,`) integration
- [ ] Live theme preview (visual feedback)
- [ ] Settings versioning for future migrations
- [ ] Modular settings registration system

## Risk Mitigation

### High-Risk Areas to Avoid
1. **Canvas resizing**: Don't change main layout
2. **Event system overhaul**: Use existing patterns only
3. **Complex state synchronization**: Keep settings isolated
4. **Breaking keyboard shortcuts**: Maintain existing shortcuts

### Testing Strategy
1. **Incremental development**: Test each component separately
2. **Existing functionality**: Verify no regressions
3. **Cross-platform**: Test web and desktop builds
4. **User workflow**: Ensure settings don't disrupt modeling workflow

## Documentation Requirements

### Code Documentation
- Follow existing 100% documentation standards
- Use established documentation templates
- Include usage examples and integration patterns

### User Documentation
- Update keyboard shortcuts reference
- Add settings usage guide
- Document theme options and customization

## Conclusion

This research guide provides a comprehensive foundation for implementing BERT's settings infrastructure using a **safe, incremental approach**. The modal overlay strategy minimizes risk while providing immediate value and establishing patterns for future preference management.

**Key Success Factor**: Leverage existing, well-documented infrastructure rather than creating new architectural patterns. 