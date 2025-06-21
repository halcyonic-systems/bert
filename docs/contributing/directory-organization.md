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

- Place project/developer documentation in `docs/contributing/`
- Place architecture documentation in `docs/architecture/`
- Place user guides in `docs/getting-started/`
- Place research materials in `docs/research/`
- Keep the main README.md in the root directory

### Configuration

- Keep build-critical configuration files in their original locations
- Document configuration options in documentation

### Source Code

- Maintain the existing `src/` and `src-tauri/` structure
- Follow module organization patterns in existing code

### Assets and Resources

- Place all static assets in the `assets/` directory
- Organize assets into subdirectories by type (images, fonts, etc.)

## Risky Reorganizations

The following changes would require updates to build configurations and should only be attempted with thorough testing:

1. **Moving Web Assets**:
   - `index.html`, `styles.css`, and `input.css` are referenced in build tools
   - Moving these would require updating Trunk.toml and other configuration files

2. **Changing Configuration File Locations**:
   - Trunk.toml, Cargo.toml, and package.json have expected locations
   - Moving these would break build tools

3. **Reorganizing Source Structure**:
   - The src/ and src-tauri/ directories have established patterns
   - Major reorganization would require comprehensive code updates

## Recommended Safe Improvements

These changes can be made without disrupting the build process:

1. **Documentation Cleanup**:
   - Move project documentation to appropriate subdirectories in `docs/`
   - Create a clear structure in the docs directory
   - Consider consolidating duplicate content

2. **Asset Organization**:
   - Organize assets into logical subdirectories
   - Use consistent naming conventions

3. **Development Tools**:
   - Consider adding development tools in a `.tools/` or `scripts/` directory
   - Document their usage in the contributing guide

## Implementation Strategy

When implementing directory organization changes:

1. **Incremental Approach**:
   - Make small, focused changes
   - Test thoroughly after each change
   - Document what you've done

2. **Compatibility Testing**:
   - Verify the build works after reorganization
   - Test on multiple platforms if possible

3. **Documentation**:
   - Update documentation to reflect new organization
   - Provide migration guides if needed