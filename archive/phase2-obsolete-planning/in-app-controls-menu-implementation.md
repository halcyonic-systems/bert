# In-App Controls Menu Implementation Research

## Executive Summary

This document provides a comprehensive analysis and implementation plan for adding a basic in-app "controls" menu to BERT (Bounded Entity Reasoning Toolkit). Based on analysis of the existing codebase architecture, this feature is assessed as **Easy to Moderate** difficulty due to BERT's well-structured Leptos UI framework and existing component patterns.

**Key Finding**: A basic controls menu can be implemented in approximately **30 minutes to 1 hour** by leveraging existing UI patterns, with more advanced features requiring 2-6 hours of development time.

## Architecture Analysis

### Current UI Infrastructure

BERT's frontend is built on a solid foundation that makes UI extensions straightforward:

#### 🏗️ **Leptos Component System**
- **Location**: `src/leptos_app/components/`
- **Existing Components**: `Button`, `InputGroup`, `Divider`, `Slider`, `Checkbox`, `SelectGroup`
- **Pattern**: Modular, reusable components with consistent styling

#### 📱 **Slide Panel Pattern**
- **Reference Implementation**: `src/leptos_app/details.rs`
- **Features**: Slide-in animation, responsive design, z-index management
- **CSS Classes**: Tailwind-based with consistent design system

#### 🔗 **Event System**
- **Architecture**: Clean Leptos ↔ Bevy communication
- **Pattern**: `event_l2b` (Leptos to Bevy) and `event_b2l` (Bevy to Leptos)
- **Usage**: File operations, tree toggles, drag events

### Existing Control Patterns

BERT already implements several UI controls that serve as templates:

```rust
// Tree Toggle Button (src/leptos_app/mod.rs:67-85)
<Show when=move || tree_visible.get()>
    <button class="px-4 py-2 rounded-lg bg-white absolute top-4 left-4 z-20">
        {"Hide Tree"}
    </button>
</Show>

// Details Panel (src/leptos_app/details.rs:75-155)
<div class="fixed inset-y-0 right-0 pl-10 max-w-full pointer-events-none">
    <div class="w-screen max-w-md transition duration-500 ease-in-out transform">
        // Panel content
    </div>
</div>
```

## Implementation Approaches

### Approach 1: Quick Implementation (30 minutes)

**Minimal viable controls overlay** that can be added immediately:

```rust
// Add to App component in src/leptos_app/mod.rs
let (show_controls, set_show_controls) = signal(false);

// Help button (add after tree button)
<button
    class="px-3 py-2 rounded bg-white absolute top-4 left-32 z-20 text-sm hover:bg-gray-50"
    on:click=move |_| set_show_controls.set(!show_controls.get())
>
    {"?"} // Help icon
</button>

// Simple overlay
<Show when=move || show_controls.get()>
    <div class="fixed top-16 left-4 z-30 p-4 bg-white rounded-lg shadow-lg max-w-xs border">
        <div class="flex justify-between items-center mb-3">
            <h3 class="font-bold text-gray-900">{"Quick Controls"}</h3>
            <button 
                on:click=move |_| set_show_controls.set(false)
                class="text-gray-400 hover:text-gray-600"
            >
                {"×"}
            </button>
        </div>
        <div class="text-sm space-y-2 text-gray-700">
            <div class="flex justify-between">
                <span>{"Zoom In"}</span>
                <kbd class="px-2 py-1 bg-gray-100 rounded text-xs">{"="}</kbd>
            </div>
            <div class="flex justify-between">
                <span>{"Zoom Out"}</span>
                <kbd class="px-2 py-1 bg-gray-100 rounded text-xs">{"-"}</kbd>
            </div>
            <div class="flex justify-between">
                <span>{"Pan Canvas"}</span>
                <kbd class="px-2 py-1 bg-gray-100 rounded text-xs">{"Right-click + drag"}</kbd>
            </div>
            <div class="flex justify-between">
                <span>{"Save"}</span>
                <kbd class="px-2 py-1 bg-gray-100 rounded text-xs">{"Ctrl+S"}</kbd>
            </div>
            <div class="flex justify-between">
                <span>{"Select Element"}</span>
                <kbd class="px-2 py-1 bg-gray-100 rounded text-xs">{"Left-click"}</kbd>
            </div>
        </div>
    </div>
</Show>
```

### Approach 2: Full Component Implementation (2-3 hours)

**Structured component following BERT patterns**:

