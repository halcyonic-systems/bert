# BERT Field Status Audit

## Executive Summary

This audit categorizes all user-facing fields in BERT by their implementation maturity and research status. The goal is to distinguish between **core fields** (stable, well-specified) and **optional fields** (requiring further research/specification) to improve user experience and prevent confusion.

## Audit Methodology

- **Source Analysis**: Examined `src/leptos_app/details.rs` (UI components) and `src/bevy_app/data_model/mod.rs` (data structures)
- **Field Classification**: Based on implementation completeness, enum constraints, and systems science maturity
- **UI Impact Assessment**: Evaluated how field status affects user interaction patterns

---

## Field Classification Results

### 🟢 **CORE FIELDS** (Stable & Well-Specified)

#### **Interaction/Flow Fields**
| Field | Type | Status | Notes |
|-------|------|--------|-------|
| **Name** | Text Input | ✅ Core | Standard naming, fully implemented |
| **Description** | Text Area | ✅ Core | Standard documentation field |
| **Interaction Usability** | Select (4 options) | ✅ Core | Well-defined enum: Resource, Disruption, Product, Waste |
| **Interaction Type** | Select (2 options) | ✅ Core | Well-defined enum: Flow, Force |
| **Substance Type** | Select (3 options) | ✅ Core | Well-defined enum: Energy, Material, Message |
| **Substance Amount** | Decimal Input | ✅ Core | Quantitative, well-specified |
| **Substance Unit** | Text Input | ✅ Core | Standard measurement units |
| **Parameters** | Dynamic List | ✅ Core | Extensible key-value pairs with units |

#### **System Fields**
| Field | Type | Status | Notes |
|-------|------|--------|-------|
| **Name** | Text Input | ✅ Core | Standard naming |
| **Description** | Text Area | ✅ Core | Standard documentation |
| **Complexity - Adaptable** | Checkbox | ✅ Core | Boolean flag, well-defined |
| **Complexity - Evolveable** | Checkbox | ✅ Core | Boolean flag, well-defined |
| **Boundary - Porosity** | Slider (0-1) | ✅ Core | Quantitative, bounded range |
| **Boundary - Perceptive Fuzziness** | Slider (0-1) | ✅ Core | Quantitative, bounded range |

#### **Interface Fields**
| Field | Type | Status | Notes |
|-------|------|--------|-------|
| **Name** | Text Input | ✅ Core | Standard naming |
| **Description** | Text Area | ✅ Core | Standard documentation |
| **Protocol** | Text Input | ✅ Core | Technical specification field |

#### **External Entity Fields**
| Field | Type | Status | Notes |
|-------|------|--------|-------|
| **Name** | Text Input | ✅ Core | Standard naming |
| **Description** | Text Area | ✅ Core | Standard documentation |
| **Model** | Text Input | ✅ Core | Reference/specification field |

---

### 🟡 **OPTIONAL FIELDS** (Research/Development Needed)

#### **System Fields Needing Specification**
| Field | Type | Status | Issues |
|-------|------|--------|--------|
| **Equivalence** | Text Input | ⚠️ Optional | No clear specification or validation |
| **Time Unit** | Text Input | ⚠️ Optional | Should be standardized dropdown |
| **History** | Text Input | ⚠️ Optional | Unclear format/structure requirements |
| **Transformation** | Text Input | ⚠️ Optional | Needs formal specification |
| **Boundary Name** | Text Input | ⚠️ Optional | Unclear when/how to use |
| **Boundary Description** | Text Input | ⚠️ Optional | Overlaps with system description |
| **Environment Name** | Text Input | ⚠️ Optional | Rarely used, unclear purpose |
| **Environment Description** | Text Input | ⚠️ Optional | Rarely used, unclear purpose |

#### **External Entity Fields Needing Specification**
| Field | Type | Status | Issues |
|-------|------|--------|--------|
| **Equivalence** | Text Input | ⚠️ Optional | No clear specification or examples |

#### **SubSystem Fields Needing Specification**
| Field | Type | Status | Issues |
|-------|------|--------|--------|
| **Complexity Type** | Select | ⚠️ Optional | Complex/Atomic/Multiset - needs better UX |
| **Membership** | Slider | ⚠️ Optional | Unclear purpose and range meaning |
| **Equivalence** | Text Input | ⚠️ Optional | Duplicate of system equivalence issue |
| **Time Unit** | Text Input | ⚠️ Optional | Same standardization issue as system |
| **History** | Text Input | ⚠️ Optional | Same specification issue as system |
| **Transformation** | Text Input | ⚠️ Optional | Same specification issue as system |

