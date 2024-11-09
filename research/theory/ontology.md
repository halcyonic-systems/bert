# Systems Ontology

GST founder Ludwig von Bertalanffy identified “systems ontology” as the field dedicated to determining “what is meant by ‘system’ and how systems are realized at the various levels of the world of observation.” In Systems Science: Theory, Analysis, Modeling, and Design (2022) George Mobus outlines a formal systems ontology — a set of terms naming what exists in the domain of systems.

The ontology is derived from the [12 principles of systems science](https://github.com/halcyonic-systems/bert/blob/main/research/theory/principles.md) and provides a set of base terms describing what fundamental components exist in all systems and gives us a basic language for discussing all relevant aspects of systemness.


# Exploring the Ontology

[Download](https://github.com/halcyonic-systems/bert/blob/main/research/theory/systemontology.rdf)

## Exploration Tools
[Protege](https://protege.stanford.edu/)

[Web Protege](https://webprotege.stanford.edu/)

[Protege tutorial](https://drive.google.com/file/d/1A3Y8T6nIfXQ_UQOpCAr_HFSCwpTqELeP/view)

[WebVOWL](https://service.tib.eu/webvowl/)



# Systems Ontology Summary Report

## 1. Introduction and Purpose

This ontology formalizes key concepts in systems science. The primary purposes are to:

1. Provide a common vocabulary for discussing systems concepts
2. Represent the structural and functional aspects of systems
3. Capture the relationships between systems, their components, and their environment
4. Serve as a foundation for further theoretical development in systems science
5. Model the emergence of systems from fundamental elements
6. Support analysis of complex systems through clear hierarchical organization

## 2. Key Components

### 2.1 Classes

The primary classes in the ontology represent the core concepts in systems theory:

| Class | Description |
| --- | --- |
| FundamentalElement | The basic building blocks from which all systems emerge |
| Energy | A fundamental element representing the capacity for work |
| Information | A fundamental element representing meaningful patterns |
| Knowledge | A fundamental element representing processed, applicable information |
| Matter | A fundamental element representing physical substance |
| System | The central concept, representing any coherent whole emerging from fundamental elements |
| Subsystem | A system that is part of a larger system |
| Component | The basic elements that make up a system |
| AtomicWorkProcess | A fundamental, indivisible unit of work within a system |
| Environment | The context in which a system exists |
| Input | Resources or information entering the system |
| Output | Resources or information leaving the system |
| Boundary | The interface between a system and its environment |
| ComplexAdaptiveSystem | A system that can adapt to its environment |
| Flow | Movement of matter, energy, or information through a system |
| Source | An entity providing inputs to a system |
| Sink | An entity receiving outputs from a system |

### 2.2 Object Properties

Object properties define relationships between classes:

| Property | Domain | Range | Description |
| --- | --- | --- | --- |
| emergesFrom | System, Component, Environment, Boundary | FundamentalElement | Represents emergence from fundamental elements |
| hasSubsystem | System | Subsystem | Relates a system to its subsystems |
| isPartOf | Subsystem, AtomicWorkProcess | System, Subsystem | Inverse of hasSubsystem |
| hasComponent | System | Component | Relates a system to its components |
| existsInEnvironment | System | Environment | Relates a system to its environment |
| hasInput | System, AtomicWorkProcess | Input | Relates a system or process to its inputs |
| hasOutput | System, AtomicWorkProcess | Output | Relates a system or process to its outputs |
| interactsWith | Component, FundamentalElement | Component, FundamentalElement | Represents interactions |
| hasBoundary | System | Boundary | Relates a system to its boundary |
| consistsOf | Input, Output | FundamentalElement | Relates flows to fundamental elements |
| transformsFundamentalElement | AtomicWorkProcess | FundamentalElement | Relates processes to elements they transform |
| consumesEnergy | AtomicWorkProcess | Energy | Relates processes to energy consumption |

### 2.3 Data Properties

Data properties assign attributes to classes:

| Property | Domain | Range | Description |
| --- | --- | --- | --- |
| hasComplexity | System | Integer | Represents the complexity level of a system |
| hasHierarchyLevel | System | Integer | Indicates the hierarchical level of a system |
| HasBoundaryPermeability | System | Float | Represents how permeable a system's boundary is |

## 3. Key Relationships and Concepts

The ontology captures several important systems concepts:

1. **Fundamental Elements**: All systems and their components emerge from interactions of Energy, Information, Knowledge, and Matter.

2. **Emergence**: Systems and their properties emerge from fundamental element interactions, represented throughout the ontology.

3. **Hierarchy**: Systems contain subsystems, which are themselves systems, creating nested levels of organization.

4. **System-Environment Interaction**: Systems exist within an environment and interact through inputs and outputs.

5. **Complex Adaptive Systems**: Systems that can adapt to their environment through dynamic reconfiguration.

6. **System Boundaries**: Emergent interfaces that regulate flows between system and environment.

7. **Component Interactions**: Internal relationships between system components.

8. **Atomic Work Processes**: Ten fundamental types of work processes that transform system elements:
   - AmplifyingProcess
   - BufferingProcess
   - CombiningProcess
   - CopyingProcess
   - ImpedingProcess
   - InvertingProcess
   - ModulatingProcess
   - PropellingProcess
   - SensingProcess
   - SplittingProcess

9. **Flows and Transformations**: Movement and transformation of fundamental elements through systems.

## 4. Potential Applications

This ontology serves multiple purposes:

1. Teaching Tool
   - Introducing systems concepts
   - Demonstrating system relationships
   - Explaining emergence and complexity

2. Analysis Framework
   - Comparing different types of systems
   - Analyzing system structure
   - Understanding system behavior

3. Domain Integration
   - Supporting interdisciplinary communication
   - Developing domain-specific extensions
   - Bridging different fields of study

4. System Modeling
   - Representing complex systems
   - Analyzing emergent properties
   - Supporting system design

5. Knowledge Management
   - Organizing systems knowledge
   - Supporting system documentation
   - Facilitating knowledge sharing

## 5. Future Development

Areas for potential expansion include:

1. System Dynamics
   - More detailed process representations
   - Enhanced feedback loop modeling
   - Temporal aspects of system behavior

2. Fundamental Elements
   - Deeper exploration of element interactions
   - More detailed transformation patterns
   - Element flow patterns

3. Complexity Analysis
   - Metrics for system complexity
   - Emergence patterns
   - Hierarchy analysis methods

4. Integration Capabilities
   - Domain-specific extensions
   - Tool integration support
   - Analysis method integration

5. Practical Applications
   - Design guidance
   - Analysis templates
   - Implementation patterns

By continuing development in these areas, the ontology will provide increasingly powerful support for systems science and engineering.

# Systems Ontology Glossary

## Fundamental Elements

### Matter
Definition: A fundamental substance from which systems and components can emerge.
Used in: Material inputs/outputs, physical system structures.

### Energy
Definition: A fundamental substance that enables work and transformation in systems.
Used in: Powering processes, enabling transformations, system operations.

### Information
Definition: A fundamental substance that represents patterns and relationships.
Used in: System signaling, control, communication.

### Knowledge
Definition: A fundamental substance representing structured understanding and capabilities.
Used in: System adaptation, learning, complex behaviors.

## Core System Concepts

### System
Definition: A coherent whole that emerges from interactions of Information, Knowledge, Matter, and Energy, composed of interrelated and interdependent parts.
Example: An ecosystem emerging from interactions between organisms and environment.

### AtomicWorkProcess
Definition: A fundamental, indivisible unit of work that transforms inputs into outputs by manipulating fundamental elements.
Types Include: Amplifying, Buffering, Combining, Copying, Impeding, Inverting, Modulating, Propelling, Sensing, Splitting.

### Environment
Definition: The context or surroundings in which a system exists, emerging from complex interactions of fundamental elements.
Example: Market conditions, regulatory framework, and technological landscape for a business system.

### Boundary
Definition: The interface between system and environment that emerges from specific configurations of fundamental elements.
Example: Cell membrane regulating molecular flows.

## Flow Concepts

### Input
Definition: Matter, Energy, Information, or Knowledge entering a system from its environment.
Types: MaterialInput, EnergyInput, MessageInput

### Output
Definition: Transformed fundamental elements leaving a system and entering its environment.
Types: MaterialOutput, EnergyOutput, MessageOutput

### Source
Definition: An entity in the environment that provides inputs to a system.
Example: Power plant providing electricity.

### Sink
Definition: An entity in the environment that receives system outputs.
Example: Waste processing facility.

## Atomic Work Processes

### AmplifyingProcess
Definition: Increases the magnitude of a signal or flow using external energy.
Example: Audio amplifier increasing sound signal strength.

### BufferingProcess
Definition: Temporarily stores a substance to regulate its flow.
Example: Water reservoir managing water supply.

### CombiningProcess
Definition: Merges multiple inputs into a single output.
Example: Chemical reaction combining reactants.

### CopyingProcess
Definition: Replicates an input to produce identical outputs.
Example: DNA replication.

### ImpedingProcess
Definition: Slows or resists the flow of a substance.
Example: Electrical resistor.

### InvertingProcess
Definition: Reverses or negates input characteristics.
Example: Logic gate inverter.

### ModulatingProcess
Definition: Adjusts signal characteristics based on control input.
Example: Radio wave modulation.

### PropellingProcess
Definition: Accelerates or facilitates substance flow.
Example: Pump moving fluid.

### SensingProcess
Definition: Detects and responds to environmental changes.
Example: Thermostat measuring temperature.

### SplittingProcess
Definition: Divides single input into multiple outputs.
Example: Power distribution network.

## Complex Systems

### ComplexAdaptiveSystem
Definition: A system that can reconfigure its internal relationships of fundamental elements in response to environmental changes.
Example: Immune system adapting to new pathogens.

### Component
Definition: A fundamental part of a system that emerges from configurations of fundamental elements.
Example: Organs within a biological system.

