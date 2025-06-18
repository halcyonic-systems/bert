# Getting Started with BERT for Systems Scientists

This guide helps systems scientists with basic coding skills understand and modify BERT.

## Understanding BERT's Purpose

BERT (Bounded Entity Reasoning Toolkit) implements a formal System Language (SL) to model complex adaptive systems. It allows you to:

1. Visually represent system components, flows, and relationships
2. Decompose systems into hierarchical layers
3. Capture structured knowledge in a standardized format
4. Save and share system models

## Key Concepts Mapped to Code

Each concept from systems science has a direct implementation in code:

| Systems Concept | BERT Implementation | File Location |
|-----------------|---------------------|--------------|
| System | `SystemComponent` | `src/bevy_app/components/system_elements.rs` |
| Subsystem | `SubsystemComponent` | `src/bevy_app/components/system_elements.rs` |
| Flow | `FlowComponent` | `src/bevy_app/components/connections.rs` |
| Interface | `InterfaceComponent` | `src/bevy_app/components/connections.rs` |
| Boundary | Implicit in visual container | `src/bevy_app/bundles/spawn/` |

## Making Simple Changes (With Examples)

### Example 1: Modifying System Element Properties

If you want to add a new property to system elements (like "criticality" or "resilience"):

1. Locate the system element definition:
   ```rust
   // In src/bevy_app/components/system_elements.rs
   #[derive(Component, Reflect, Debug, Clone)]
   pub struct SystemComponent {
       pub name: String,
       pub description: String,
       // Add your new property here
       pub criticality: f32, // Example addition
   }
   ```

2. Update the UI component to edit this property:
   ```rust
   // In src/leptos_app/details.rs (find the system element editor)
   // Add a new slider for criticality
   <Slider
       label="Criticality"
       min=0.0
       max=10.0
       step=0.1
       value=criticality
       on_change=move |v| set_criticality(v)
   />
   ```

3. Ensure it's saved in the data model:
   ```rust
   // In src/bevy_app/data_model/save.rs
   // Make sure the new property is included in serialization
   ```

### Example 2: Changing Visual Representation

To change how a system element looks:

1. Find the spawning code:
   ```rust
   // In src/bevy_app/bundles/spawn/main_system.rs
   // Look for the shape definition
   ```

2. Modify the shape parameters (size, color, etc.)

### Example 3: Adding a New System Element Type

To add a completely new element type (like "Buffer"):

1. Define the new component:
   ```rust
   // In src/bevy_app/components/system_elements.rs
   #[derive(Component, Reflect, Debug, Clone)]
   pub struct BufferComponent {
       pub name: String,
       pub capacity: f32,
   }
   ```

2. Create a spawning bundle:
   ```rust
   // Create a new file: src/bevy_app/bundles/spawn/buffer.rs
   // Define the bundle for spawning this element
   ```

3. Add a UI button:
   ```rust
   // In src/leptos_app/components/button.rs
   // Add a new button for creating buffers
   ```

## Using LLM-Assisted Development

When making changes, you can use Claude or similar LLMs to help:

1. **Understand code**: "Explain how the SystemComponent in BERT works"
2. **Generate snippets**: "Create a new property for tracking system reliability"
3. **Debug issues**: "Why isn't my new buffer element appearing on the canvas?"
4. **Plan changes**: "How should I implement a time delay property for flows?"

## Assigning Tasks to Engineers

For more complex changes, create detailed task descriptions:

```
Task: Implement System Stability Metrics

Description: Add functionality to calculate and visualize system stability 
based on the interconnections and feedback loops.

Technical Requirements:
1. Create a StabilityMetrics component in system_elements.rs
2. Implement an algorithm to assess feedback loop stability
3. Add visualization cues (e.g., color coding) for stability indicators
4. Update the UI to display stability metrics

Files to modify:
- src/bevy_app/components/system_elements.rs
- src/bevy_app/systems/ui/color.rs
- src/leptos_app/details.rs
```

## Next Steps

Once you're comfortable with basic modifications:

1. Explore the [Comprehensive Architecture Overview](../architecture/comprehensive-architecture-overview.md)
2. Look at the [Contributing Guidelines](../contributing/guidelines.md)
3. Try [System Modeling Tutorials](../tutorials/basic-system-modeling.md)

## Tips for Efficient Usage

- Use multi-select (Shift+click) to manipulate multiple elements simultaneously
- Hide elements (H key) to reduce visual complexity when working on specific parts of a system
- Use Ctrl+S frequently to save your work
- For complex systems, build hierarchically from top-down, decomposing subsystems as needed
- Create a backup of important models by using "Save As" with a new filename

## Creating Professional Documentation

When preparing diagrams for academic papers, presentations, or formal documentation:

- Use **Ctrl+Alt+B** to toggle to a clean white background for better contrast in printed materials
- The white background theme provides a professional appearance suitable for:
  - Academic publications and research papers
  - Professional presentations and reports
  - Screenshots for documentation
  - High-contrast diagrams for accessibility

Switch back to the normal beige background anytime using the same **Ctrl+Alt+B** shortcut for regular modeling work.