# Glossary

> **Systems Theory Foundations**: This glossary provides formal definitions from systems theory alongside practical guidance for BERT modeling.

## Fundamental Concepts

### System

**Formal Definition**: A system is a "thing, a whole entity." More formally, "a system S is a 7-tuple: S = (C, N, G, B, T, H, Δt)" where each element represents components, network, goals, boundary, time, history, and time step.

**In BERT**: The main system boundary (large circle) that you create as your starting point for analysis.

{% hint style="info" %}
**BERT Example**: When modeling a restaurant, the main system circle represents the entire restaurant operation within its physical and operational boundaries.
{% endhint %}

#### System Types

**Adaptable or Evolvable Systems**: 
- **Complex Adaptive Systems (CAS)**: "Are able to respond to changes in their nominal environmental conditions"
- **Complex Evolvable Systems (CAES)**: Can "undergo modifications that permanently change their structures and behaviors to meet the demands of longer-term environmental changes"

**In BERT**: Use subsystem decomposition to model how different parts of your system adapt to changes.

### Component

**Definition**: "Every object element within the system boundary, including the interfaces, channels, stocks, sensors, and regulators, are components of the system."

**In BERT**: All elements you place inside your system boundary - subsystems, interfaces, and internal flows.

#### Component Complexity Types

| Type | Definition | BERT Example |
|------|------------|--------------|
| **Simple** | Limited number of elements and hierarchy levels | A single coffee machine subsystem |
| **Complex** | "Contains many heterogeneous parts and many levels of organization" | Kitchen operations with multiple cooking stations |
| **Complex Adaptive (CAS)** | "Ability to change internally to compensate for environmental changes" | Restaurant adapting menu based on supply availability |
| **Complex Adaptive and Evolvable (CAES)** | Can make "permanent restructuring in response to long-term environmental alterations" | Restaurant expanding from fast-food to full-service model |

#### Component Properties

**Atomic Components**: "Leaf nodes in the deconstruction tree" - components not further decomposed because their internal structures are given.

**In BERT**: When you stop decomposing a subsystem because its internal operation is well-understood or outside your analysis scope.

**Member Autonomy (0-1)**: The degree of autonomy a component has within a system.

**Multiset**: A component containing "multiple instances of the same kind of element."

**In BERT**: Multiple identical subsystems (like multiple cashier stations in a restaurant).

### Boundary

**Definition**: "Boundedness constructs effective boundaries to systems."

**In BERT**: The visual boundary of your main system circle and subsystem circles.

#### Boundary Properties

**Porosity (0-1)**: The degree to which a boundary allows matter, energy, or messages to pass through it.

**In BERT**: Represented by the interfaces you place on system boundaries - more interfaces indicate higher porosity.

**Perceptive Fuzziness (0-1)**: The degree of ambiguity in identifying the boundary. Some boundaries are "not easily identifiable enclosures."

{% hint style="warning" %}
**Modeling Tip**: If you're struggling to define where your system boundary should be, you may be dealing with perceptive fuzziness. Start with a clear core and add boundary elements iteratively.
{% endhint %}

## System Elements

### Flow

**Definition**: A movement of material, energy, or message.

**In BERT**: The curved arrows you create between elements to represent transfers.

#### Flow Types by Interaction

| Type | Definition | BERT Example |
|------|------------|--------------|
| **Flow** | Movement of substance from source to sink | Money flowing from customer to restaurant |
| **Force** | Interaction involving push or pull | Regulatory pressure on restaurant operations |

#### Flow Types by Substance

| Substance Type | Definition | BERT Examples |
|----------------|------------|---------------|
| **Material** | Tangible substance | Food ingredients, dishes, waste |
| **Energy** | Flow that does work or can be converted to work | Electricity, heat, human labor |
| **Message** | Flow of information or influence | Orders, feedback, regulations |

#### Flow Attributes

**In BERT**: Edit these in the Properties Panel when you select a flow:

