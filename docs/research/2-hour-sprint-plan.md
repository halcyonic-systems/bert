# 2-Hour Settings Sprint Plan

## 👥 **Context for Pair Programming Partner**

### The Situation
We're implementing BERT's first user preference feature - a black/white theme toggle for better screenshots. This is strategically important as it establishes the foundation for all future settings.

### Why This Focused Approach?
- **Original scope**: Comprehensive settings infrastructure (8-12 hours)
- **Sprint reality**: 2 hours available
- **Strategic choice**: Build minimal viable architecture that can be extended

### Technical Landscape You're Entering
**BERT** = Bevy (Rust game engine) + Leptos (Rust web framework)
- **Complex integration**: 15+ synchronized event channels between Bevy/Leptos
- **Existing patterns**: Well-established serialization, resource management, and UI components
- **Critical constraint**: Cannot modify core Bevy-Leptos bridge (high breaking risk)
- **Safe zones**: Can extend resources, constants, events, and add new systems

### Architecture Decision
**Modal overlay approach** instead of menu system integration:
- ✅ **Low risk**: No core integration changes needed
- ✅ **Fast**: Leverages existing component library
- ✅ **Extensible**: Foundation for comprehensive settings later
- ✅ **Learning focused**: Demonstrates key patterns without complexity

### Your Existing Advantages
- Strong Rust/systems experience
- Familiar with event-driven architectures  
- Can focus on BERT-specific patterns rather than language fundamentals

### What We're NOT Doing (Scope Boundaries)
- Full settings modal UI (future sprint)
- Settings persistence (future sprint)
- Multiple themes beyond B&W (future sprint)
- Menu system integration (architectural risk)

---

## 🎯 **Sprint Objective**
Get black/white theme toggle working with minimal infrastructure to validate architecture approach.

## 📊 **Time Budget**
- **Phase 1**: Core Infrastructure (45 minutes)
- **Phase 2**: Theme Toggle (45 minutes) 
- **Phase 3**: UI Integration (20 minutes)
- **Phase 4**: Testing (10 minutes)

---

## Phase 1: Core Infrastructure (45 minutes)

### 1.1 Minimal UserPreferences Resource (15 minutes)
**File**: `src/bevy_app/resources/user_preferences.rs`
```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserPreferences {
    pub theme: ThemeSettings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeSettings {
    pub scheme: ColorScheme,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum ColorScheme {
    #[default]
    Default,
    BlackAndWhite,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            scheme: ColorScheme::Default,
        }
    }
}
```

### 1.2 Theme Constants (10 minutes)
**Extend**: `src/bevy_app/constants.rs`
```rust
// Add after existing CLEAR_COLOR
pub const CLEAR_COLOR_DEFAULT: Color = Color::srgb(0.98, 0.92, 0.84);
pub const CLEAR_COLOR_BW: Color = Color::srgb(1.0, 1.0, 1.0);
```

### 1.3 Settings Event (10 minutes)
**Extend**: `src/bevy_app/events.rs`
```rust
// Add to TriggerEvent enum
ToggleTheme,
```

### 1.4 Resource Registration (10 minutes)
**Extend**: `src/bevy_app/mod.rs`
```rust
// Add to app initialization
.init_resource::<UserPreferences>()
```

---

## Phase 2: Theme Toggle System (45 minutes)

### 2.1 Theme Application System (25 minutes)
**File**: `src/bevy_app/systems/theme.rs`
```rust
use bevy::prelude::*;
use crate::bevy_app::{
    resources::{UserPreferences, ColorScheme},
    constants::{CLEAR_COLOR_DEFAULT, CLEAR_COLOR_BW},
    events::TriggerEvent,
};

pub fn apply_theme_system(
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

pub fn handle_theme_toggle(
    mut preferences: ResMut<UserPreferences>,
    mut trigger_events: EventReader<TriggerEvent>,
) {
    for event in trigger_events.read() {
        if let TriggerEvent::ToggleTheme = event {
            preferences.theme.scheme = match preferences.theme.scheme {
                ColorScheme::Default => ColorScheme::BlackAndWhite,
                ColorScheme::BlackAndWhite => ColorScheme::Default,
            };
        }
    }
}
```