```rust
// src/leptos_app/controls.rs
use leptos::prelude::*;
use crate::leptos_app::components::Divider;

#[component]
pub fn ControlsMenu(
    #[prop(into)] visible: Signal<bool>,
    set_visible: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            // Backdrop
            <div 
                class="fixed inset-0 z-20 bg-black bg-opacity-25"
                on:click=move |_| set_visible.set(false)
            ></div>
            
            // Menu Panel
            <div class="fixed top-0 left-0 z-30 w-80 h-screen bg-white shadow-xl transform transition-transform duration-300">
                <div class="flex flex-col h-full">
                    // Header
                    <div class="flex justify-between items-center p-4 border-b">
                        <h2 class="text-lg font-semibold text-gray-900">{"Controls & Shortcuts"}</h2>
                        <button 
                            on:click=move |_| set_visible.set(false)
                            class="text-gray-400 hover:text-gray-600 text-xl"
                        >
                            {"×"}
                        </button>
                    </div>
                    
                    // Content
                    <div class="flex-1 overflow-y-auto p-4 space-y-4">
                        <ControlSection 
                            title="Navigation"
                            controls=vec![
                                ("Zoom In", "="),
                                ("Zoom Out", "-"),
                                ("Pan Canvas", "Right-click + drag"),
                                ("Reset View", "Ctrl+R"),
                            ]
                        />
                        
                        <ControlSection 
                            title="Selection"
                            controls=vec![
                                ("Select Element", "Left-click"),
                                ("Multi-select", "Shift + click"),
                                ("Deselect All", "Escape"),
                            ]
                        />
                        
                        <ControlSection 
                            title="Element Management"
                            controls=vec![
                                ("Delete Selected", "Delete/Backspace"),
                                ("Hide Selected", "H"),
                                ("Unhide All", "U"),
                                ("Move Elements", "Click + drag"),
                            ]
                        />
                        
                        <ControlSection 
                            title="File Operations"
                            controls=vec![
                                ("Save", "Ctrl+S"),
                                ("Open File", "Ctrl+L"),
                            ]
                        />
                    </div>
                    
                    // Footer
                    <div class="p-4 border-t bg-gray-50">
                        <p class="text-xs text-gray-500 text-center">
                            {"For complete documentation, see the user guide"}
                        </p>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn ControlSection(
    title: &'static str, 
    controls: Vec<(&'static str, &'static str)>
) -> impl IntoView {
    view! {
        <div>
            <Divider name=title />
            <div class="space-y-2">
                <For
                    each=move || controls.clone()
                    key=|(action, _)| action.to_string()
                    children=|(action, shortcut)| {
                        view! {
                            <div class="flex justify-between items-center py-1">
                                <span class="text-sm text-gray-700">{action}</span>
                                <kbd class="px-2 py-1 text-xs bg-gray-100 text-gray-600 rounded border border-gray-200 font-mono">
                                    {shortcut}
                                </kbd>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
```

### Approach 3: Interactive Controls Panel (4-6 hours)

**Advanced implementation with interactive features**:

Features to add:
- **Toggle-able settings** (grid display, snap-to-grid, auto-save)
- **Zoom level indicator** with slider control
- **Element count display** (systems, flows, interfaces)
- **Theme switching** (light/dark mode)
- **Export options** (PNG, SVG, PDF)

```rust
// Additional interactive components
#[component]
fn InteractiveControls() -> impl IntoView {
    let (auto_save, set_auto_save) = signal(true);
    let (show_grid, set_show_grid) = signal(false);
    let (zoom_level, set_zoom_level) = signal(100.0);
    
    view! {
        <div class="space-y-4">
            <Divider name="Settings" />
            
            <Checkbox
                id="auto-save"
                label="Auto-save enabled"
                checked=auto_save
                on_toggle=move |checked| set_auto_save.set(checked)
            />
            
            <Checkbox
                id="show-grid"
                label="Show grid"
                checked=show_grid
                on_toggle=move |checked| set_show_grid.set(checked)
            />
            
            <div>
                <label class="text-sm font-medium text-gray-700">{"Zoom Level"}</label>
                <Slider
                    id="zoom-control"
                    label=""
                    value=zoom_level
                    min=25.0.into()
                    max=400.0.into()
                    step=5.0.into()
                    on_input=move |value| set_zoom_level.set(value)
                />
                <span class="text-xs text-gray-500">{move || format!("{}%", zoom_level.get() as u32)}</span>
            </div>
        </div>
    }
}
```

## Integration Points

### File Structure

```
src/leptos_app/
├── components/
│   ├── mod.rs              # Export ControlsMenu
│   └── controls_menu.rs    # New component
├── mod.rs                  # Main app integration
└── details.rs              # Reference implementation
```

### CSS Integration

BERT uses **Tailwind CSS** with consistent design patterns:

```css
/* Consistent with existing panels */
.controls-panel {
  @apply fixed bg-white shadow-xl z-30;
  @apply transition-transform duration-300 ease-in-out;
}

/* Consistent with existing buttons */
.controls-button {
  @apply px-4 py-2 rounded-lg bg-white absolute z-20;
  @apply hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-cyan-500;
}
```

### Event Integration

```rust
// Add to existing event system if needed
#[derive(Event, Debug, Clone)]
pub enum ControlsEvent {
    ToggleGrid,
    SetZoom(f32),
    ToggleAutoSave,
}

// In mod.rs
let (controls_event_sender, controls_event_receiver) = event_l2b::<ControlsEvent>();
```

## Implementation Timeline

### Phase 1: Basic Controls Overlay (30 minutes)
- [ ] Add help button to main UI
- [ ] Create simple overlay with key shortcuts
- [ ] Test basic show/hide functionality

### Phase 2: Structured Component (2-3 hours)
- [ ] Create `src/leptos_app/components/controls_menu.rs`
- [ ] Implement `ControlsMenu` component
- [ ] Add proper styling and animations
- [ ] Integrate with main app layout

### Phase 3: Enhanced Features (4-6 hours)
- [ ] Add interactive controls (checkboxes, sliders)
- [ ] Implement Bevy integration for settings
- [ ] Add status displays (zoom level, element counts)
- [ ] Create export/import shortcuts

### Phase 4: Advanced Features (1-2 days)
- [ ] Theme switching capabilities
- [ ] Custom keyboard shortcut configuration
- [ ] Search and filter functionality
- [ ] Recent files quick access

## Technical Considerations

### Performance Impact

- **Minimal**: Static menu content with simple state management
- **Memory**: ~5-10KB additional bundle size for basic implementation
- **Rendering**: Only rendered when visible (Leptos `<Show>` component)

### Accessibility

```rust
// Ensure accessibility compliance
view! {
    <div 
        role="dialog"
        aria-labelledby="controls-title"
        aria-modal="true"
    >
        <h2 id="controls-title" class="sr-only">{"Controls Menu"}</h2>
        // ... content
    </div>
}
```

### Mobile Responsiveness

```rust
// Responsive design for different screen sizes
<div class="w-80 sm:w-96 md:w-80 lg:w-96">
    // Adjust width based on screen size
</div>
```

## Testing Strategy

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_controls_menu_visibility() {
        // Test show/hide state management
    }
    
    #[test]
    fn test_shortcut_display() {
        // Verify all shortcuts are displayed correctly
    }
}
```

### Integration Testing
- Verify menu doesn't interfere with existing UI
- Test keyboard shortcuts still work with menu open
- Confirm responsive behavior on different screen sizes

### User Testing
- Validate shortcut descriptions match actual behavior
- Ensure menu is discoverable and intuitive
- Test accessibility with screen readers

## Risk Assessment

### Low Risk
- **UI conflicts**: Well-isolated z-index management
- **Performance**: Minimal computational overhead
- **Compatibility**: Uses existing component patterns

### Mitigation Strategies
- **Regression testing**: Verify existing functionality unchanged
- **Feature flags**: Allow disabling if issues arise
- **Graceful degradation**: Menu failure doesn't affect core functionality

## Future Enhancements

### Short-term (Next Sprint)
- **Keyboard shortcut for menu toggle** (e.g., `F1` or `?`)
- **Tooltips on hover** for additional context
- **Recent actions history** (undo/redo hints)

### Medium-term (Next Month)
- **Customizable shortcuts** with user preferences
- **Context-sensitive help** based on current selection
- **Interactive tutorials** with guided workflows

### Long-term (Next Quarter)
- **AI-powered suggestions** for common workflows
- **Collaborative features** (shared shortcuts, team templates)
- **Advanced analytics** (usage patterns, optimization suggestions)

## Conclusion

Implementing an in-app controls menu in BERT is highly feasible due to:

1. **Solid Architecture**: Leptos component system provides excellent foundation
2. **Existing Patterns**: Multiple reference implementations (Details panel, Tree toggle)
3. **Low Risk**: Well-isolated feature with minimal system impact
4. **High Value**: Significant UX improvement for minimal development cost

**Recommendation**: Start with **Approach 1** for immediate value, then evolve to **Approach 2** for a polished experience. The modular architecture makes incremental enhancement straightforward.

**Next Steps**:
1. Implement basic overlay (30 minutes)
2. Gather user feedback on content and positioning
3. Evolve to full component based on usage patterns
4. Consider advanced features based on user requests

This feature represents an excellent opportunity to enhance BERT's usability while leveraging the existing robust architecture and demonstrating the extensibility of the Leptos-based UI framework. 