- **Substance Sub-Type**: Specific kind (e.g., "electricity" vs "gravitational potential")
- **Substance Unit**: Unit of measure (e.g., "TONS", "kWh", "$/hour")  
- **Substance Amount**: Quantity (e.g., "10/hr")
- **Parameters**: Rate, timing, variance attributes

#### Flow Outputs

**Product**: Output that "gives the system its purpose" - the primary valuable output.

**In BERT**: The main flows exiting through your primary output interfaces.

**Waste**: Unusable outputs of a process.

**In BERT**: Secondary flows that represent byproducts or disposal needs.

{% hint style="info" %}
**Best Practice**: Always model both product and waste flows for complete system representation.
{% endhint %}

### Interface

**Definition**: The boundary of a component that receives or sends flows.

**In BERT**: Small circles placed on system/subsystem boundaries where flows connect.

**Protocol**: Set of rules governing substance flow into or out of a component.

**In BERT**: Document protocols in the interface properties or description fields.

{% hint style="tip" %}
**BERT Usage**: Double-click an interface to edit its protocol rules and connection specifications.
{% endhint %}

### Source/Sink (External Entities)

**Definition**: Sources are the origin of a flow while sinks are the destination.

**In BERT**: Square elements placed outside your system boundary representing external entities.

#### Common Source/Sink Types

| Type | Role | BERT Example |
|------|------|--------------|
| **Source** | Provides inputs to system | Suppliers, customers placing orders |
| **Sink** | Receives outputs from system | Customers receiving food, waste disposal |
| **Source & Sink** | Both provides and receives | Customers (provide money, receive food) |

## Advanced Concepts

### Equivalence

**Definition**: An equivalence class determined by common criteria among multiple components.

**In BERT**: Group similar elements to simplify complex models.

{% hint style="info" %}
**Practical Use**: Instead of modeling 5 identical cash registers separately, create one "Cash Register" subsystem to represent the equivalence class.
{% endhint %}

### Transformation

**Definition**: "The dynamic behavior of the system of interest" - how inputs are transformed into outputs through differential equations or computer programs.

**In BERT**: The internal processes within subsystems that convert inputs to outputs.

**Implementation**: Document transformation rules in subsystem descriptions or link to external process documentation.

### Time Elements

**Time Unit**: The time step over which discrete simulation would operate.

**History**: Historical records of system behavior, "captured in accounting records" or linked documentation.

**In BERT**: Reference historical data in system properties or maintain links to time-series data files.

## Model Concepts

### Model

**Definition**: A representation of a system that captures essential features for understanding or prediction. Multiple models of a single system can exist at different abstraction levels.

**In BERT**: Your complete BERT diagram is a model. Use system decomposition to create models at different detail levels.

### Disruption

**Definition**: A change in the environment that impacts system functions.

**In BERT**: Model disruptions as:
- Changes in external entity behavior
- New flow requirements  
- Modified boundary conditions
- System adaptations (new subsystems/interfaces)

---

## Quick Reference for BERT Users

### Creating Elements Based on Definitions

1. **Start with System**: Create main system boundary
2. **Identify Boundaries**: Where does your system interact with the environment?  
3. **Add Interfaces**: Place connection points on boundaries
4. **Connect External Entities**: Add sources and sinks outside boundary
5. **Model Flows**: Connect entities through interfaces with appropriate substance types
6. **Decompose Components**: Break complex subsystems into simpler parts
7. **Document Transformations**: Describe how each component processes its inputs

### Using Equivalence Classes

- Group similar components to reduce model complexity
- Use representative subsystems for identical processes
- Apply consistent naming conventions for equivalent elements

### Managing Complexity

- **Simple Systems**: Few elements, clear hierarchy
- **Complex Systems**: Use hiding (`H` key) and decomposition strategically  
- **Adaptive Systems**: Model feedback loops and control mechanisms
- **Evolvable Systems**: Plan for structural changes over time

{% hint style="success" %}
**Remember**: Every BERT model is a formal system representation. Understanding these theoretical foundations will make your models more rigorous and meaningful.
{% endhint %}