#### **Interaction Fields Needing Specification**
| Field | Type | Status | Issues |
|-------|------|--------|--------|
| **Substance Sub Type** | Text Input | ⚠️ Optional | Free text, should be constrained/standardized |

---

## Recommended UX Improvements

### **Phase 1: Visual Distinction (Immediate)**
```css
/* Core fields - standard styling */
.field-core {
    border: 2px solid #10b981; /* Green border */
    background: #f0fdf4;
}

/* Optional fields - research styling */
.field-optional {
    border: 2px dashed #f59e0b; /* Amber dashed border */
    background: #fffbeb;
}

.field-optional::after {
    content: "⚠️ Research Field";
    font-size: 0.75rem;
    color: #d97706;
    float: right;
}
```

### **Phase 2: Progressive Disclosure**
- **Default View**: Show only core fields
- **Advanced Toggle**: "Show Research Fields" checkbox
- **Tooltips**: Explain field status and usage guidance

### **Phase 3: Field Standardization**
1. **Time Unit**: Convert to dropdown (seconds, minutes, hours, days, years)
2. **Substance Sub Type**: Create constrained vocabulary
3. **Equivalence**: Develop specification and examples
4. **History/Transformation**: Create structured input formats

---

## Implementation Strategy

### **Phase 1: Visual Distinction (1-2 hours with AI agents)**
1. Add CSS classes to distinguish field types
2. Add visual indicators (icons, borders)
3. Group fields by status in UI
4. Apply styling across all detail components

### **Phase 2: Progressive Disclosure (2-4 hours with AI agents)**
1. Implement "Show Research Fields" toggle
2. Add comprehensive tooltips with field guidance
3. Create collapsible field sections
4. Update all UI components with new patterns

### **Phase 3: Field Standardization (1-2 days with AI agents)**
1. Convert Time Unit to standardized dropdown
2. Create constrained vocabulary for Substance Sub Type
3. Develop specification templates for research fields
4. Implement field validation systems

### **Phase 4: Advanced UX (2-3 days with AI agents)**
1. Create interactive field specification documentation
2. Implement context-sensitive help system
3. Add field usage analytics and recommendations
4. Develop field maturity progression workflows

**AI Agent Acceleration Benefits:**
- **Parallel Implementation**: Multiple components updated simultaneously
- **Pattern Consistency**: Automated application of UX patterns across codebase
- **Comprehensive Testing**: Automated edge case coverage
- **Documentation Generation**: Auto-generated field specifications and user guides

---

## Field Count Summary

| Category | Core Fields | Optional Fields | Total |
|----------|-------------|-----------------|-------|
| **Interaction** | 8 | 1 | 9 |
| **System** | 6 | 8 | 14 |
| **Interface** | 3 | 0 | 3 |
| **External Entity** | 3 | 1 | 4 |
| **SubSystem** | 0 | 7 | 7 |
| **TOTAL** | **20** | **17** | **37** |

**Status**: 54% of fields are core/stable, 46% need research/specification work.

---

## Next Steps

### **Immediate Actions (AI Agent Ready)**
1. **Phase 1 Implementation**: Visual field distinction (1-2 hours)
2. **Component Analysis**: Identify all UI components needing updates
3. **CSS Framework**: Establish field status styling system
4. **Pattern Library**: Create reusable field status components

### **Short Term (AI Agent Accelerated)**
1. **Progressive Disclosure**: Implement advanced/research field toggle
2. **Field Specifications**: Auto-generate documentation for optional fields
3. **Validation Framework**: Create field constraint and validation system
4. **User Testing**: Rapid prototype and iterate with field status UX

### **Development Approach**
- **AI-First**: Leverage Cursor agents for parallel component updates
- **Pattern-Driven**: Establish consistent UX patterns across all field types
- **Specification-Guided**: Use this audit as the definitive field classification
- **User-Centered**: Focus on preventing confusion while maintaining power-user access

**Total Estimated Timeline**: **3-5 days** (vs. 1-2 months without AI agents)

This audit provides the foundation for implementing clear field status UX that will prevent user confusion and guide appropriate usage of BERT's modeling capabilities. 