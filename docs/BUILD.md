# BERT Build & Release Guide

## Quick Reference

### Development
```bash
cargo tauri dev          # Desktop with hot reload
trunk serve              # Web at localhost:1320
```

### Production Builds
```bash
cargo tauri build --no-bundle           # Binary only (~24MB)
cargo tauri build --bundles app         # macOS .app bundle
cargo tauri build --bundles dmg         # macOS DMG installer (~17MB)
```

### Prerequisites Check
```bash
cargo --version         # Rust toolchain
cargo tauri --version   # tauri-cli 2.6.2+
trunk --version         # trunk for frontend builds
```

---

## Build Architecture

BERT uses Tauri's hybrid architecture:
- **Frontend**: Leptos (Rust → WASM) served via Trunk
- **Backend**: Rust native application
- **Communication**: Tauri IPC bridge

### Build Outputs
- **Binary**: `target/release/bert-tauri`
- **App Bundle**: `target/release/bundle/macos/bert.app`
- **DMG**: `target/release/bundle/dmg/bert_<version>_aarch64.dmg`

---

## Platform Builds

### macOS

**ARM64 (Apple Silicon)**:
```bash
cargo tauri build --bundles dmg
```

**Universal (Intel + ARM)**:
```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo tauri build --target universal-apple-darwin --bundles dmg
```

### Windows
```bash
cargo tauri build --bundles msi
```

### Linux
```bash
cargo tauri build --bundles appimage,deb
```

---

## Release Workflow (CI/CD)

GitHub Actions automatically builds all platforms on version tags.

### Create a Release
```bash
git tag v0.2.5
git push origin v0.2.5
```

This triggers parallel builds for:
- macOS ARM64 + Intel (`.dmg`)
- Windows (`.msi`, `.exe`)
- Linux (`.AppImage`, `.deb`)

### Manual Trigger
1. GitHub Actions → "Release Desktop App" workflow
2. Click "Run workflow" → enter version → run

### Expected Assets
```
bert_<version>_aarch64.dmg      # macOS ARM64
bert_<version>_x64.dmg          # macOS Intel
BERT_<version>_x64-setup.msi    # Windows
bert_<version>_amd64.AppImage   # Linux
bert_<version>_amd64.deb        # Linux Debian
```

---

## Version Management

Update version in these files before release:
- `src-tauri/tauri.conf.json` → `version`
- `src-tauri/Cargo.toml` → `version`

---

## Troubleshooting

### Missing Icons
```bash
cd src-tauri/icons
sips -s format icns icon.png --out icon.icns
```

### Frontend Build Failures
```bash
npm install
trunk build
```

### Debug
```bash
cargo tauri build -v          # Verbose output
cargo tauri info              # Configuration check
```

---

## Configuration Files

| File | Purpose |
|------|---------|
| `src-tauri/tauri.conf.json` | Tauri config, version, window settings |
| `src-tauri/Cargo.toml` | Rust deps, app metadata |
| `Trunk.toml` | WASM bundler config |
| `.github/workflows/release.yml` | CI/CD pipeline |
