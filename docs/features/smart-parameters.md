# Feature: Smart Parameters

## Overview

**Feature Name**: Smart Parameters  
**Branch**: feature/smart-parameters  
**Status**: In Progress  
**Contributors**: Shingai Thornton, Claude  
**Date**: 2025-08-22

## Description

Replace the clunky multi-field parameter approach from BERT 1.0 with a unified, intelligent parameter system. Eliminates redundant "Substance Unit" and "Substance Amount" fields by consolidating all quantified properties into a smart Name/Value/Unit table that suggests appropriate units based on substance type context.

## Implemented Functionality

- Unified parameter table for all quantified flow properties
- Smart unit suggestions based on substance type (Energy, Material, Message)
- Context-aware unit recommendations (e.g. Energy → watts, BTU/hr, joules)  
- Custom unit support for domain-specific needs
- Simplified substance type classification (remove Sub Type cognitive overhead)
- Clean separation between qualitative properties and quantitative parameters

## Technical Implementation

### Components Added

- **Unit Suggestion Engine**: Smart unit recommendations based on substance type context
- **Unit Categories Database**: Organized collections of common units by domain (Mass, Energy, Information, etc.)
- **Parameter Validation System**: Ensures unit-value compatibility and reasonable ranges

### Unit Suggestion Engine Implementation Options

#### Core Data Structure
```rust
// Unit categories organized by substance type
struct UnitDatabase {
    energy_units: Vec<UnitSuggestion>,
    material_units: Vec<UnitSuggestion>, 
    message_units: Vec<UnitSuggestion>,
}

struct UnitSuggestion {
    display_name: String,     // "Temperature"
    unit_options: Vec<String>, // ["°C", "°F", "K"]
    common_names: Vec<String>, // ["temp", "temperature", "heat"]
    category: UnitCategory,    // Temperature, Rate, Amount, etc.
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

#### User Interaction Examples
```
User types "pow" in Energy context:
→ "Power (watts, kilowatts, horsepower)"
→ "Power Density (watts/m²)" 
→ "Custom unit..."

User types "flow" in Material context:
→ "Flow Rate (kg/s, liters/min, m³/hr)"
→ "Custom unit..."

User types "temp" in any context:
→ "Temperature (°C, °F, K)"
→ "Custom unit..."
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

### User Experience Flow
1. **Create Flow Element**: User names flow "Heat Transfer" 
2. **Select Substance Type**: Choose "Energy" from dropdown
3. **Add Parameters**: Click "+" to add parameter
   - Start typing "rat" → autocomplete suggests "Rate (watts, BTU/hr, cal/s)"
   - Select "watts" → parameter created with unit validation
4. **Add Temperature Parameter**: 
   - Type "temp" → suggests "Temperature (°C, °F, K)"
   - Select "°C" → ready for value input

### Unit Suggestion Categories
```
Energy: watts, kilowatts, BTU/hr, joules/sec, calories/sec
Mass: kg, grams, tons, pounds, kg/s (flow rates)
Volume: liters, m³, gallons, liters/min (flow rates)  
Information: bits, bytes, messages/sec, packets/sec
Temperature: °C, °F, K
Pressure: Pa, psi, bar, atm
```

## Testing Strategy

### Unit Tests
- **Unit suggestion engine**: Verify correct suggestions for each substance type
- **Parameter validation**: Test unit-value compatibility checking  
- **Data model migration**: Ensure old format files load correctly with new parameter structure
- **Autocomplete functionality**: Test fuzzy matching and suggestion ranking

### Integration Tests  
- **UI workflow**: Complete flow from substance type selection to parameter entry
- **Serialization round-trip**: Save/load cycles preserve parameter data correctly
- **Cross-element consistency**: Parameters work consistently across different element types

### User Experience Tests
- **Cognitive load reduction**: Compare task completion time vs. old multi-field approach
- **Unit discovery**: Test users finding appropriate units without documentation
- **Error scenarios**: Invalid units, missing values, edge cases

## Future Improvements

- **Domain-specific unit libraries**: Biochemistry, economics, engineering-specific unit collections
- **Unit conversion**: Automatic conversion between compatible units (kg ↔ pounds)
- **Parameter relationships**: Support for calculated parameters (e.g. rate = amount/time)
- **Parameter templates**: Common parameter sets for typical flow types
- **Import from external standards**: Integration with scientific unit databases
- **Dynamic simulation support**: Time-varying parameter values for future dynamics

## Related Documentation

- **BERT 1.0 Parameter History**: Previous implementation in archived phase1-docs-contributing
- **Flow Element Components**: `src/bevy_app/components/system_elements.rs`
- **Element Details UI**: `src/leptos_app/details.rs`
- **Data Model Serialization**: `src/bevy_app/data_model/save.rs` and `load.rs`
- **Systems Science Context**: Flow modeling principles from System Language framework
- **User Experience Research**: Cognitive load reduction in system modeling interfaces

---

_This documentation was automatically generated for the Smart Parameters feature on 2025-08-22._
