# Controls and Interface Guide

Master BERT's interface and controls to build systems efficiently and confidently.

## Quick Start

**New to BERT?** These four controls will get you started:

1. **🖱️ Left-click** any element to select it
2. **🖱️ Right-click + drag** to move around the canvas  
3. **⌨️ Press `=`** to zoom in, **`-`** to zoom out
4. **⌨️ Press `Ctrl+S`** to save your work

**That's it!** You're ready to start building systems.

## Interface Overview

BERT's interface is designed for intuitive system modeling with these main components:

### Main Canvas
- **Central workspace** where you build your systems
- **Infinite canvas** - zoom and pan to see any level of detail
- **Visual feedback** - elements highlight when selected or hovered

### Element Creation Toolbar
- **System elements** - Create systems, subsystems, interfaces
- **External entities** - Add sources and sinks
- **Flows** - Connect elements with material, energy, or information flows
- **One-click creation** - Select tool, then click on canvas

### Properties Panel
- **Automatic display** - Opens when you select any element
- **Live editing** - Changes apply immediately as you type
- **Context-sensitive** - Shows relevant properties for each element type
- **Validation** - Highlights required fields and format errors

### Navigation Controls
- **Breadcrumb navigation** - Shows your current location in system hierarchy
- **Zoom controls** - Visual zoom level indicator
- **View reset** - Quick return to default view

## Navigation and Movement

### Moving Around the Canvas

| What you want to do | How to do it | Visual cue |
|---------------------|--------------|------------|
| **Move around the canvas** | Right-click and drag | 🖱️ Cursor changes to hand |
| **Zoom in** (see details) | Press `=` key | 🔍 Elements get larger |
| **Zoom out** (see overview) | Press `-` key | 🔍 Elements get smaller |
| **Reset to center** | Press `Ctrl+R` | 🎯 View returns to starting position |

### Scrolling Options
- **Mouse wheel** - Scroll up/down and left/right
- **Trackpad** - Use natural scrolling gestures
- **Keyboard** - Arrow keys for precise movement

## Selection and Manipulation

### Basic Selection

| What you want to do | How to do it | What happens |
|---------------------|--------------|--------------|
| **Select one element** | Left-click on it | 🟦 Element gets blue outline |
| **Select multiple elements** | Hold `Shift` + click each one | 🟦 Multiple blue outlines appear |
| **Multi-select tip** | `Shift+Click` works for any number of elements | 💡 Great for moving related parts together |
| **Deselect everything** | Press `Escape` or click empty space | ⚪ All outlines disappear |

### Working with Selected Elements
- **Move them** - Click and drag any selected element
- **Delete them** - Press `Delete` or `Backspace`  
- **Hide them** - Press `H` (useful for complex diagrams)
- **Edit properties** - Properties panel opens automatically

## Creating System Elements

### Element Types and Usage

| Element Type | What it represents | When to use it | Visual appearance |
|--------------|-------------------|----------------|-------------------|
| **🏢 System** | Main system boundary | Starting point for analysis | Large circle with label |
| **🔗 Interface** | Connection points | Where system connects to world | Small circles on system boundary |
| **⚙️ Subsystem** | Internal components | Parts inside your main system | Smaller circles inside systems |
| **🌐 External Entity** | Outside sources/sinks | Things that interact with system | Squares outside system boundary |
| **➡️ Flow** | Transfers between elements | Energy, materials, information | Curved lines with arrows |

### Creating Elements
1. **Select element type** from the toolbar
2. **Click on canvas** where you want to place it
3. **Element appears** with default properties
4. **Edit properties** in the panel that opens automatically

### Connecting Elements

**Simple 3-step process:**
1. **Click** the source element (where flow starts)
2. **Drag** to the target element (where flow ends)
3. **Release** to create the connection

**💡 Tip**: The connection line shows you where it will connect before you release.

## Element Relationships

Understanding how elements connect helps you model systems accurately:

| Relationship | Description | Example |
|--------------|-------------|---------|
| **External Entity → Interface** | External entities connect through interfaces | Customer → Order Counter |
| **Interface → Subsystem** | Interfaces connect to internal components | Order Counter → Kitchen |
| **Subsystem → Interface** | Internal components connect to output interfaces | Kitchen → Pickup Counter |
| **Interface → Interface** | Direct interface connections | Order Counter → Pickup Counter |
| **System → System** | High-level system connections | Restaurant → Supplier |

## Editing and Organization

### Moving Elements
- **Single element**: Click and drag
- **Multiple elements**: Select multiple, then drag any one
- **Precise positioning**: Use small mouse movements for fine control
- **Snap to grid**: Elements align automatically for clean layouts

### Organizing Your View

