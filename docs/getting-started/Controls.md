# BERT Controls Guide

This document provides a comprehensive reference for all keyboard and mouse controls in BERT (Bounded Entity Reasoning Toolkit).

## Navigation Controls

| Action | Control |
|--------|---------|
| **Pan** | Right-click and drag |
| **Scroll** | Use mouse wheel to scroll vertically/horizontally |
| **Reset View** | Press `Ctrl+R` (both Mac and Windows) |

## Zoom Controls

| Action | Control |
|--------|---------|
| **Zoom Out** | Press `-` key (elements appear smaller) |
| **Zoom In** | Press `=` key (elements appear larger) |

## Selection Controls

| Action | Control |
|--------|---------|
| **Select Element** | Left-click on an element |
| **Multi-select** | Hold `Shift` while clicking elements |
| **Deselect All** | Press `Escape` |

## Element Management

| Action | Control |
|--------|---------|
| **Move Elements** | Click and drag selected elements |
| **Delete Elements** | Select element(s) and press `Delete` or `Backspace` |
| **Hide Elements** | Select element(s) and press `H` |
| **Unhide Elements** | Press `U` to unhide previously hidden elements |

## File Operations

| Action | Control |
|--------|---------|
| **Open File** | Press `Ctrl+L` (both Mac and Windows) |
| **Save** | Press `Ctrl+S` (both Mac and Windows) |

## Advanced Controls

| Action | Control |
|--------|---------|
| **Apply Sink/Source Equivalence** | Press `E` |

## System Creation and Editing

### Creating Elements
- Use the toolbar buttons to create system elements:
  - External Entities 
  - Interfaces
  - Subsystems
  - Flows

### Connecting Elements
1. Select the source element
2. Click and drag to the target element
3. Release to create a connection

### Editing Elements
1. Select an element to view its properties
2. Edit properties in the properties panel
3. Changes are applied immediately

## System Decomposition

### Enter Subsystem
- Double-click on a subsystem to enter it and model its internal components

### Navigate System Hierarchy
- Use breadcrumb navigation at the top of the screen to move between system levels
- Zoom in/out to see different levels of detail

### Flow Connections
- Connect elements with flows to represent material, energy, or information transfers
- Flows can connect between:
  - External entities and interfaces
  - Interfaces and other interfaces
  - Subsystems (representing higher-level connections)

## Context-Specific Controls

### When Working with Flows
- Click on flow endpoints to modify connection points
- Adjust flow curvature by dragging the flow line

### When Working with Subsystems
- Resize subsystems by selecting and using the resize handles
- Automatically position interfaces around subsystem boundaries

## File Management

### Starting a New Project
- BERT automatically starts with a blank system canvas
- A main system circle is provided as your starting point
- Begin adding elements directly to this canvas

### Opening Existing Projects
1. Press `Ctrl+L` to open the file browser
2. Navigate to and select your BERT JSON file
3. Click "Open" to load the file into the editor

### Saving Your Work
- Press `Ctrl+S` to save your current work
- When prompted, choose a location and filename
- BERT will save your model in its native JSON format

## Tips for Efficient Usage

- Use multi-select (Shift+click) to manipulate multiple elements simultaneously
- Hide elements (H key) to reduce visual complexity when working on specific parts of a system
- Use Ctrl+S frequently to save your work
- For complex systems, build hierarchically from top-down, decomposing subsystems as needed
- Create a backup of important models by using "Save As" with a new filename
