# BERT UI Consistency Issues
*Tracking UI/UX inconsistencies and polish items discovered during manual modeling*

**Created**: 2025-08-05  
**Purpose**: Document UI inconsistencies for future fixes  
**Priority**: Polish items for v0.2.1 or v0.3.0

---

## 🔴 CRITICAL PRIORITY - Spatial Rendering Bug

### Issue #6: CRITICAL - Zoom-Triggered Spatial Layout Collapse
**Location**: ALL MODELS when zooming in/out  
**Current**: Zoom operations cause complete spatial layout failure - systems overlap chaotically  
**Impact**: BLOCKS ALL modeling work - makes BERT unusable for complex workflows  
**Trigger**: Zoom in/out operations on any model (not complexity-dependent)  
**Status**: 🔴 ACTIVE CRITICAL BUG - BERT unusable for complex work  
**Severity**: CRITICAL - Prevents any serious modeling work  
**Evidence**: Screenshots show complete system overlap, no spatial constraints  
**Root Cause**: Camera/viewport coordinate system failure during zoom operations  
**Affects**: All models, not just complex ones with subsystems  
**Workaround**: Avoid zooming - work at default zoom level only  
**Fix Required**: Debug Bevy camera/transform system during zoom operations

---

## 🔴 HIGH PRIORITY - Field Ordering Issues

### Issue #1: Boundary Details Panel Field Order
**Location**: Boundary Details Panel (when clicking boundary ring)  
**Current**: Description appears before Name  
**Expected**: Name should appear first (consistent with all other panels)  
**Impact**: Breaks user expectation and muscle memory  
**Fix**: Reorder fields in boundary details component to match standard pattern

---

## 🟡 MEDIUM PRIORITY - Spatial Interaction Issues

### Issue #2: Subsystem Attachment Complexity
**Location**: Automated subsystem creation from JSON  
**Current**: Subsystems created via JSON often spawn disconnected in center  
**Expected**: Subsystems should properly attach to parent interfaces  
**Impact**: Models with subsystems may not load with proper connections  
**Workaround**: Manual creation using GUI workflow  
**Fix**: Review spawn_loaded_subsystem logic and parent_interface handling

### Issue #4: Legacy Fields in Manual Guide
**Location**: Manual modeling guide flow configuration  
**Current**: Guide referenced Substance Sub Type, Amount, and Unit fields  
**Expected**: Only show fields actually available in simplified UI  
**Status**: ✅ FIXED - Removed legacy field references  
**Note**: These fields still exist in JSON but aren't exposed in current UI

### Issue #5: CRITICAL - Subsystem Creation Workflow Error
**Location**: Both manual modeling guides  
**Current**: Guides incorrectly described creating flows first, then subsystems spawn at intersections  
**Expected**: Click green circle buttons on interfaces to spawn subsystems directly  
**Impact**: CRITICAL - Completely wrong workflow instructions  
**Status**: ✅ FIXED - Updated both guides to show correct green circle workflow  
**Resolution**: Subsystems spawn directly from interface green circle buttons, not from flow creation

---

## 🟢 LOW PRIORITY - Documentation Gaps

### Issue #3: Spatial Interaction Not Documented
**Location**: Manual modeling guide  
**Current**: Guide didn't mention clicking boundary ring for boundary details  
**Expected**: Clear documentation of spatial interaction features  
**Status**: ✅ FIXED - Updated in both guides  
**Resolution**: Added explicit instructions about boundary ring clicking

---

## 📝 DISCOVERED PATTERNS

### Positive Patterns (Keep These)
1. **Spatial separation works well** - System interior vs boundary ring vs environment clicking
2. **Visual feedback on hover** - Boundary ring highlighting helps discoverability
3. **Outputs-before-inputs workflow** - Enforces good modeling practice
4. **Auto-spawning subsystems** - Reduces manual positioning work

### Needs Improvement
1. **Field ordering consistency** - Name should always be first
2. **Subsystem attachment from JSON** - Complex models don't load properly
3. **Save feedback** - No clear indication when save succeeds
4. **Undo/Redo** - No way to reverse actions

---

## 🔧 SUGGESTED FIXES

### Quick Fixes (< 1 hour each)
1. **Reorder boundary panel fields** - Simple component change
2. **Add save success toast** - Visual confirmation of save
3. **Standardize all panel field orders** - Systematic review and fix

### Medium Fixes (2-4 hours)
1. **Fix subsystem attachment** - Review spawn logic
2. **Add keyboard shortcuts documentation** - In-app help
3. **Improve error messages** - More helpful validation feedback

### Major Enhancements (> 1 day)
1. **Implement undo/redo system** - Command pattern for reversibility
2. **Add model validation panel** - Real-time model health checks
3. **Create interactive tutorial** - Guided first-time user experience

---

## 📊 TRACKING METRICS

### UI Consistency Score
- **Field Ordering**: 4/5 panels consistent (80%)
- **Visual Feedback**: Good hover states, missing save confirmation
- **Workflow Clarity**: Improved with spatial interaction
- **Error Handling**: Needs improvement

### User Experience Notes
- **Spatial interaction is intuitive** once discovered
- **Outputs-first workflow is educational** but needs clear documentation
- **Manual creation works well**, automated creation needs work
- **Save operation needs better feedback**

---

## 🚀 NEXT STEPS

1. **Immediate**: Continue documenting issues during manual creation
2. **Short-term**: Fix field ordering in boundary panel
3. **Medium-term**: Improve subsystem attachment logic
4. **Long-term**: Implement undo/redo and validation systems

## 🔮 FUTURE EXPLORATION - LLM Integration

### Session End Discussion Topic: LLM-Assisted Knowledge Synthesis
**Context**: BERT has a live `llm-feature` branch (not yet merged)
**Potential**: LLM integration could bridge the gap between current "User Cognitive Process" placeholder and actual automated knowledge synthesis

**Exploration Areas**:
- **Session Analysis**: LLM analyzes modeling patterns and suggests insights about system relationships
- **Learning Synthesis**: Generate summaries of insights developed during modeling sessions  
- **Theory Connection**: Identify connections between user models and systems science principles
- **Model Enhancement**: Suggest improvements based on systems theory best practices
- **Explanatory Generation**: Create educational descriptions of modeled systems

**Systems Science Interest**: Recursive AI assistance - using AI system to help understand other systems through modeling process

**Status**: Tagged for end-of-session exploration and future development consideration

---

*This document will be updated as new issues are discovered during BERT usage*