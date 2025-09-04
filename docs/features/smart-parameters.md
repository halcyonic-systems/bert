# Feature: Smart Parameters

## Overview

**Feature Name**: Smart Parameters  
**Branch**: feature/smart-parameters  
**Status**: In Progress  
**Contributors**: Shingai Thornton, Claude  
**Date**: 2025-08-22 (Updated 2025-09-04 with Cliff's categorical variable framework)

## Description

Replace the clunky multi-field parameter approach from BERT 1.0 with a unified, intelligent parameter system that handles both quantitative measurements and qualitative characteristics. Eliminates redundant "Substance Unit" and "Substance Amount" fields by consolidating all flow properties into a smart parameter system that suggests appropriate data types and values based on substance type context.

### Enhanced with Categorical Variable Framework

Following Cliff's August 28th recommendations, Smart Parameters now supports a **hierarchy of three data types** for comprehensive system modeling:

- **Ordinal Variables**: Total order structure (high/medium/low, critical/important/optional)
- **Categorical Variables**: Small cardinality discrete categories (solid/liquid/gas, HTTP/TCP/UDP) 
- **Boolean Variables**: True/false states (active/inactive, bidirectional flows)
- **Numeric Variables**: Traditional quantified measurements with intelligent unit suggestions

## Implemented Functionality

### Core Smart Parameters Features
- Unified parameter table for all flow properties (quantitative AND qualitative)
- Context-aware parameter type suggestions based on substance type
- Intelligent data type selection (Numeric, Ordinal, Categorical, Boolean)
- Smart unit suggestions for numeric parameters (Energy → watts, BTU/hr, joules)
- Predefined option sets for categorical and ordinal parameters
- Custom parameter support for domain-specific needs
- Simplified substance type classification (remove Sub Type cognitive overhead)

### Categorical Variable Integration (Cliff's Framework)
- **Ordinal Parameters**: Hierarchical selections with total ordering (High/Medium/Low)
- **Categorical Parameters**: Discrete classification sets (Solid/Liquid/Gas)  
- **Boolean Parameters**: Simple binary states (Active/Inactive)
- **Numeric Parameters**: Enhanced unit intelligence for quantified measurements

## Technical Implementation

### Components Added

- **Parameter Type Engine**: Intelligent parameter type selection (Numeric/Ordinal/Categorical/Boolean)
- **Unit Suggestion Engine**: Smart unit recommendations for numeric parameters based on substance type context
- **Categorical Database**: Predefined option sets for ordinal and categorical parameters organized by domain
- **Unit Categories Database**: Organized collections of common units by domain (Mass, Energy, Information, etc.)
- **Parameter Validation System**: Ensures type-value compatibility and reasonable ranges across all parameter types

### Unit Suggestion Engine Implementation Options

#### Enhanced Data Structure (with Categorical Variables)
```rust
// Unified parameter suggestion system
struct SmartParameterDatabase {
    // Numeric parameters with unit suggestions  
    numeric_suggestions: HashMap<SubstanceType, Vec<NumericSuggestion>>,
    // Ordinal parameters with level options
    ordinal_suggestions: HashMap<SubstanceType, Vec<OrdinalSuggestion>>, 
    // Categorical parameters with option sets
    categorical_suggestions: HashMap<SubstanceType, Vec<CategoricalSuggestion>>,
    // Boolean parameters with contextual naming
    boolean_suggestions: HashMap<SubstanceType, Vec<BooleanSuggestion>>,
}

struct NumericSuggestion {
    display_name: String,     // "Temperature", "Flow Rate"
    unit_options: Vec<String>, // ["°C", "°F", "K"]
    search_terms: Vec<String>, // ["temp", "temperature", "heat"]
    category: UnitCategory,    // Temperature, Rate, Amount, etc.
}

struct OrdinalSuggestion {
    display_name: String,     // "Priority", "Efficiency"  
    levels: Vec<String>,      // ["High", "Medium", "Low"]
    search_terms: Vec<String>, // ["priority", "importance"]
}

struct CategoricalSuggestion {
    display_name: String,     // "Material State", "Protocol Type"
    options: Vec<String>,     // ["Solid", "Liquid", "Gas"]
    search_terms: Vec<String>, // ["state", "phase", "form"]
}

struct BooleanSuggestion {
    display_name: String,     // "Active", "Bidirectional"
    true_label: String,       // "Active", "Yes", "Enabled"
    false_label: String,      // "Inactive", "No", "Disabled"  
    search_terms: Vec<String>, // ["active", "enabled", "on"]
}
```

#### Implementation Approach Options

**Option 1: Static Database**
- Hardcoded unit categories in Rust code
- Fast, predictable, works offline
- Easy to maintain and version control
- Best for: MVP implementation, guaranteed performance

**Option 2: JSON Configuration**
- External JSON file with unit definitions
- Easier to extend without recompilation  
- Could support user-defined units
- Best for: Extensibility, domain-specific customization

**Option 3: Hybrid Smart Matching**
- Core units hardcoded for performance
- Pattern recognition for common physics relationships
- "power" + Energy context → suggests watts, horsepower
- "flow" + Material context → suggests kg/s, liters/min
- Best for: Intelligence while maintaining simplicity

#### Suggestion Logic Flow
1. **Context Detection**: User selects substance type (Energy/Material/Message)
2. **Input Analysis**: User types parameter name (e.g. "rat")  
3. **Fuzzy Matching**: Engine matches against common_names database
4. **Context Filtering**: Return only units relevant to selected substance type
5. **Ranking**: Most common units first, specialized units after
6. **Custom Option**: Always provide "Custom Unit" fallback

#### Enhanced User Interaction Examples (with Categorical Variables)
```
User types "eff" in Energy context:
→ [Ordinal] "Efficiency (Poor/Fair/Good/Excellent)" 
→ [Numeric] "Efficiency (%, ratio, decimal)"
→ "Custom parameter..."

User types "state" in Material context:
→ [Categorical] "Material State (Solid/Liquid/Gas)"
→ [Categorical] "Process State (Heating/Cooling/Mixing)"
→ "Custom parameter..."

User types "active" in any context:
→ [Boolean] "Active (Active/Inactive)"
→ [Boolean] "Enabled (Yes/No)"
→ "Custom parameter..."

User types "flow" in Material context:
→ [Numeric] "Flow Rate (kg/s, liters/min, m³/hr)"
→ [Ordinal] "Flow Level (High/Medium/Low)"
→ [Boolean] "Flow Direction (Bidirectional: True/False)"
→ "Custom parameter..."
```

### Components Modified

- **Flow Element Component**: Remove `substance_unit` and `substance_amount` fields, consolidate to parameters table
- **Element Details UI**: Simplified substance classification, enhanced parameter input with autocomplete
- **Serialization/Deserialization**: Update data model to handle unified parameter structure
- **Substance Type Dropdown**: Remove "Sub Type" field, keep only fundamental Energy/Material/Message classification

### Architecture Decisions

**Unified Parameter Paradigm**: All quantified properties use consistent Name/Value/Unit structure instead of mixing typed fields with parameter tables. This eliminates cognitive overhead and provides flexible extensibility.

**Context-Driven Intelligence**: Unit suggestions driven by substance type context rather than generic lists. Energy flows suggest energy-related units (watts, joules), Material flows suggest mass/volume units (kg, liters), Message flows suggest information units (bits, messages/sec).

**Progressive Disclosure**: Start with simple substance type classification, then allow detailed parameterization. Users aren't forced into premature categorization but can add precision as needed.

## Usage Examples

### Enhanced User Experience Flow (with Categorical Variables)
1. **Create Flow Element**: User names flow "Heat Transfer" 
2. **Select Substance Type**: Choose "Energy" from dropdown
3. **Add Numeric Parameter**: Click "+" to add parameter
   - Type "temp" → suggests "[Numeric] Temperature (°C, °F, K)"  
   - Select "°C" → numeric input field with validation
4. **Add Ordinal Parameter**:
   - Type "eff" → suggests "[Ordinal] Efficiency (Poor/Fair/Good/Excellent)"
   - Select suggestion → dropdown with ordinal levels
5. **Add Boolean Parameter**:
   - Type "active" → suggests "[Boolean] Active (Active/Inactive)"
   - Select suggestion → toggle switch interface
6. **Add Categorical Parameter**:
   - Type "state" → suggests "[Categorical] Process State (Heating/Cooling/Stable)"
   - Select suggestion → dropdown with category options

### Parameter Suggestion Categories by Type

#### Numeric Parameters (with units)
```
Energy: watts, kilowatts, BTU/hr, joules/sec, calories/sec
Mass: kg, grams, tons, pounds, kg/s (flow rates)
Volume: liters, m³, gallons, liters/min (flow rates)  
Information: bits, bytes, messages/sec, packets/sec
Temperature: °C, °F, K
Pressure: Pa, psi, bar, atm
```

#### Ordinal Parameters (hierarchical levels)
```
Performance: Excellent > Good > Fair > Poor
Priority: Critical > Important > Optional > Deferred  
Quality: Premium > Standard > Basic > Minimal
Efficiency: High > Medium > Low
Risk: Severe > Moderate > Minor > Negligible
```

#### Categorical Parameters (discrete options)
```
Material States: Solid, Liquid, Gas, Plasma
Transport Protocols: HTTP, TCP, UDP, WebSocket
Process Types: Batch, Continuous, Semi-Batch
Flow Directions: Unidirectional, Bidirectional, Multi-directional
```

#### Boolean Parameters (binary states)
```
System States: Active/Inactive, Enabled/Disabled, On/Off
Flow Properties: Bidirectional (True/False), Reversible (Yes/No)
Quality Flags: Encrypted (True/False), Validated (Yes/No)
```

## Testing Strategy

### Unit Tests
- **Parameter type engine**: Verify correct type suggestions (Numeric/Ordinal/Categorical/Boolean) for each substance type
- **Unit suggestion engine**: Verify correct unit suggestions for numeric parameters
- **Categorical validation**: Test ordinal ordering and categorical option sets
- **Parameter validation**: Test type-value compatibility checking across all parameter types
- **Data model migration**: Ensure old format files load correctly with new unified parameter structure  
- **Autocomplete functionality**: Test fuzzy matching and suggestion ranking across all parameter types

### Integration Tests  
- **UI workflow**: Complete flow from substance type selection to parameter entry
- **Serialization round-trip**: Save/load cycles preserve parameter data correctly
- **Cross-element consistency**: Parameters work consistently across different element types

### User Experience Tests
- **Cognitive load reduction**: Compare task completion time vs. old multi-field approach
- **Unit discovery**: Test users finding appropriate units without documentation
- **Error scenarios**: Invalid units, missing values, edge cases

## Future Improvements

### Core System Enhancements
- **Domain-specific parameter libraries**: Biochemistry, economics, engineering-specific parameter collections
- **Unit conversion**: Automatic conversion between compatible units (kg ↔ pounds)
- **Parameter relationships**: Support for calculated parameters (e.g. rate = amount/time)
- **Parameter templates**: Common parameter sets for typical flow types
- **Import from external standards**: Integration with scientific unit databases and categorical taxonomies
- **Dynamic simulation support**: Time-varying parameter values for future dynamics

### Advanced Categorical Features  
- **Custom ordinal scales**: User-defined hierarchical orderings for domain-specific needs
- **Hierarchical categories**: Multi-level categorical structures (Material > Metal > Steel > Carbon Steel)
- **Parameter dependencies**: Conditional parameter suggestions based on other parameter values
- **Validation rules**: Cross-parameter validation (e.g., if Material State = "Gas" then Pressure > 0)
- **Statistical analysis**: Ordinal correlations and categorical distributions in system analysis

## Related Documentation

- **BERT 1.0 Parameter History**: Previous implementation in archived phase1-docs-contributing
- **Flow Element Components**: `src/bevy_app/components/system_elements.rs`
- **Element Details UI**: `src/leptos_app/details.rs`
- **Data Model Serialization**: `src/bevy_app/data_model/save.rs` and `load.rs`
- **Systems Science Context**: Flow modeling principles from System Language framework
- **User Experience Research**: Cognitive load reduction in system modeling interfaces

---

## Summary: Unified Smart Parameters with Categorical Variables

This enhanced Smart Parameters system integrates **Cliff's categorical variable framework** with the original unit intelligence approach, creating a comprehensive parameter system that handles:

1. **Quantitative Measurements** (original focus): Flow rates, temperatures, pressures with intelligent unit suggestions
2. **Qualitative Characteristics** (Cliff's enhancement): Performance levels, material states, operational flags

**Key Benefits**:
- **Unified Interface**: Single parameter table handles all data types intelligently
- **Context Awareness**: Parameter suggestions based on substance type (Energy/Material/Message)  
- **Cognitive Simplicity**: Eliminates redundant fields while expanding capability
- **Systems Completeness**: Supports both numerical analysis AND categorical modeling for comprehensive system characterization

**Perfect Synergy**: The thesis network analysis workflow benefits from both numeric flow measurements AND categorical system characteristics, making this enhanced design ideal for economic systems modeling where both quantitative flows and qualitative states matter.

---

_This documentation was generated for Smart Parameters on 2025-08-22, enhanced with categorical variables on 2025-09-04 following Cliff's recommendations._
