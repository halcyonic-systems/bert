# Reference Guide

## Reference Guide

This section provides detailed reference information about BERT's controls, interface, and formats.

### Keyboard Shortcuts

Master these shortcuts to work efficiently in BERT:

| Action                            | Control                     |
| --------------------------------- | --------------------------- |
| **Pan**                           | Right-click and drag        |
| **Zoom Out**                      | Press `-` key               |
| **Zoom In**                       | Press `=` key               |
| **Select Element**                | Left-click                  |
| **Multi-select**                  | Hold `Shift` while clicking |
| **Delete Elements**               | `Delete` or `Backspace`     |
| **Open File**                     | `Ctrl+L`                    |
| **Save**                          | `Ctrl+S`                    |
| **Reset View**                    | `Ctrl+R`                    |
| **Hide Elements**                 | `H`                         |
| **Unhide Elements**               | `U`                         |
| **Apply Sink/Source Equivalence** | `E`                         |

See our complete Controls Guide for more details.

### UI Components

BERT's interface is designed for intuitive system modeling:

* Main canvas
* Element creation toolbar
* Properties panel
* Navigation controls
* System hierarchy viewer

_Detailed component reference coming soon..._

### File Formats

Understanding BERT's file formats and data structures:

* BERT JSON format specification
* Import/export capabilities
* Compatibility with other tools
* Data backup best practices

_Detailed format specifications coming soon..._

### System Requirements

* Hardware requirements
* Software dependencies
* Platform-specific notes
* Performance optimization

_Detailed requirements coming soon..._





Element Relationships

The relationships between elements define how your system functions:

| Relationship                    | Description                                                 | Example                            |
| ------------------------------- | ----------------------------------------------------------- | ---------------------------------- |
| **External Entity → Interface** | External entities connect to your system through interfaces | Customer → Order Counter           |
| **Interface → Subsystem**       | Interfaces can connect to internal subsystems               | Order Counter → Kitchen Subsystem  |
| **Subsystem → Interface**       | Subsystems can connect to interfaces for outputs            | Kitchen Subsystem → Pickup Counter |
| **Interface → Interface**       | Interfaces can connect directly to other interfaces         | Order Counter → Pickup Counter     |

***

**Related Sections:**

* Basic Tutorials
* Advanced Features
