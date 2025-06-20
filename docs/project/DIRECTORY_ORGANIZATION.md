# BERT Directory Organization Guide

This document provides guidance for organizing the BERT repository structure to maintain a clean, intuitive codebase while ensuring build compatibility.

## Current Structure

The BERT repository uses a hybrid structure with these key components:

- **Source Code**:
  - `src/` - Rust application code (Bevy and Leptos)
  - `src-tauri/` - Tauri desktop integration

- **Web Assets**:
  - `index.html` - Primary entry point
  - `input.css` - Tailwind CSS input
  - `styles.css` - Compiled styles
  - `assets/` - Static assets and resources

- **Documentation**:
  - `docs/` - Project documentation
  - `research/` - Research materials
  - `gitbook/` - User documentation

- **Configuration**:
  - `Trunk.toml` - Trunk bundler configuration
  - `Cargo.toml` - Rust package dependencies
  - `package.json` - Node.js dependencies
  - `rust-toolchain.toml` - Rust version specification
  - `.taurignore` - Tauri build exclusions

## Organization Principles

When organizing the repository, follow these principles:

1. **Maintain Build Compatibility**: 
   - Don't move files referenced in build configurations (index.html, styles.css, etc.)
   - If reorganization is needed, use symbolic links rather than moving critical files

2. **Logical Grouping**:
   - Group related files in appropriately named directories
   - Use nested directories for better categorization

3. **Documentation Organization**:
   - Keep primary README files in the root directory
   - Move detailed documentation to appropriate subdirectories in `docs/`
   - Use cross-references to maintain discoverability

4. **Clean Version Control**:
   - Exclude build artifacts and system files from git
   - Keep repository size manageable

## Safe Organization Guidelines

When adding new files or reorganizing existing ones:

### Documentation

- Place project/developer documentation in `docs/project/`
- Place architecture documentation in `docs/architecture/`
- Place user documentation in `docs/user/` or `gitbook/`
- Keep the main README.md in the root directory

### Configuration

- Keep build-critical configuration files in their original locations
- Document configuration options in `docs/project/configuration.md`

### Source Code

- Maintain the existing `src/` and `src-tauri/` structure
- Follow module organization patterns in existing code

### Assets and Resources

- Place all static assets in the `assets/` directory
- Organize assets into subdirectories by type (images, fonts, etc.)

## Future Improvements

For future consideration (requires build configuration updates):

1. Creating a `web/` directory for all web-related assets
2. Consolidating configuration files in a `.config/` directory
3. Creating a more structured documentation hierarchy

These changes should only be undertaken as part of a dedicated refactoring effort with comprehensive testing.