| Action | Shortcut | Why use it |
|--------|----------|------------|
| **Hide elements** | Select + press `H` | Reduce clutter while working |
| **Show hidden elements** | Press `U` | Bring back hidden elements |
| **Delete elements** | Select + press `Delete` | Remove unwanted elements |
| **Group selection** | `Shift` + click multiple | Move related elements together |

### Working with Properties

1. **Select any element** - Properties panel opens on the right
2. **Edit the fields** - Name, description, type, amounts, units
3. **Changes save automatically** - No need to click "Save"
4. **Required fields** - Highlighted when missing
5. **Validation** - Real-time feedback on data format

## File Management

### Saving Your Work

| Action | Shortcut | When to use it |
|--------|----------|----------------|
| **Save** | `Ctrl+S` | Save frequently! Every few minutes |
| **Save As** | `Ctrl+Shift+S` | Create versions or backups |
| **Open file** | `Ctrl+L` | Load an existing BERT model |

**💡 Pro tip**: Save early, save often. BERT files are small and save quickly.

### File Formats

- **BERT files** end in `.json` - Complete system models
- **Portable format** - Works across all BERT versions
- **Human-readable** - Can be viewed in any text editor
- **Version control friendly** - Works well with Git

## Advanced Features

### System Decomposition

**Go deeper into your systems:**

1. **Double-click a subsystem** - Enter it to see internal details
2. **Use breadcrumbs** - Navigate back up through system levels
3. **Build hierarchically** - Start simple, add detail as needed
4. **Unlimited nesting** - Decompose to any level of detail

### Flow Management

**Make your flows more precise:**
- **Click flow endpoints** - Adjust where connections attach
- **Drag flow curves** - Change the path flows take
- **Edit flow properties** - Set amounts, units, and types
- **Flow validation** - Ensures connections make sense

### Keyboard Shortcuts Reference

| Action | Shortcut | Category |
|--------|----------|----------|
| **Apply equivalence** | `E` | Advanced |
| **Quick save** | `Ctrl+S` | File |
| **Quick open** | `Ctrl+L` | File |
| **Reset view** | `Ctrl+R` | Navigation |
| **Hide selected** | `H` | Organization |
| **Unhide all** | `U` | Organization |
| **Delete selected** | `Delete` | Editing |
| **Deselect all** | `Escape` | Selection |
| **Zoom in** | `=` | Navigation |
| **Zoom out** | `-` | Navigation |
| **Multi-select** | `Shift + Click` | Selection |

### Advanced Shortcuts
- **Apply Equivalence** (`E` key) - Automatically balance flows between connected interfaces

## Workflow Tips

### For Beginners
1. **Start simple** - Create your main system first
2. **Add interfaces** - Where does your system connect to the world?
3. **Add flows** - What moves in and out?
4. **Decompose gradually** - Add internal details as needed
5. **Save frequently** - Protect your work

### For Complex Systems
1. **Use hiding** (`H` key) to focus on specific parts
2. **Work top-down** - Big picture first, details later
3. **Save versions** - Use "Save As" for different iterations
4. **Multi-select** (`Shift+click`) to move related elements together
5. **Plan hierarchy** - Think about decomposition levels upfront

### Performance Tips
- **Hide unused elements** - Improves performance with large models
- **Use subsystems** - Break complex systems into manageable parts
- **Regular saves** - Prevent data loss
- **Zoom appropriately** - Use the right level of detail for your task

## Troubleshooting

### Common Issues and Solutions

| Problem | Solution |
|---------|----------|
| **Can't see my elements** | Press `Ctrl+R` to reset view |
| **Elements won't connect** | Make sure you're dragging from one element to another |
| **Properties panel is empty** | Click on an element to select it first |
| **Lost my work** | Check if you saved recently (`Ctrl+S`) |
| **App is slow** | Hide unused elements with `H` key |
| **Can't find an element** | Use `U` to unhide all elements |
| **Accidental deletion** | Use `Ctrl+Z` to undo (if available) |

### Getting Help

- **Stuck?** Try pressing `Escape` to deselect everything and start fresh
- **Lost your view?** Press `Ctrl+R` to reset to center
- **Want to experiment?** Save your work first, then try new things
- **Need more help?** Check out our [step-by-step tutorials](creating-your-first-system/)

## System Requirements

### Minimum Requirements
- **Web browser**: Chrome, Firefox, Safari, or Edge (latest versions)
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 100MB for desktop app
- **Internet**: Required for web version, optional for desktop

### Optimal Performance
- **RAM**: 16GB or more for large systems
- **Graphics**: Dedicated GPU recommended for complex models
- **Display**: 1920x1080 or higher resolution
- **Input**: Mouse recommended (trackpad supported)

### Platform Support
- **Web**: All modern browsers
- **Desktop**: macOS 10.15+, Windows 10+
- **Mobile**: Limited support (view-only)

---

**Ready to build amazing systems?** Remember: Start simple, save often, and don't be afraid to experiment! 🚀 