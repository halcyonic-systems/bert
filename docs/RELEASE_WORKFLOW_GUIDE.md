# BERT Multi-Platform Release Workflow Guide

## Overview

BERT now uses GitHub Actions to automatically build for all major platforms and create releases. This ensures consistent, reliable builds across Windows, macOS (Intel & ARM), and Linux.

## What the CI/CD Pipeline Builds

### 🍎 macOS Builds
- **macOS ARM64** (Apple Silicon: M1/M2/M3): `.dmg` installer
- **macOS x86_64** (Intel): `.dmg` installer
- Automatically generates missing `icon.icns` if needed
- Code-signed ready (requires certificates in secrets)

### 🪟 Windows Builds  
- **Windows x86_64**: `.msi` installer and portable `.exe`
- Compatible with Windows 10/11
- Digitally signed ready (requires certificate)

### 🐧 Linux Builds
- **Linux x86_64**: `.AppImage` (universal portable) and `.deb` package
- Works across most Linux distributions
- GTK-based desktop integration

## How to Create a Release

### Method 1: Git Tag Release (Recommended)
```bash
# Create and push a version tag
git tag v0.2.0
git push origin v0.2.0
```

The workflow automatically triggers and builds all platforms.

### Method 2: Manual Trigger
1. Go to **GitHub Actions** tab in your repo
2. Select **"Release Desktop App"** workflow  
3. Click **"Run workflow"**
4. Enter version (e.g., `v0.2.0`)
5. Click **"Run workflow"**

## Release Process Timeline

1. **Trigger** → Workflow starts on 4 parallel runners
2. **~5-10 minutes** → All platform builds complete
3. **Auto-release** → GitHub release created with all assets
4. **Draft release** → Review and publish when ready

## Expected Release Assets

When complete, your GitHub release will have:

```
BERT-v0.2.0/
├── bert_0.2.0_aarch64.dmg              # macOS ARM64 (M1/M2/M3)
├── bert_0.2.0_x64.dmg                  # macOS Intel  
├── BERT_0.2.0_x64-setup.msi            # Windows installer
├── bert-tauri.exe                      # Windows portable
├── bert_0.2.0_amd64.AppImage           # Linux universal
└── bert_0.2.0_amd64.deb                # Linux Debian package
```

## Workflow Configuration

The release workflow (`.github/workflows/release.yml`) includes:

- **Cross-platform builds**: 4 matrix combinations
- **Automatic icon generation**: Handles missing macOS icons
- **Dependency management**: Installs all required tools
- **Caching**: Speeds up subsequent builds
- **Draft releases**: Allows review before publishing

## Prerequisites

### Repository Setup
- Workflow file in `.github/workflows/release.yml` ✅
- All required icons in `src-tauri/icons/` ✅  
- Version in `src-tauri/tauri.conf.json` ✅

### GitHub Secrets (Optional)
For signed releases, add to GitHub repo secrets:
- `APPLE_CERTIFICATE` - macOS code signing
- `WINDOWS_CERTIFICATE` - Windows code signing  
- `SIGNING_PASSWORD` - Certificate passwords

## Testing the Workflow

### Test Build (No Release)
Create a test branch and modify the workflow to remove the release step:

```bash
git checkout -b test-ci-build
# Edit .github/workflows/release.yml
# Remove the tauri-action release portions
git push origin test-ci-build  
```

### Full Release Test  
Use a pre-release version:
```bash
git tag v0.2.0-beta1
git push origin v0.2.0-beta1
```

## Troubleshooting

### Common Issues

#### Build Failures
- **Missing icons**: Workflow auto-generates macOS icons
- **Dependency issues**: Check platform-specific install steps
- **Rust compilation**: Ensure `Cargo.toml` versions are compatible

#### Release Issues
- **Permission denied**: Ensure `GITHUB_TOKEN` has write permissions
- **Tag conflicts**: Delete and recreate tags if needed
- **Draft not created**: Check workflow logs for errors

### Debug Commands
```bash
# Check workflow status
gh workflow list
gh workflow view "Release Desktop App"
gh run list --workflow=release.yml

# Download build artifacts for testing
gh release download v0.2.0 --pattern "*.dmg"
```

## Version Management

When preparing a release:

1. **Update version** in `src-tauri/tauri.conf.json`
2. **Update version** in `src-tauri/Cargo.toml`  
3. **Create release notes** (see Release Notes Guide)
4. **Tag and push** → Automatic builds begin

## Local Testing Before Release

```bash
# Test builds locally first
cargo tauri build --no-bundle  # Quick binary test
cargo tauri build --bundles dmg # Full bundle test (macOS)
```

## Post-Release

After successful CI build:
1. **Review draft release** on GitHub
2. **Test download links** and installers
3. **Edit release notes** if needed
4. **Publish release** when ready
5. **Announce** to users/community

---

**Status**: ✅ Multi-platform CI/CD workflow configured
**Supports**: Windows, macOS (Intel + ARM), Linux  
**Build Time**: ~10 minutes for all platforms
**Last Updated**: August 19, 2024