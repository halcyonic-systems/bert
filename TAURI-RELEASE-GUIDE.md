# BERT Tauri Release Build Guide

> **Status**: Partial Success - Bundle generation working, but development environment compromised  
> **Date**: August 19-20, 2025  
> **Context**: Full day of systematic debugging to achieve cross-platform Tauri releases

## 🎯 **WHAT WE ACHIEVED**

### ✅ **Successful Bundle Generation**
- **macOS Intel**: ✅ Complete success - `.dmg` and `.app` files generated and uploaded
- **macOS ARM**: ✅ Bundle created but executable corrupted (cross-compilation issue)
- **Linux**: ❌ Failed on icon path issue (fixable)
- **Windows**: 🔄 Build pipeline working but not fully tested

### ✅ **Breakthrough Discoveries**
1. **Platform-specific bundle types required**: `--bundles app dmg` not `--bundles all`
2. **Real bundle generation takes 25-40 minutes**, not 15 minutes
3. **GitHub Actions cache behavior**: Initial long compiles, then faster subsequent builds
4. **Tauri-action artifact discovery**: Requires proper `package.json` name/version fields

## 🚨 **CRITICAL ISSUES INTRODUCED**

### ❌ **Development Environment Broken**
- **Local `cargo tauri dev` now has severe UI layout issues**
- **Same issues appear in both local dev and Intel release builds**
- **Problems did NOT exist before release work began**
- **Must revert to working state before continuing releases**

## 📋 **COMPLETE SOLUTION ROADMAP**

### **Phase 1: Recovery (IMMEDIATE)**
1. **Backup current state**: `git checkout -b backup-release-progress`
2. **Identify last working commit**: Find commit before release work started
3. **Revert to working local dev**: `git reset --hard <working-commit>`
4. **Verify local dev works**: Confirm UI layout is normal
5. **Document the specific breaking changes**

### **Phase 2: Incremental Release Fixes**
Apply changes ONE AT A TIME, testing local dev after each:

#### **Change 1: Package.json Name/Version**
```json
{
  "name": "bert",
  "version": "0.2.0",
  "dependencies": { ... }
}
```
**Test**: Verify local dev still works after this change

#### **Change 2: Bundle Configuration**
```json
"bundle": {
  "active": true,
  "targets": "all",
  "icon": [ ... ]
}
```
**Test**: Verify local dev still works after this change

#### **Change 3: Platform-Specific Bundle Matrix**
```yaml
matrix:
  include:
    - platform: 'macos-latest'
      args: '--target aarch64-apple-darwin'
      bundles: '--bundles app dmg'
    - platform: 'macos-latest'
      args: '--target x86_64-apple-darwin'
      bundles: '--bundles app dmg'
    - platform: 'ubuntu-22.04'
      args: ''
      bundles: '--bundles deb appimage'
    - platform: 'windows-latest'
      args: ''
      bundles: '--bundles msi nsis'
```

## 🔧 **SYSTEMATIC DEBUGGING METHODOLOGY**

### **The Process That Worked**
1. **Start with simplest approach**: Try config-file solutions first
2. **Document each iteration**: test2 → test13 systematic progression
3. **Fix one error at a time**: Don't compound multiple changes
4. **Read error messages carefully**: Platform-specific requirements revealed in errors
5. **Expect long compile times**: Real bundle generation is expensive (25-40 min)
6. **Platform-specific testing**: Each platform has unique issues

### **Key Error Patterns We Solved**

#### **"No artifacts were found"**
- **Cause**: Missing `package.json` name/version fields
- **Solution**: Add `"name": "bert"` and `"version": "0.2.0"`

#### **"Invalid value 'all' for --bundles"**
- **Cause**: `--bundles all` doesn't exist
- **Solution**: Platform-specific bundle types:
  - macOS: `app dmg`
  - Linux: `deb appimage`  
  - Windows: `msi nsis`

#### **Linux Icon Path Error**
- **Error**: `resource path 'icons/icon.icns' doesn't exist`
- **Cause**: Linux trying to use macOS icon format
- **Solution**: Linux-specific icon configuration (needs proper syntax)

#### **ARM Binary Corruption**
- **Problem**: ARM builds complete but executable is damaged
- **Cause**: Cross-compilation issues in GitHub Actions
- **Status**: Unsolved - needs investigation

## 🏗️ **WORKING GITHUB ACTIONS CONFIGURATION**

### **Successful Release Workflow Structure**
```yaml
name: "Release Desktop App"
on:
  push:
    tags: ["v*"]

jobs:
  publish-tauri:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest'
            args: '--target aarch64-apple-darwin'
            bundles: '--bundles app dmg'
          - platform: 'macos-latest'
            args: '--target x86_64-apple-darwin'
            bundles: '--bundles app dmg'
          - platform: 'ubuntu-22.04'
            args: ''
            bundles: '--bundles deb appimage'
          - platform: 'windows-latest'
            args: ''
            bundles: '--bundles msi nsis'

    runs-on: ${{ matrix.platform }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      # ... Linux dependencies, Rust setup, Node setup, icon generation ...

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0.5  # Stable version
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ github.event.inputs.version || github.ref_name }}
          releaseName: 'BERT ${{ github.event.inputs.version || github.ref_name }}'
          releaseBody: 'See the assets to download this version and install.'
          releaseDraft: true
          prerelease: false
          args: '${{ matrix.args }} ${{ matrix.bundles }}'
          projectPath: '.'
          tauriScript: 'cargo tauri'
```

