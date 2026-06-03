# Getting Started with BERT

BERT is a visual systems modeling tool. You describe a system in plain English, BERT generates a structured model, and you refine it by dragging, connecting, and chatting.

This guide covers your first five minutes.

## Install

**Web (WASM)**: Open [bert.systems](https://bert.systems) in any modern browser. No install needed. Supports manual modeling, the model browser, and file import. NL generation requires the desktop app.

**Desktop**: Download the latest release for your platform from [GitHub Releases](https://github.com/halcyonic-systems/bert/releases). Available as `.dmg` (macOS), `.msi` (Windows), and `.AppImage`/`.deb` (Linux).

**From source** (developers):
```bash
git clone https://github.com/halcyonic-systems/bert.git && cd bert
npm install && cargo tauri dev
```

See [BUILD.md](BUILD.md) for full build instructions and prerequisites.

## The Landing Screen

When you open BERT, you see four entry points:

| Entry Point | What it does | Requirements |
|---|---|---|
| **Describe a System** | Type a plain-English description, hit Generate, get a structured model | Desktop + [Ollama](https://ollama.com) running |
| **Start from Scratch** | Opens a blank canvas with an empty root system | None |
| **Browse Models** | Load a built-in example (LLM, Bitcoin, Ethereum, Cosmos Hub, Solana) | None |
| **Open File** | Load a `.json` model from disk | None |

The landing screen also shows your recent models for quick access (desktop only).

### Ollama Setup

The "Describe a System" flow and the chat panel use a local LLM via [Ollama](https://ollama.com). Install it, then make sure it is running before you launch BERT. The landing screen shows a green dot next to "Describe a System" when Ollama is detected, or an amber warning if it is not.

If you just want to explore, use **Browse Models** or **Start from Scratch** — neither requires Ollama.

## Build Your First Model

1. Open BERT (desktop, with Ollama running).
2. In the "Describe a System" field, type `a coffee shop` and press Enter (or click Generate).
3. A spinner appears while BERT extracts system structure from the description and compiles a validated model. This takes a few seconds.
4. The canvas loads your model.

## What You See

After generation, the canvas shows:

- **S0** — the root system (large circle), named after your description. This is your system of interest.
- **Subsystems** — internal components (smaller circles inside S0). For a coffee shop: Kitchen, Service Counter, Supply Management, etc.
- **Sources** — external entities that provide inputs (e.g., Coffee Suppliers, Customers entering).
- **Sinks** — external entities that receive outputs (e.g., Customers leaving with coffee, Waste disposal).
- **Flows** — arrows connecting sources, subsystems, and sinks. Each flow carries material, energy, or information.
- **Boundary** — the gray ring around S0, separating the system from its environment.

The chat panel opens automatically on the right side after generation. It confirms the first draft was created and invites you to refine it.

### The 8-Tuple

Every BERT model is grounded in the Mobus 8-tuple — eight dimensions that define any system:

| Symbol | Dimension | What you see on the canvas |
|--------|-----------|---------------------------|
| **C** | Components | Subsystem circles inside S0 |
| **N** | Network | Flow arrows between subsystems |
| **E** | Environment | The space outside the boundary |
| **G** | External interactions | Sources and sinks |
| **B** | Boundary | The gray ring around S0 |
| **T** | Transformation | What each subsystem does (its process primitives) |
| **H** | History | What the system has learned over time |
| **dt** | Time scale | How fast each subsystem operates |

You don't need to memorize this to use BERT — the tool handles the formal structure. But when you see these symbols in the chat panel or property inspector, this is what they refer to. See [mobus-reference.md](mobus-reference.md) for the full formal definition.

## Basic Interactions

**Select**: Left-click any element. The properties panel opens on the right showing its name, type, and editable fields.

**Pan**: Right-click and drag.

**Zoom**: Press `=` to zoom in, `-` to zoom out. `Ctrl+R` resets the view.

**Move elements**: Click and drag.

**Connect elements**: Press `F` to enter Flow Mode, click a source element, then click a target.

**Delete**: Select an element, press `Delete` or `Backspace`.

**Enter a subsystem**: Double-click any subsystem circle to zoom into its internal structure. Use the breadcrumb trail at top to navigate back up.

**Undo / Redo**: `Ctrl+Z` / `Ctrl+Shift+Z`.

**Save**: `Ctrl+S`. **Save As**: `Ctrl+Shift+S`. Save early, save often.

**Open**: `Ctrl+L` to load a file.

## Refine with Chat

With a model loaded (desktop), click the **Chat** button in the toolbar to open the chat panel. Describe what you want to change in plain English — add a subsystem, rename a flow, restructure the hierarchy. Click **Generate Model** to rebuild from the full conversation context.

The chat shows which 8-tuple dimensions each response touches, along with source references from Mobus systems science.

## Next Steps

- **Explore the examples**: Open the Model Browser and load the LLM or Bitcoin model to see how complex systems are decomposed.
- **Try manual modeling**: Start from Scratch and build a system by hand using the toolbar palette.
- **Learn the 8-tuple**: See [mobus-reference.md](mobus-reference.md) for the formal system definition that grounds every BERT model.
- **Understand the schema**: See [system-language-spec.md](system-language-spec.md) for the System Language specification and [bert-schema-reference.md](bert-schema-reference.md) for the JSON format.

## Quick Reference

| Action | Shortcut |
|---|---|
| Save | `Ctrl+S` |
| Save As | `Ctrl+Shift+S` |
| Open file | `Ctrl+L` |
| Undo / Redo | `Ctrl+Z` / `Ctrl+Shift+Z` |
| Zoom in / out | `=` / `-` |
| Scroll zoom | `Ctrl+scroll` |
| Reset view | `Ctrl+R` |
| Delete selected | `Delete` / `Backspace` |
| Flow mode | `F` |
| Interface subsystem | `I` (with interface selected) |
| Set equivalence | `E` |
| Hide selected | `H` |
| Unhide all | `U` |
| Screenshot | `Ctrl+P` |
| Toggle background | `Ctrl+Alt+B` |
| Deselect | `Escape` |
| Multi-select | `Shift+click` |
