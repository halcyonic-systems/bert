# Mobus Systems Science Reference for BERT

Quick reference for Mobus's Deep Systems Analysis (DSA) framework as applied to BERT development.

## The 8-Tuple System Definition

Every system in BERT follows this formal structure:

| Symbol | Name | BERT Mapping |
|--------|------|--------------|
| **C** | Components | Subsystems (circles) |
| **N** | Network | Internal flows between subsystems |
| **E** | Environment | External sources/sinks |
| **G** | External interactions | Flows crossing boundary |
| **B** | Boundary | System boundary ring |
| **T** | Transformation | Protocols on interfaces |
| **H** | Hierarchy | Nested systems (zoom in) |
| **Δt** | Time scale | Time unit setting |

## Core DSA Principles

### 1. Purpose-Driven Analysis
- Start with outputs (what the system produces)
- Work backward to required inputs
- System exists to transform inputs → outputs

### 2. Recursive Decomposition
- Systems contain subsystems
- Subsystems are themselves systems
- Continue until components are "atomic enough" for your purpose

### 3. Interface-Centric Design
- Interfaces are boundary components that regulate flow
- Every flow crosses the boundary through an interface
- Protocols define HOW crossing occurs

### 4. Three Substance Types
| Type | Examples | Flow Characteristics |
|------|----------|---------------------|
| **Material** | Glucose, steel, documents | Conserved, transformable |
| **Energy** | ATP, electricity, heat | Conserved, degradable |
| **Message** | Signals, data, commands | Copyable, not conserved |

### 5. Flow Usability Types
| Usability | Meaning |
|-----------|---------|
| **Resource** | Input needed by system |
| **Product** | Intended output |
| **Waste** | Byproduct to dispose |

## Agent Archetype Model

For agent-based simulation (future BERT feature):

```
Agent = Computational Engine + Decision Model + Experiential Memory
```

Key archetypes (from system-archetypes.md):
- **Controller**: Regulates processes, feedback loops
- **Generator**: Produces outputs from inputs
- **Consumer**: Receives and processes inputs
- **Coordinator**: Manages multiple subsystems
- **Filter**: Selects/transforms flow content

## Key Quotes for Reference

> "A system may be defined as a set of elements standing in interrelations."
> — Bertalanffy

> "Deep Systems Analysis refers to a methodology for helping scientists deeply understand complex systems and then constructing models based on this deep understanding."
> — Mobus

> "BERT is a prototype of the first application designed specifically to help systems scientists perform DSA."
> — Mobus

## BERT Implementation Notes

### What BERT Enforces
- Subsystems MUST attach to existing interfaces (no floating components)
- Outputs should be defined before inputs (purpose-driven)
- Every flow has a source and sink (conservation)

### What BERT Simplifies
- GUI abstracts the 8-tuple (users build visually)
- Protocols are text descriptions (not executable code... yet)
- Hierarchy is navigable via zoom (not explicit H parameter)

## Further Reading

- `gitbook/for-researchers/system-archetypes.md` - Archetype theory (agent, governance, economy)
- `gitbook/for-researchers/deep-systems-analysis.md` - DSA methodology
- Mobus, George E. *Systems Science: Theory, Analysis, Modeling, and Design* (2022)
