# Getting Started

## Prerequisites

1. **Rust**: Install via [rustup.rs](https://rustup.rs/)
2. **Tauri Prerequisites**: Follow the [Tauri guide](https://tauri.app/v1/guides/getting-started/prerequisites)

## Development Setup

```bash
git clone git@github.com:halcyonic-systems/bert.git
cd bert
cargo tauri dev
```

Application opens with BERT interface - try creating a system model to verify everything works.

## Development Standards

BERT maintains professional development standards:

- **100% Documentation Compliance** - All code must follow template guidelines
- **No Clippy Warnings** - Code must pass `cargo clippy --all-targets -- -D warnings`
- **Comprehensive Testing** - Unit + integration tests required
- **Feature Documentation** - Use `./scripts/bert.sh feature "Feature Name"`

## First Contribution

1. **Read the [Contributing Guide](contributing.md)** - Complete workflow details
2. **Review [Documentation Standards](https://github.com/halcyonic-systems/bert/blob/main/docs/technical/rust-documentation-guidelines.md)** - Template requirements
3. **Check Current Status** - See [implementation analysis](https://github.com/halcyonic-systems/bert/blob/main/docs/technical/documentation-implementation-analysis.md)
4. **Create Feature Branch** - Follow `feature/descriptive-name` pattern

## Key Workflows

**Adding Features:**
```bash
git checkout -b feature/my-feature
./scripts/bert.sh feature "My Feature"  # Generate documentation
# Implement + document + test
```

**Quality Checks:**
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings  
cargo test --all
```