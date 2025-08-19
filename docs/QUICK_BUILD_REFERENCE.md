# BERT Quick Build Reference

## Essential Commands

### Development
```bash
cargo tauri dev          # Development server with hot reload
```

### Production Builds
```bash
cargo tauri build --no-bundle           # Binary only (~24MB)
cargo tauri build --bundles app         # macOS .app bundle
cargo tauri build --bundles dmg         # macOS DMG installer (~17MB)
```

### Build Outputs
- **Binary**: `target/release/bert-tauri`
- **App Bundle**: `target/release/bundle/macos/bert.app`
- **DMG**: `target/release/bundle/dmg/bert_0.2.0_aarch64.dmg`

## Prerequisites Check
```bash
cargo --version         # Rust toolchain
cargo tauri --version   # tauri-cli 2.6.2+
trunk --version         # trunk for frontend builds
```

## Troubleshooting
- **Missing icons**: Run `sips -s format icns icon.png --out icon.icns` in `src-tauri/icons/`
- **Build failures**: Ensure you're in the `/bert/bert/` directory, not the root
- **Frontend issues**: Run `npm install` if needed

## File Structure
```
bert/
├── src-tauri/
│   ├── tauri.conf.json     # Version: 0.2.0
│   ├── icons/              # All required icon formats
│   └── Cargo.toml          # Rust dependencies
├── dist/                   # Frontend build output
└── target/release/         # Build artifacts
```

**Status**: ✅ Fully tested on macOS ARM64
**Version**: 0.2.0
**Last Updated**: August 19, 2024