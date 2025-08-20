# 🚀 BERT Tauri Release - Tomorrow Morning Action Plan

> **Date**: August 20, 2025  
> **Status**: Ready to execute - all groundwork complete  
> **Goal**: Apply proven release fixes incrementally without breaking development

## 🌅 **MORNING STARTUP CHECKLIST**

### **✅ Pre-Work Verification**
```bash
# 1. Verify we're in the right place
cd /Users/home/Desktop/halcyonic/bert/bert

# 2. Confirm main branch is working
git status
cargo tauri dev
# → Should show perfect UI, no layout issues
# → If broken, something went wrong overnight

# 3. Check our knowledge base exists
ls -la TAURI-RELEASE-GUIDE.md
# → Should be 300+ lines of complete documentation
```

## 🌿 **PHASE 1: SAFE BRANCH SETUP (5 minutes)**

### **Step 1: Create Feature Branch**
```bash
git checkout main
git pull origin main  # Get any overnight changes
git checkout -b feature/tauri-releases
git push -u origin feature/tauri-releases
```

### **Step 2: Verify Clean Slate**
```bash
cargo tauri dev
# → Confirm UI still works perfectly on branch
# → If broken, stop and investigate
```

## 🔧 **PHASE 2: INCREMENTAL RELEASE FIXES**

> **Rule**: Test local dev after EVERY change. If it breaks, revert immediately.

### **Change 1: Package.json Name/Version (10 minutes)**

**Apply:**
```bash
# Edit package.json - add these fields:
{
  "name": "bert",
  "version": "0.2.0",
  "dependencies": { ... existing ... }
}
```

**Test:**
```bash
cargo tauri dev
# → UI should still work perfectly
# → If broken: git checkout package.json and investigate
```

**Commit:**
```bash
git add package.json
git commit -m "feat: add package.json name/version for release artifact naming

- Add name: bert (matches tauri.conf.json productName)
- Add version: 0.2.0 (matches tauri.conf.json version)
- Required for tauri-action artifact discovery
- Local dev verified working"
```

### **Change 2: Bundle Configuration (10 minutes)**

**Apply:**
```bash
# Edit src-tauri/tauri.conf.json - add to bundle section:
"bundle": {
  "active": true,
  "targets": "all",
  "icon": [ ... existing ... ]
}
```

**Test:**
```bash
cargo tauri dev
# → UI should still work perfectly
# → If broken: revert tauri.conf.json and investigate
```

**Commit:**
```bash
git add src-tauri/tauri.conf.json
git commit -m "feat: enable bundle generation in tauri config

- Add bundle.active: true to trigger bundle creation
- Add bundle.targets: all for cross-platform bundles
- Local dev verified working"
```

### **Change 3: Workflow Platform Matrix (10 minutes)**

**Apply:**
```bash
# Edit .github/workflows/release.yml
# Replace the matrix section with our proven working version:

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

# And update the args line to:
args: '${{ matrix.args }} ${{ matrix.bundles }}'
```

**Test:**
```bash
cargo tauri dev
# → Should have no effect on local dev
# → This only affects GitHub Actions
```

**Commit:**
```bash
git add .github/workflows/release.yml
git commit -m "feat: add platform-specific bundle matrix to workflow

- Add bundles field to matrix for each platform
- macOS: app dmg, Linux: deb appimage, Windows: msi nsis
- Use proven working configuration from test13 success
- Ready for release testing"
```

### **Change 4: Upgrade tauri-action Version (5 minutes)**

**Apply:**
```bash
# In .github/workflows/release.yml, change:
# uses: tauri-apps/tauri-action@v0
# to:
# uses: tauri-apps/tauri-action@v0.5
```

**Test:**
```bash
cargo tauri dev
# → Should have no effect on local dev
```

**Commit:**
```bash
git add .github/workflows/release.yml
git commit -m "feat: upgrade tauri-action to stable v0.5

- Change from @v0 to @v0.5 for better stability
- Fixes known artifact discovery issues
- Proven working in test13 success"
```

