# Settings Implementation Roadmap

## Overview

This roadmap provides a step-by-step implementation plan for BERT's settings infrastructure, based on the comprehensive research in [`settings-infrastructure-research-guide.md`](settings-infrastructure-research-guide.md).

## Implementation Strategy: Settings Modal Approach

**Target**: 8-12 hours total implementation time with AI agents
**Risk Level**: Low (avoids core Bevy-Leptos integration changes)

---

## Phase 1: Core Infrastructure (3-4 hours)

### 1.1 UserPreferences Resource
**File**: `src/bevy_app/resources/user_preferences.rs`
```rust
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct UserPreferences {
    pub theme: ThemeSettings,
    pub interface: InterfaceSettings,
    pub performance: PerformanceSettings,
}
```

### 1.2 Settings Modal Component  
**File**: `src/leptos_app/components/settings_modal.rs`
- Modal overlay with tab navigation
- Theme selection (Default, Black & White, High Contrast)
- Settings persistence controls
- Keyboard shortcut support (`Ctrl+,`)

### 1.3 Event Integration
**Extend**: `src/bevy_app/events.rs`
- Add `SettingsUpdateEvent` to `TriggerEvent` enum
- Handle settings changes in `react_to_trigger_event`

---

## Phase 2: Theme System (2-3 hours)

### 2.1 Theme Constants
**Extend**: `src/bevy_app/constants.rs`
```rust
pub struct ThemeColors {
    pub background: Color,
    pub text: Color,
    pub accent: Color,
}

pub const DEFAULT_THEME: ThemeColors = ThemeColors { /* ... */ };
pub const BLACK_WHITE_THEME: ThemeColors = ThemeColors { /* ... */ };
```

### 2.2 Theme Application System
**File**: `src/bevy_app/systems/ui/theme.rs`
- Apply theme colors to UI elements
- Update CSS variables for Leptos components
- Handle theme transitions

### 2.3 CSS Variables
**Extend**: `styles.css`
```css
:root {
    --bert-bg-color: var(--theme-background);
    --bert-text-color: var(--theme-text);
    --bert-accent-color: var(--theme-accent);
}
```

---

## Phase 3: Persistence Layer (2-3 hours)

### 3.1 Settings Serialization
**File**: `src/bevy_app/data_model/settings.rs`
- Extend existing serialization patterns
- Settings file format and versioning
- Migration system for future changes

### 3.2 Local Storage Integration
**Extend**: `src/bevy_app/plugins/file_dialog.rs`
- Settings save/load operations
- Browser localStorage for web builds
- Tauri app data directory for desktop

### 3.3 Startup/Shutdown Systems
- Load settings on app initialization
- Auto-save on settings changes
- Graceful fallback to defaults

---

## Phase 4: UI Integration (1-2 hours)

### 4.1 Settings Button
**Extend**: `src/leptos_app/mod.rs`
- Add settings gear icon to main UI
- Position in top-right corner
- Keyboard shortcut registration

### 4.2 Modal State Management
- Settings modal open/close state
- Tab navigation within modal
- Form validation and feedback

### 4.3 Live Preview
- Real-time theme preview
- Settings changes without modal close
- Revert/apply confirmation

---

## Implementation Checklist

### Core Infrastructure
- [ ] Create `UserPreferences` resource
- [ ] Implement settings modal component
- [ ] Add settings events to trigger system
- [ ] Register new systems in app setup

### Theme System  
- [ ] Define theme color constants
- [ ] Create theme application systems
- [ ] Update CSS with theme variables
- [ ] Test theme switching functionality

### Persistence
- [ ] Implement settings serialization
- [ ] Add local storage integration
- [ ] Create startup settings loading
- [ ] Add auto-save on changes

### UI Integration
- [ ] Add settings button to main UI
- [ ] Implement modal open/close
- [ ] Add keyboard shortcut support
- [ ] Create settings form validation

### Testing & Polish
- [ ] Test all theme options
- [ ] Verify settings persistence
- [ ] Test keyboard shortcuts
- [ ] Cross-platform compatibility check

---

## Key Constraints (From Research)

1. **No Core Integration Changes**: Avoid modifying Bevy-Leptos bridge
2. **Modal Overlay Only**: Don't change canvas layout
3. **Existing Patterns**: Use established event and resource patterns
4. **Progressive Enhancement**: Start with basic functionality, add features

---

## Success Criteria

- [ ] User can open settings with `Ctrl+,` or button click
- [ ] Black & white theme available for screenshots
- [ ] Settings persist across app restarts
- [ ] No impact on existing functionality
- [ ] Clean, intuitive user interface

---

## Next Steps

1. **Start with Phase 1**: Core infrastructure provides foundation
2. **Use AI Agents**: Leverage Cursor/Claude Code for parallel implementation
3. **Test Incrementally**: Verify each phase before proceeding
4. **Reference Research**: Use research guide for detailed technical decisions

This roadmap transforms the research into actionable implementation steps, ready for AI-accelerated development. 