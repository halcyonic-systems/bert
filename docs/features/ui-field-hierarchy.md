# BERT UI Field Hierarchy - Strategic Design Notes

## Stream of Consciousness Analysis (2025-08-01)

### Current State
- **New simplified panel**: Name, Description, Complexity, Equivalence
- **Old comprehensive panel**: Name, Description, Complexity, Equivalence, Time Unit, History, Transformation
- **Issue**: Arbitrary removal - kept Equivalence but removed Time Unit (more fundamental)

### Field Priority Ranking for New Users

1. **Time Unit** (HIGHEST PRIORITY)
   - Purpose: Orient users to the temporal scale of their system
   - Example: Seconds for chemical reactions, Years for ecosystems
   - Essential for establishing modeling context

2. **Complexity** (HIGH PRIORITY)  
   - Purpose: Categorize system behavior type
   - Options: Simple, Adaptive, Evolveable
   - Core to understanding system dynamics

3. **Equivalence Class** (MEDIUM PRIORITY)
   - Purpose: Group similar systems conceptually
   - Challenge: Requires good explanations and examples
   - Important but more abstract concept

4. **Transformation Rules** (FUTURE FEATURE)
   - Purpose: Define system state transitions
   - Dependency: Requires temporal data collection first
   - Cannot be meaningfully defined without history

5. **History** (FUTURE FEATURE)
   - Purpose: Track system state over time
   - Dependency: Only relevant after establishing temporal framework
   - Becomes useful once state changes are captured

### Design Principle: Progressive Disclosure
Rather than overwhelming users with all fields, introduce concepts progressively:
- Start with essential orientation (Time Unit, Complexity)
- Build conceptual understanding (Equivalence with help)
- Enable advanced features once data exists (Transformation, History)

### Next Steps
- Consider reintroducing Time Unit as primary field
- Ensure robust context help for each field
- Plan phased rollout of advanced temporal features

---

## Flow Details Panel Analysis (2025-08-01)

### Current Issues

1. **Field Ordering Problems**
   - Current: Interaction Usability → Interaction Type → Substance Type
   - Better: Interaction Type → Substance Type → Interaction Usability
   - Logic: Define WHAT before HOW IT'S USED

2. **Label Clarity**
   - "Interaction Usability" - unclear meaning
   - "Substance Sub Type" - too granular for initial users
   - "Substance Amount" - ambiguous concept
   - All need context help (?) with explanations

3. **Empty Fields Overwhelm**
   - Substance Sub Type (blank)
   - Substance Unit (blank)
   - Creates cognitive overload for new users

4. **Parameters Section Confusion**
   - Current order: Name → Value → Unit
   - Better order: Name → Unit → Value
   - "Value" implies measurement result (dynamic) not definition (static)

### Critical Design Junction: Static vs Dynamic

**The Challenge**: BERT is transitioning from static structural tool to dynamic modeling/simulation kit

**Design Principle**: Clear separation of concerns
- **Static Analysis Mode**: Structure, relationships, definitions
- **Dynamic Modeling Mode**: Measurements, simulations, temporal data

**Risk**: Overwhelming users with dynamic concepts before they understand static structure

### Proposed Approach

1. **Phase 1 - Static Structure** (Current)
   - Focus on system definition
   - Hide measurement-oriented fields
   - Emphasize relationships and types

2. **Phase 2 - Dynamic Modeling** (Future)
   - Introduce temporal concepts
   - Add measurement capabilities
   - Enable simulation features

### Flow Panel Redesign Considerations
- Reorder fields logically (Type → Substance → Usage)
- Add context help for all non-obvious terms
- Consider hiding advanced fields initially
- Clarify static vs dynamic field purposes
- Possibly rename "Value" to "Default" or "Initial Value"