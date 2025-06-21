# Screenshot Export Feature

## Overview
The Screenshot Export feature allows users to capture screenshots of the BERT system diagrams and save them to disk. This feature is essential for documentation, presentations, and sharing visual system models with others.

## Implementation
The feature uses a two-part architecture:
1. **Tauri Backend**: Handles file save dialogs and file operations
2. **Leptos Frontend**: Provides UI controls and event coordination

**Note:** This initial implementation provides the UI and dialog workflow but contains a placeholder for the actual screenshot capture functionality, which will be fully implemented in a future update.

### Key Components

#### Backend (Tauri)
- **Dependencies**: `chrono` for timestamp generation
- **Commands**:
  - `take_screenshot`: Captures a screenshot without saving
  - `save_screenshot_with_dialog`: Captures and opens a save dialog

#### Frontend (Leptos)
- **UI Component**: `ScreenshotControls` - Floating controls in the bottom-right corner
- **Events**: `ScreenshotEvent` for Bevy-Leptos communication
- **Hooks**: `use_screenshot` for programmatic screenshot capture

### User Interface
The UI provides two buttons:
- **Save Screenshot**: Captures and shows a save dialog
- **Quick Screenshot**: Captures without saving (for quick preview)

A status message appears below the buttons to show the result of the screenshot operation.

## User Experience
1. Click "Save Screenshot" or "Quick Screenshot" button
2. For "Save Screenshot":
   - A save dialog appears with a default filename including timestamp
   - User selects location and confirms
   - Status message confirms successful save with path
3. For "Quick Screenshot":
   - Status message confirms capture (screenshot is not saved to disk)

## Technical Details

### Event Flow
1. User clicks screenshot button in Leptos UI
2. Tauri command is invoked via WASM bindings
3. Screenshot is captured by `tauri-plugin-screenshots`
4. Result is returned to Leptos UI
5. Status message is updated accordingly

### File Format
Screenshots are saved as PNG images with the naming format: `bert_screenshot_YYYYMMDD_HHMMSS.png`

## Future Enhancements
- Add keyboard shortcuts (e.g., Ctrl+S for Save Screenshot)
- Implement image format options (PNG, JPG, SVG)
- Add annotation capabilities
- Provide option to copy to clipboard