### **Required Files for Bundle Generation**
```
package.json:
{
  "name": "bert",           ← REQUIRED for artifact naming
  "version": "0.2.0",       ← REQUIRED for artifact naming
  "dependencies": { ... }
}

src-tauri/tauri.conf.json:
{
  "productName": "bert",     ← Must match package.json name
  "version": "0.2.0",        ← Must match package.json version
  "bundle": {
    "active": true,          ← REQUIRED for bundle generation
    "targets": "all",        ← Platform bundles
    "icon": [ ... ]
  }
}
```

## ⚠️ **TAURI GOTCHAS DISCOVERED**

### **Bundle Generation Tribal Knowledge**
1. **Config files ignored in CI**: `bundle.active=true` often ignored, need explicit `--bundles` flags
2. **Platform-specific bundle types**: No universal `all` option exists
3. **Long compile times normal**: 25-40 minutes for real bundle generation vs 15 min for binary-only
4. **Cache behavior**: First builds slow, subsequent builds faster due to GitHub Actions cache
5. **Naming consistency critical**: package.json name must match tauri.conf.json productName

### **macOS Code Signing Issues**
- **Unsigned apps show "damaged" errors** on Apple Silicon Macs
- **Intel builds run via Rosetta** but with UI scaling problems
- **ARM cross-compilation** often produces corrupted executables
- **Security overrides**: `sudo xattr -cr /path/to/app.app` to bypass Gatekeeper

### **Cross-Platform Icon Requirements**
- **macOS**: Needs `.icns` file
- **Windows**: Needs `.ico` file with multiple resolutions
- **Linux**: Needs `.png` files only, fails on `.icns`

## 🔍 **UNSOLVED PROBLEMS**

### **1. ARM Cross-Compilation Corruption**
- **Symptom**: ARM builds complete but executable is damaged/corrupted
- **Impact**: Apple Silicon Macs can't run native builds
- **Workaround**: Intel builds work via Rosetta (with UI issues)
- **Next Steps**: Investigate GitHub Actions ARM cross-compilation setup

### **2. Development Environment Breaking** ✅ SOLVED
- **Symptom**: Changes that enable releases break local development
- **Root Cause**: Changed `rel="tailwind-css"` to `rel="css"` in index.html
- **Impact**: Completely broke UI layout in both dev and release builds
- **Solution**: Revert to `rel="tailwind-css"` for working local development
- **New Challenge**: Find way to make GitHub Actions work with tailwind-css attribute

### **3. UI Layout Issues**
- **Symptom**: Severe layout problems in both local dev and release builds
- **Impact**: App not usable for testing
- **Scope**: Affects both development and releases
- **Next Steps**: Debug after fixing core development environment

## 📈 **SUCCESS METRICS ACHIEVED**

### **Bundle Generation Pipeline**
- ✅ **Proof of concept**: Bundle generation works end-to-end
- ✅ **Cross-platform**: Successfully building for multiple platforms
- ✅ **Artifact upload**: Files correctly uploaded to GitHub releases
- ✅ **Systematic debugging**: Methodical approach successfully identified and solved multiple issues

### **Tauri Knowledge Base**
- ✅ **Platform-specific requirements documented**
- ✅ **Common error solutions identified**
- ✅ **GitHub Actions workflow patterns established**
- ✅ **Community tribal knowledge captured**

## 🎯 **RECOMMENDED NEXT STEPS**

### **Immediate Priority (This Session)**
1. **Revert to working local dev** - Find and revert breaking changes
2. **Document the breaking change** - Identify what specifically broke development
3. **Test incremental approach** - Apply one change at a time

### **Short Term (Next Session)**
1. **Fix Linux icon issue** with proper syntax
2. **Investigate ARM cross-compilation** corruption
3. **Complete Windows build testing**

### **Medium Term**
1. **Solve development/release environment conflict**
2. **Implement proper code signing** for macOS
3. **Fix UI layout issues** affecting usability

## 📝 **LESSON LEARNED**

### **Critical Success Factors**
1. **Systematic approach works** - Incremental debugging test2→test13 was effective
2. **Platform-specific knowledge essential** - Generic solutions often fail
3. **Community knowledge invaluable** - Tauri gotchas are well-documented pain points
4. **Testing local dev mandatory** - Changes can have unexpected side effects

### **Avoid These Mistakes**
1. **Don't compound multiple changes** - One change at a time with testing
2. **Don't trust "successful" builds** - Verify artifacts actually work
3. **Don't ignore local development** - Release changes can break development environment
4. **Don't assume config files work** - CI/CD often needs explicit command flags

---

**This guide preserves the complete journey from broken releases to working bundle generation, including all the tribal knowledge discovered and the systematic debugging approach that proved successful.**