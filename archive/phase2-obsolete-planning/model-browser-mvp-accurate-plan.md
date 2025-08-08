# Model Browser MVP - Accurate Implementation Plan

## Branch Setup
```bash
git checkout -b feature/model-browser
./scripts/gen-feature-docs.sh "Model Browser"
```

## Key Insights from Codebase Analysis

### Current File Loading Flow
1. **Trigger**: Ctrl+L (or Cmd+L on Mac) triggers file dialog
2. **Event**: `LoadFileEvent { file_path: String, data: Vec<u8> }`
3. **Processing**: `load_from_bytes()` deserializes JSON to `WorldModel`
4. **Rendering**: Complex spawning of entities, interfaces, interactions

### UI Pattern (from ControlsMenu)
- Uses Tailwind CSS classes
- Component takes `Signal<bool>` for visibility
- Uses `Callback<()>` for close action
- Clean modal overlay pattern with proper event handling

### Integration Points
- Main toolbar buttons in `leptos_app/mod.rs` (~line 90)
- Components in `leptos_app/components/`
- File loading through `LoadFileEvent` 

## MVP Implementation Plan (2-3 hours)

### Step 1: Create Model Browser Component (45 min)
**File**: `src/leptos_app/components/model_browser.rs`

```rust
use leptos::prelude::*;
use crate::LoadFileEvent;

#[component]
pub fn ModelBrowser(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_load: Callback<LoadFileEvent>,
) -> impl IntoView {
    // For MVP: Hardcode 3 example models
    let models = vec![
        ("Simple Cell", "cell", include_bytes!("../../../assets/models/cell.json")),
        ("Organization", "org", include_bytes!("../../../assets/models/organization.json")),
        ("Circuit", "circuit", include_bytes!("../../../assets/models/circuit.json")),
    ];

    view! {
        <Show when=move || visible.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-30 flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-xl max-w-3xl max-h-[80vh] m-4 p-6">
                    <div class="flex justify-between items-center mb-6">
                        <h2 class="text-2xl font-bold text-gray-900">"Model Browser"</h2>
                        <button
                            class="text-gray-400 hover:text-gray-600 text-2xl font-bold"
                            on:click=move |_| on_close.run(())
                        >
                            "×"
                        </button>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        {models.into_iter().map(|(name, id, data)| {
                            let data = data.to_vec();
                            let file_path = format!("{}.json", id);
                            view! {
                                <button
                                    class="p-4 border rounded-lg hover:bg-gray-50 text-left"
                                    on:click=move |_| {
                                        on_load.run(LoadFileEvent {
                                            file_path: file_path.clone(),
                                            data: data.clone(),
                                        });
                                        on_close.run(());
                                    }
                                >
                                    <h3 class="font-semibold text-gray-800">{name}</h3>
                                    <p class="text-sm text-gray-600 mt-1">"Click to load example"</p>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    
                    <div class="mt-6 text-center text-sm text-gray-600">
                        "More models coming soon. Press Ctrl+L to load your own files."
                    </div>
                </div>
            </div>
        </Show>
    }
}
```

### Step 2: Add to Components Module (5 min)
**File**: `src/leptos_app/components/mod.rs`

Add:
```rust
mod model_browser;
pub use model_browser::ModelBrowser;
```

### Step 3: Integrate into Main App (20 min)
**File**: `src/leptos_app/mod.rs`

1. Add import:
```rust
use crate::leptos_app::components::ModelBrowser;
```

2. Add signal (near line 59):
```rust
let (model_browser_visible, set_model_browser_visible) = signal(false);
```

3. Add button in toolbar (near line 90):
```rust
<button
    class="menu-button"
    on:click=move |_| set_model_browser_visible.set(true)
>
    "Model Browser"
</button>
```

4. Add component (near ControlsMenu):
```rust
<ModelBrowser 
    visible=model_browser_visible
    on_close=Callback::new(move |_| set_model_browser_visible.set(false))
    on_load=Callback::new(move |event: LoadFileEvent| {
        load_file_writer.add(event);
    })
/>
```

### Step 4: Create Example Models (30 min)
**Directory**: `assets/models/`

Create simple, valid JSON models based on WorldModel structure:

**cell.json**:
```json
{
  "systems": [{
    "info": {
      "id": {"ty": "System", "indices": [0]},
      "name": "Cell",
      "description": "A simple biological cell",
      "level": 0
    },
    "radius": 200.0,
    "complexity": 5,
    "boundary": {
      "info": {"id": {"ty": "Boundary", "indices": [0]}, "name": "Cell Membrane", "description": "Selectively permeable membrane"},
      "porosity": 0.3,
      "perceptive_fuzziness": 0.1,
      "interfaces": []
    }
  }],
  "interactions": [],
  "environment": {"info": {"id": {"ty": "Environment", "indices": []}, "name": "Environment", "description": "", "level": -1}, "sources": [], "sinks": []},
  "hidden_entities": [],
  "is_same_as_id_counter": 0
}
```

### Step 5: Test & Polish (30 min)
1. Run `cargo tauri dev` for desktop testing
2. Run `trunk serve` for web testing
3. Verify:
   - Model Browser button appears
   - Modal opens/closes properly
   - Models load when clicked
   - No regression to Ctrl+L functionality

## Fallback Strategies

### If include_bytes! doesn't work:
- Use string literals with escaped JSON
- Load from static files at runtime
- Start with UI only, add loading later

### If integration is complex:
- Start with just the button and modal UI
- Use existing Ctrl+L for actual loading
- Add loading functionality in next iteration

## Next Steps After MVP
1. Dynamic model loading from files
2. Model metadata and previews
3. Categories and search
4. User model library
5. Remote model repository

## Success Criteria
- [ ] Model Browser button visible
- [ ] Modal appears with 3 example models
- [ ] Clicking a model loads it
- [ ] Works on both desktop and web
- [ ] Feature documentation updated

---
*This plan is based on actual codebase patterns and should integrate smoothly.*