## 🧪 **PHASE 3: RELEASE TEST (30 minutes)**

### **Test Release Build**
```bash
# Create test tag
git tag v0.2.0-working-test1
git push origin v0.2.0-working-test1

# Monitor GitHub Actions for ~20-30 minutes
# Expected: Should work for macOS Intel and Linux
# Expected: macOS ARM might still have corruption issue
# Expected: Windows should work
```

### **If Release Test Succeeds**
```bash
# Celebrate! 🎉
# Document success in TAURI-RELEASE-GUIDE.md
# Plan next steps for remaining issues (ARM corruption, etc.)
```

### **If Release Test Fails**
```bash
# Don't panic - we have complete debugging guide
# Check TAURI-RELEASE-GUIDE.md for error patterns
# Debug incrementally on the branch
# Local dev is safe on main branch
```

## 🎨 **PHASE 4: SOLVE TAILWIND CONFLICT (If time permits)**

### **Research Options**

**Option A: Conditional Index.html**
- Different index.html for dev vs CI
- GitHub Actions step to swap files

**Option B: Trunk Version/Config**
- Update Trunk to newer version
- Research Trunk configuration for tailwind-css

**Option C: CI Preprocessing**
- GitHub Actions step: sed 's/tailwind-css/css/g' index.html
- Keeps dev environment untouched

### **Test Approach**
```bash
# Try each option on the branch
# Test local dev after each attempt
# Use git reset if anything breaks
```

## 🚨 **EMERGENCY PROCEDURES**

### **If Local Dev Breaks**
```bash
# Check what changed
git diff HEAD~1

# Revert last change
git reset --hard HEAD~1

# Or go nuclear back to main
git checkout main
git branch -D feature/tauri-releases
git checkout -b feature/tauri-releases
```

### **If Completely Stuck**
```bash
# You have complete documentation in:
# - TAURI-RELEASE-GUIDE.md (300+ lines)
# - CLAUDE.md (Tauri gotchas section)
# - This plan file

# All the hard debugging is done
# All solutions are documented
# All working configurations are preserved
```

## 📋 **SUCCESS CRITERIA**

### **Phase 2 Success**: 
- ✅ All changes applied incrementally
- ✅ Local dev working after each change
- ✅ Clean commit history on branch

### **Phase 3 Success**:
- ✅ GitHub Actions builds complete
- ✅ Artifacts generated and uploaded
- ✅ At least one platform fully working

### **Complete Success**:
- ✅ macOS Intel: Working installers
- ✅ Linux: Working installers  
- ✅ Windows: Working installers
- ✅ Local dev: Perfect UI

## 🎯 **MINDSET FOR TOMORROW**

### **Remember**:
- ✅ **90% of the work is done** - you proved bundle generation works
- ✅ **All hard debugging completed** - systematic approach was successful
- ✅ **Knowledge preserved** - complete documentation exists
- ✅ **Safe approach** - branch strategy prevents breaking main
- ✅ **Incremental progress** - test after every single change

### **You're Not Starting From Scratch**:
- Bundle generation: ✅ Proven working
- Platform targets: ✅ Proven working
- Artifact upload: ✅ Proven working
- Error solutions: ✅ All documented

### **Today Was a WIN**:
- Identified exact root cause of UI breakage
- Built complete release pipeline knowledge
- Restored working development environment
- Created systematic approach for tomorrow

## ⏱️ **ESTIMATED TIMELINE**

- **Phase 1**: 5 minutes
- **Phase 2**: 35 minutes  
- **Phase 3**: 30 minutes
- **Phase 4**: Variable (research/experimentation)

**Total**: ~70 minutes to working releases (if no surprises)

## 🚀 **GET STARTED COMMAND**

```bash
# Copy-paste this to start:
cd /Users/home/Desktop/halcyonic/bert/bert && \
git checkout main && \
git pull origin main && \
cargo tauri dev
```

**When UI looks perfect, you're ready to begin Phase 1! 🌟**

---

*This plan builds on proven success from yesterday's systematic debugging. All the hard work is done - tomorrow is just careful application of known solutions.*