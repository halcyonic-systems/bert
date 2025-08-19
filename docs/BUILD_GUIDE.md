# BERT v0.2.0 Executable Build Guide

## Overview

BERT uses Tauri framework for building cross-platform desktop applications. This guide documents the complete build process for creating executables and distributable packages.

## Prerequisites

### Required Tools
- **Rust toolchain** (stable)
- **cargo-tauri** CLI (v2.6.2+)
- **trunk** frontend build tool
- **Node.js** and npm (for frontend dependencies)

### Platform-Specific Requirements

#### macOS
- Xcode Command Line Tools
- For signing: Apple Developer account and certificates
- `sips` tool (built-in) for icon conversion

#### Windows
- Visual Studio Build Tools
- Windows SDK

#### Linux
- Build essentials (gcc, make, etc.)
- GTK development libraries

## Build Architecture

BERT follows Tauri's hybrid architecture:
- **Frontend**: Leptos (Rust WASM) served via Trunk
- **Backend**: Rust native application
- **Communication**: Tauri's IPC bridge

## Build Process

### 1. Development Build

For testing and development:

```bash
cargo tauri dev
```

This starts the development server with hot reload.

### 2. Release Build (Binary Only)

Create optimized binary without bundling:

```bash
cargo tauri build --no-bundle
```

**Output**: `target/release/bert-tauri` (executable)
**Size**: ~24MB (ARM64 macOS)

### 3. Platform Bundles

#### macOS App Bundle + DMG

```bash
cargo tauri build --bundles dmg
```

**Outputs**:
- `target/release/bundle/macos/bert.app` - macOS application bundle
- `target/release/bundle/dmg/bert_0.2.0_aarch64.dmg` - Distributable DMG (~17MB)

#### Application Bundle Only

```bash
cargo tauri build --bundles app
```

**Output**: `target/release/bundle/macos/bert.app`

### 4. Cross-Platform Targets

#### Universal macOS Build (Intel + ARM)

```bash
cargo tauri build --target universal-apple-darwin --bundles dmg
```

**Requirements**: Both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets installed:

```bash
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
```

#### Windows Build (from macOS/Linux)

```bash
# Install Windows target
rustup target add x86_64-pc-windows-msvc

# Cross-compile (requires additional setup)
cargo tauri build --target x86_64-pc-windows-msvc
```

#### Linux Build

```bash
cargo tauri build --bundles appimage,deb
```

## Build Configuration

### Key Files
- `src-tauri/tauri.conf.json` - Main Tauri configuration
- `src-tauri/Cargo.toml` - Rust dependencies and metadata
- `Trunk.toml` - Frontend build configuration

### Build Stages

1. **Frontend Build** (`trunk build`)
   - Compiles Leptos to WASM
   - Generates optimized CSS/JS
   - Output: `dist/` directory

2. **Rust Compilation** (`cargo build --release`)
   - Compiles Rust backend with Tauri runtime
   - Links frontend assets
   - Output: Native executable

3. **Bundling** (platform-specific)
   - Creates platform packages (DMG, MSI, AppImage, etc.)
   - Includes icons, metadata, and signing

## Icon Requirements

BERT requires multiple icon formats:

### Generated Icons
- `src-tauri/icons/32x32.png`
- `src-tauri/icons/64x64.png`  
- `src-tauri/icons/128x128.png`
- `src-tauri/icons/icon.ico` (Windows)
- `src-tauri/icons/icon.icns` (macOS)

### Icon Generation
If missing icon.icns, generate from PNG:

```bash
cd src-tauri/icons
sips -s format icns icon.png --out icon.icns
```

## Performance Optimization

### Release Build Features
- Dead code elimination
- Link-time optimization (LTO)
- Binary stripping
- Asset compression

### Size Optimization
Current release binary sizes:
- **macOS ARM64**: ~24MB executable, ~17MB DMG
- Includes Tauri runtime, WebView, and all dependencies

## Distribution

### macOS Distribution
- **Development**: Direct `.app` bundle sharing
- **Production**: Signed DMG with Apple Developer ID
- **App Store**: Additional entitlements required

### Windows Distribution
- **Development**: Portable executable
- **Production**: Signed MSI installer
- **Microsoft Store**: MSIX package

### Linux Distribution
- **AppImage**: Portable application
- **DEB/RPM**: System packages
- **Flatpak/Snap**: Universal packages

## Troubleshooting

### Common Issues

#### Missing Icons
```
Error: resource path `icons/icon.icns` doesn't exist
```
**Solution**: Generate missing icon format (see Icon Generation above)

#### Frontend Build Failures
```
Error: trunk build failed
```
**Solution**: Check Node.js dependencies, run `npm install`

#### Cross-compilation Issues
- Ensure target architectures are installed via `rustup target add`
- Some dependencies may require native compilation tools

### Debug Commands

```bash
# Verbose build output
cargo tauri build -v

# Check available targets
rustc --print target-list | grep -E "(apple|windows|linux)"

# Verify configuration
cargo tauri info
```

## Version Management

Current version: **0.2.0** (set in `src-tauri/tauri.conf.json`)

Version updates require changes to:
- `src-tauri/tauri.conf.json` → `version`
- `src-tauri/Cargo.toml` → `version`
- This documentation

## CI/CD Integration

For automated builds, see `.github/workflows/` for GitHub Actions configuration supporting:
- Multi-platform builds
- Automated testing
- Release artifact creation
- Code signing integration

---

*Generated for BERT v0.2.0 - Systems Modeling Platform*
*Last Updated: August 19, 2024*