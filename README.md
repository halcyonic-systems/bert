# BERT (Bounded Entity Reasoning Toolkit)
A software tool that guides users through the rigorous analysis of complex adaptive systems

# Quick Start

Visit [bert.systems](https://bert.systems/) to use the web application

**Download BERT for**:

[MacOS Apple Silicon](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert_0.1.0_aarch64.dmg)

[MacOS Intel](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert_0.1.0_x64.dmg)

[Windows](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert.exe)



View [our tutorial videos](https://github.com/halcyonic-systems/bert/blob/main/docs/Tutorials.md) introducing BERT's basic functionality.

# Key Features
- Guides the systematic decomposition of complex systems by enforcing Deep Systems Analysis methodology
- Visual system mapping that preserves critical flows, interfaces, and relationships between components
- Structured knowledge capture in a standardized, computable format enabling validation and sharing

# Example Analyses
- [The Bitcoin Network](https://github.com/halcyonic-systems/bert/blob/main/btc.json)

# Why BERT?
BERT enables analysts to develop detailed understanding of complex systems while preserving the crucial details and meanings often lost through abstract modeling approaches. The tool implements a rigorous methodology called Deep Systems Analysis (DSA).

# Technical Background
BERT implements ideas from [George Mobus's](https://directory.tacoma.uw.edu/employee/gmobus) [Systems Science: Theory, Analysis, Modeling and Design](https://link.springer.com/book/10.1007/978-3-030-93482-8). After an interdisciplinary career spanning naval engineering, robotics, artificial intelligence, computer science, energy systems modeling, and systems science, Mobus identified key limitations in standard systems modeling frameworks like [Stella](https://www.iseesystems.com/store/products/stella-online.aspx) and [UML](https://www.uml.org/)/[SysML](https://sysml.org/). To address these gaps, he proposed the creation of a new formal "System Language" (SL) grounded in systems science principles.

BERT represents a first step toward developing this formal systems language, built specifically for modern systems scientists.

Read here for more information about the [various components of SL](https://github.com/halcyonic-systems/bert/blob/main/research/system%20language/system_language.md)

# Getting Started
Bert is a Rust-based project that uses [Leptos](https://leptos.dev/) for web UIs, and [Bevy](https://bevyengine.org/) for creating and interacting with systems and their components.
[Tauri](https://v2.tauri.app/) is used for building a desktop version of the project.

## Prerequisites
**Rust**: Install Rust using `rustup` by following the instructions [here](https://www.rust-lang.org/tools/install).

**Tauri**: Install Tauri using `cargo install create-tauri-app --locked` by following the instructions [here](https://v2.tauri.app/start/).

## Run the Project
Start the development server:
```rust
cargo tauri dev
```
This command will compile the Rust code and open a window with your web content.

## Explore the Codebase
Inside the `src-tauri` directory, you'll find the following files:

- `src/main.rs`: The entry point of the application.
- `tauri.conf.json`: Configuration file for Tauri.
- `Cargo.toml`: Cargo manifest for the Tauri project.
- `src/lib.rs`: Code for building a tauri application and its extra functions to communicate with the native parts of the application.

The project (`~bert/src/`) is structured as follows:
 - `bevy_app`: Contains the Bevy application and its components.
 - `leptos_app`: Contains the Leptos web ui components.

# Contributing
