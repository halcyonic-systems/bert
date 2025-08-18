# Controls and Interface

## Essential Controls

### Navigation
- **Pan**: Right-click and drag
- **Zoom in**: Press `=` 
- **Zoom out**: Press `-`
- **Reset view**: Press `Ctrl+R`

### Selection
- **Select element**: Left-click
- **Multi-select**: Hold `Shift` + click
- **Deselect**: Press `Escape` or click empty space

### Creation
- **Add elements**: Click toolbar button, then click canvas
- **Connect elements**: Click source, drag to target, release
- **Delete**: Select element + press `Delete`

### File Operations
- **Save**: Press `Ctrl+S` (watch for green notification)
- **Open**: Press `Ctrl+L`
- **Save As**: Press `Ctrl+Shift+S`

## Working with Elements

### Element Types
- **System**: Main analysis boundary (large circle)
- **Subsystem**: Internal component (smaller circle inside system)
- **Interface**: Connection point (rectangle on boundary)
- **Flow**: Connection between elements (arrow line)
- **Source/Sink**: External entities (squares outside system)

### Editing Properties
1. Click any element to select it
2. Properties panel opens on the right
3. Edit fields directly - changes apply immediately
4. Required fields are highlighted if empty

### Organizing Your Model
- **Hide elements**: Select + press `H` (reduce clutter)
- **Show hidden**: Press `U` (unhide all)
- **Move multiple**: Select several with `Shift+click`, then drag

## Advanced Features

### System Decomposition
- **Enter subsystem**: Double-click any subsystem
- **Navigate up**: Use breadcrumb trail at top
- **Recursive analysis**: Any subsystem can become a new system

### Spatial Regions (v0.2.0)
- **Boundary ring**: Gray ring shows system boundary
- **Click boundary**: Selects the system
- **Environment**: Space outside your system boundary

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Save | `Ctrl+S` |
| Open | `Ctrl+L` |
| Zoom in | `=` |
| Zoom out | `-` |
| Reset view | `Ctrl+R` |
| Delete | `Delete` |
| Hide selected | `H` |
| Unhide all | `U` |
| Deselect | `Escape` |
| Multi-select | `Shift+click` |

## Tips

### Getting Started
1. Start with your main system circle
2. Add interfaces where system connects to world
3. Create flows for inputs and outputs
4. Add subsystems for internal components
5. Save frequently with `Ctrl+S`

### If You Get Stuck
- **Lost your view?** Press `Ctrl+R` to reset
- **Can't see elements?** Press `U` to unhide all
- **Properties missing?** Click an element to select it
- **Elements won't connect?** Drag from one element to another

---

Need more help? Follow the [11-step tutorial](creating-your-first-system/) for hands-on practice.