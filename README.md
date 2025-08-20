# BERT - Bounded Entity Reasoning Toolkit

The systems scientists tookit.

[Website](https://bert.systems) • [Documentation](https://bert.gitbook.io/bert-documentation) • [Downloads](https://github.com/halcyonic-systems/bert/releases)

## What is BERT?

BERT is a desktop application that supports deep analysis of complex adaptive systems. It implements the [System Language](https://bert.gitbook.io/bert-documentation/system-language) framework, allowing you to:

- Understand how a system relates to its environment
- Map internal flows of energy, information, and matter between a system's components 
- View the hierarchical structure of systems composed of systems
- Identify and understand the properties of system boundaries

## Installation

### Desktop Application

Download the latest release for your platform:

- **Windows**: Download the `.msi` installer from [Releases](https://github.com/halcyonic-systems/bert/releases)
- **macOS**: Download the `.dmg` file from [Releases](https://github.com/halcyonic-systems/bert/releases)  
- **Linux**: Download the `.AppImage` or `.deb` from [Releases](https://github.com/halcyonic-systems/bert/releases)

Note: macOS users may need to allow the app in Security & Privacy settings on first run.

### Web Version

Try BERT in your browser at [bert.systems](https://bert.systems).

### Build from Source

Requirements: Rust, Node.js, npm

```bash
git clone https://github.com/halcyonic-systems/bert.git
cd bert
npm install
cargo tauri dev    # Development
cargo tauri build  # Production build
```

## Features

**v0.2.0 - Current Release**
- Interactive graph visualization of system structure
- Built in model browser with example systems
- Cross-platform desktop application
- System complexity counter
- Simplified element details panel

## Documentation

- [User Guide](https://bert.gitbook.io/bert-documentation)
- [System Language Overview](https://bert.gitbook.io/bert-documentation/system-language)
- [Contributing Guide](https://bert.gitbook.io/bert-documentation/for-developers/contributing)

## Contributing

See our [Contributing Guide](CONTRIBUTING.md) for:
- Development setup
- Code standards
- Pull request process

## License

MIT License - see [LICENSE](LICENSE) file for details.