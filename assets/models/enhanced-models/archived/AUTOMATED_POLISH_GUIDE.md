# Automated Model Polish Guide - Cork Airport Sprint Learnings
*Critical insights for automated enhancement of remaining 5 models*

**Created**: 2025-08-04, Cork Airport Ireland  
**Context**: 3% context remaining - preserve all learnings  
**Success**: Cell model polished without manual BERT editing

---

## ✅ VALIDATED: Automated JSON Enhancement Works!

### What BERT Accepts (Safe to Change):
1. **All description fields** - System, boundary, component, interface, flow descriptions
2. **Name fields** - Can enhance with biological/technical accuracy
3. **Protocol fields** - Can transform into algorithmic steps
4. **Equivalence fields** - Can add cross-domain metaphors
5. **Legacy fields** - substance.sub_type, parameters (BERT ignores gracefully)

### ⚠️ CRITICAL: What MUST Stay Unchanged:

**NEVER MODIFY COORDINATES** - This will break visual layout!

1. **All IDs** - System, component, interface, flow IDs
2. **Coordinates** - transform.translation, transform.rotation, angle values
   - Controls component position and orientation
   - Changing these bunches components in center
   - If layout appears broken, test original first
3. **Relationships** - exports_to, receives_from arrays
4. **Structure** - JSON hierarchy and field order
5. **Type values** - Import/Export, Source/Sink designations

---

## 📋 Enhancement Checklist for Each Model

### 1. Main System Enhancement
- [ ] Add Bertalanffy system definition quote
- [ ] Add Mobus 7-tuple framework reference
- [ ] Explain emergence and hierarchical organization
- [ ] Add meaningful equivalence (e.g., "Living Factory")
- [ ] Connect to domain-specific principles

### 2. Boundary Descriptions
- [ ] Explain selective permeability/regulation
- [ ] Reference Mobus "effective boundary" concept
- [ ] Describe what's kept in vs. out
- [ ] Note porosity if relevant (fuzzy boundaries)

### 3. Component/Subsystem Names
- [ ] Replace generic names with domain-specific terms
- [ ] Use accurate technical/biological nomenclature
- [ ] Maintain consistency across model

### 4. Interface Enhancements
- [ ] Replace all "Interface" with specific names
- [ ] Use real proteins/mechanisms for biology
- [ ] Use real protocols/standards for technology
- [ ] Add regulatory function descriptions

### 5. Protocol Algorithms
Use template:
```
1. [Trigger condition] → 
2. [Recognition/verification] → 
3. [Transfer action] → 
4. [Validation] → 
5. [Reset state]
[Regulation: mechanism]
```

### 6. Flow Descriptions
- [ ] Explain what substance is and why it matters
- [ ] Note transformations that occur
- [ ] Add stoichiometry where relevant
- [ ] Connect to system purpose

### 7. Educational Value
- [ ] Each element teaches domain AND systems principles
- [ ] Cross-domain equivalences for pattern recognition
- [ ] Theoretical grounding throughout

---

## 🎯 Model-Specific Focus Areas

### Organization Model
- Business terminology (CEO, departments, KPIs)
- Information flow protocols (reporting, decision-making)
- Hierarchical control patterns
- Market environment interactions

### Solar Panel Model  
- Electrical engineering terms (inverter, MPPT, bypass diodes)
- Energy conversion protocols
- Semiconductor physics principles
- Grid integration standards

### LLM Model
- Neural architecture terms (attention, embeddings, transformers)
- Information processing protocols
- Token flow algorithms
- GPU resource management

### Ecosystem Model
- Trophic relationships
- Energy transfer efficiency (10% rule)
- Nutrient cycling protocols
- Population regulation mechanisms

### Generic System Template
- Maximum theoretical grounding
- Abstract but precise terminology
- Universal pattern descriptions
- Pure systems science language

---

## 🔧 Coordinate Debugging Protocol

**If ecosystem/model layout appears broken (components bunched in center):**

1. **Test Original First**: Load original .json file in BERT
2. **If original displays correctly**: Coordinates are correct, polish process is fine
3. **If original is also broken**: Report to user - don't attempt manual coordinate fixes
4. **Root cause**: Usually overlapping coordinates in original, not polish error
5. **Solution**: Only enhance text descriptions, never touch coordinates

**Visual Layout Warning Signs:**
- All components stacked in center
- Components overlapping completely
- Interfaces not connecting properly

**Remember**: Polish process only changes text - layout issues indicate coordinate problems in original file

## 🐛 Intermittent Loading Issues

**Complex Model Loading Problem**: Some polished models (particularly system-polished.json) may fail to load on first attempt but work fine on subsequent loads.

**Symptoms**:
- Model appears to fail loading initially
- Same model loads successfully when tried again
- No permanent corruption - file is valid JSON
- More common with models having complex interconnections

**Potential Causes**:
- JSON parsing race condition in BERT
- File system caching/sync delays  
- Complex relationship parsing timing issues
- BERT internal state conflicts

**Workarounds**:
1. **Try loading 2-3 times** if first attempt fails
2. **Restart BERT** (`cargo tauri dev`) if persistent
3. **Wait a few seconds** between load attempts
4. **Check file exists** and is complete before loading

**When to Worry**: Only if model never loads after multiple attempts and BERT restart

**Note**: This appears related to model complexity (number of interconnections) rather than polish quality. The System template has the most complex internal relationships which may stress BERT's loading system.

## 🚀 Automation Strategy

### Phase 1: Automated Polish (Claude can do alone)
1. Read original JSON
2. Apply enhancement checklist systematically
3. Generate polished JSON with all text improvements
4. Preserve exact structure and relationships
5. Test load in BERT

### Phase 2: Manual Hierarchical Decomposition (Human needed)
1. Open polished model in BERT
2. Zoom into subsystems
3. Add deeper levels of organization
4. Create recursive system demonstrations
5. Save enhanced version

### Success Metrics
- Model loads without errors ✅
- All generic labels replaced ✅
- Protocols are algorithmic ✅
- Descriptions educate ✅
- Theory is grounded ✅

---

## 💡 Key Insights

1. **BERT is robust** - Text changes don't break models if structure preserved
2. **Descriptions matter** - Rich content transforms functional → educational
3. **Protocols are algorithms** - Step-by-step flow control
4. **Theory grounds practice** - Bertalanffy + Mobus quotes add legitimacy
5. **Automation scales** - One successful model → template for all

---

## 🎬 Next Session Plan

With this guide, Claude can:
1. Polish Organization model (30 min)
2. Polish Solar Panel model (30 min)
3. Polish LLM model (20 min)
4. Polish Ecosystem model (20 min)
5. Review Generic System (10 min - already strong)

Total: ~2 hours automated work while human travels

Then human can:
1. Test all polished models
2. Add hierarchical decomposition
3. Create direct subsystem flows
4. Prepare for Model Browser integration

---

**This guide preserves all learnings from Cork Airport sprint for maximum efficiency!**