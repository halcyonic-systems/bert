<h1 align="center">BERT (Bounded Entity Reasoning Toolkit)</h1>

<p align="center">A visual software tool that guides users through the rigorous analysis and decomposition of complex adaptive systems.</p>

<p align="center">
  <a href="https://bert.systems/">Website</a> •
  <a href="https://bert.gitbook.io/bert-documentation">Documentation</a> •
  <a href="https://github.com/halcyonic-systems/bert/releases">Downloads</a> •
  <a href="#getting-started">Getting Started</a>
</p>

![BERT Demo](https://bert.systems/demo.gif)

## 📑 Table of Contents

- [📑 Table of Contents](#-table-of-contents)
- [📖 Background](#-background)
- [🚀 Getting Started](#-getting-started)
- [🔧 Key Features](#-key-features)
- [📚 Documentation](#-documentation)
- [💾 Installation](#-installation)
- [👨‍💻 Development](#-development)
- [🤝 Contributing](#-contributing)
- [©️ License](#️-license)

## 📖 Background

BERT implements ideas from [George Mobus's](https://directory.tacoma.uw.edu/employee/gmobus) [Systems Science: Theory, Analysis, Modeling and Design](https://link.springer.com/book/10.1007/978-3-030-93482-8). It was created to address key limitations in standard systems modeling frameworks like [Stella](https://www.iseesystems.com/store/products/stella-online.aspx) and [UML](https://www.uml.org/)/[SysML](https://sysml.org/).

BERT enables analysts to develop detailed understanding of complex systems while preserving crucial details and meanings often lost through abstract modeling approaches. The tool implements a rigorous methodology called Deep Systems Analysis (DSA) and represents a first step toward developing a formal systems language, built specifically for modern systems scientists.

## 🚀 Getting Started

### Web Version
- Visit [bert.systems](https://bert.systems/) to use BERT directly in your browser

### Desktop Applications
- [MacOS Apple Silicon](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert_0.1.0_aarch64.dmg)
- [MacOS Intel](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert_0.1.0_x64.dmg)
- [Windows](https://github.com/halcyonic-systems/bert/releases/download/v0.1.0-beta/bert.exe)

Once you've launched BERT, check out our [interactive tutorial](https://bert.gitbook.io/bert-documentation/getting-started/basic-controls) or see the [Controls Guide](docs/Controls.md) for navigation tips.

## 🔧 Key Features

- Guides systematic decomposition of complex systems using Deep Systems Analysis methodology
- Visual system mapping that preserves critical flows, interfaces, and relationships
- Structured knowledge capture in a standardized, computable format
- Hierarchical modeling with unlimited decomposition levels

## 📚 Documentation

- [Written Tutorials](https://bert.gitbook.io/bert-documentation) - Comprehensive user guides
- [Video Tutorials](https://github.com/halcyonic-systems/bert/blob/main/docs/Tutorials.md) - Visual walkthroughs
- [Architecture Overview](ARCHITECTURE.md) - Understand how BERT works
- [Example: Bitcoin Network](https://github.com/halcyonic-systems/bert/blob/main/btc.json) - See BERT in action

### For Systems Scientists

If you're a systems scientist interested in using or extending BERT:

1. Start with [Getting Started for Systems Scientists](docs/getting-started/for-systems-scientists.md)
2. Reference [ARCHITECTURE.md](ARCHITECTURE.md) for a high-level conceptual overview
3. Use [Code Navigation Guide](docs/getting-started/code-navigation-guide.md) when exploring the code

## 💾 Installation

BERT is available as a desktop application for macOS and Windows, or you can use the web version. See the [Getting Started](#getting-started) section for download links.

## 👨‍💻 Development

BERT is a Rust-based project that uses:
- [Leptos](https://leptos.dev/) for web UIs
- [Bevy](https://bevyengine.org/) for creating and interacting with systems and their components
- [Tauri](https://v2.tauri.app/) for building desktop applications

### Prerequisites
- **Rust**: Install using `rustup` ([installation instructions](https://www.rust-lang.org/tools/install))
- **Tauri**: Install using `cargo install create-tauri-app --locked` ([installation instructions](https://v2.tauri.app/start/))

### Run the Project
```bash
cargo tauri dev
```

### Project Structure
- `src-tauri/src/main.rs`: Application entry point
- `src-tauri/tauri.conf.json`: Tauri configuration
- `src/bevy_app`: Bevy application and components
- `src/leptos_app`: Leptos web UI components

For more details, see [ARCHITECTURE.md](ARCHITECTURE.md) and the [documentation](https://bert.gitbook.io/bert-documentation).

## 🤝 Contributing

Interested in contributing to BERT? We welcome contributions of all kinds:

- Review and update documentation
- Test the application and report issues
- Implement new features or fix bugs
- Share your experience using BERT

For more information, see [Contributing Guidelines](docs/contributing/guidelines.md).