### 2.2 System Registration (10 minutes)
**Extend**: `src/bevy_app/systems/mod.rs`
```rust
pub mod theme;
pub use theme::*;
```

### 2.3 Add to App (10 minutes)
**Extend**: `src/bevy_app/mod.rs`
```rust
// Add to Update systems
(apply_theme_system, handle_theme_toggle),
```

---

## Phase 3: UI Integration (20 minutes)

### 3.1 Simple Theme Toggle Button (20 minutes)
**Extend**: `src/leptos_app/mod.rs`
Add a basic button that sends theme toggle event:

```rust
// Add button to main UI
<button 
    on:click=move |_| {
        trigger_event_sender.send(TriggerEvent::ToggleTheme);
    }
    class="fixed top-4 right-4 z-50 p-2 bg-gray-200 rounded"
>
    "🎨 Theme"
</button>
```

---

## Phase 4: Testing (10 minutes)

### 4.1 Manual Testing Checklist
- [ ] App starts with default theme
- [ ] Button click toggles to black/white
- [ ] Button click toggles back to default
- [ ] No crashes or errors
- [ ] Background color changes are visible

---

## 🔑 **Success Criteria for 2-Hour Sprint**

### Must Have
- [ ] Theme toggle button visible in UI
- [ ] Button successfully changes background color
- [ ] No breaking changes to existing functionality

### Nice to Have (if time permits)
- [ ] Keyboard shortcut (Ctrl+T)
- [ ] Visual feedback on button state
- [ ] Settings persistence (basic)

---

## 🚀 **Next Sprint Preparation**

After this 2-hour sprint, the next development cycles can focus on:

1. **Settings Modal UI** (full modal interface)
2. **Persistence Layer** (save/load settings)
3. **Additional Themes** (high contrast, custom colors)
4. **Settings Categories** (interface, performance, accessibility)

---

## 🎓 **Learning Objectives (Systems Architecture)**

This sprint demonstrates:

1. **Resource Management**: How Bevy resources provide global state
2. **Event-Driven Architecture**: Communication between UI and engine
3. **System Composition**: Adding new systems without breaking existing ones
4. **Incremental Development**: Building minimal viable features first
5. **Constraint Management**: Working within existing architectural boundaries

---

## 🔧 **Implementation Order**

1. **Start with data structures** - Define the types first
2. **Add business logic** - Theme switching system
3. **Wire up events** - Communication layer
4. **Add UI trigger** - User interaction point
5. **Test incrementally** - Verify each piece works

This approach follows good systems thinking: define the system boundaries, establish the interfaces, implement the core logic, then add the user interface.

---

## 🗂️ **Key Files You'll Touch**

### New Files (Create)
- `src/bevy_app/resources/user_preferences.rs` - Settings data structure
- `src/bevy_app/systems/theme.rs` - Theme application logic

### Existing Files (Extend)
- `src/bevy_app/constants.rs` - Add theme color constants
- `src/bevy_app/events.rs` - Add ToggleTheme event
- `src/bevy_app/mod.rs` - Register new resource and systems
- `src/bevy_app/systems/mod.rs` - Export theme module
- `src/leptos_app/mod.rs` - Add theme toggle button

### Reference Files (Don't Modify)
- `src/bevy_app/data_model/` - Serialization patterns to follow later
- `src/leptos_app/components/` - UI components for future modal
- Comprehensive research docs for post-sprint expansion

---

## 🚨 **Potential Gotchas for Experienced Developer**

1. **Event System**: BERT uses custom `TriggerEvent` enum, not standard Bevy events
2. **Color Constants**: Background uses `ClearColor` resource, not CSS
3. **Module Structure**: Bevy resources live in `resources/`, systems in `systems/`
4. **Import Paths**: Complex nested module structure - follow existing patterns
5. **Bevy Change Detection**: `preferences.is_changed()` is key for performance

Ready to dive in! 🚀 