# System Archetypes Reference

## Abstract

This reference document synthesizes the relationships between three major model archetypes in George Mobus's "Systems Science: Theory, Analysis, Modeling, and Design" (2022): agents, governance systems, and economic systems. It clarifies their definitions, relationships, and scale-dependent nature, providing both theoretical foundations and practical applications for systems scientists and practitioners.

## Table of Contents
1. [Introduction](#1-introduction)
2. [Theoretical Framework](#2-theoretical-framework)
3. [The Three Model Archetypes](#3-the-three-model-archetypes)
4. [Scale-Dependent Relationships](#4-scale-dependent-relationships)
5. [Practical Applications](#5-practical-applications)
6. [Quick Reference Guide](#6-quick-reference-guide)
7. [Glossary](#7-glossary)

---

## 1. Introduction

### 1.1 Overview of Model Archetypes

Mobus identifies three fundamental "sub-archetypes" that constitute the major subsystems of any complex adaptive and evolvable system (CAS/CAES):

> "In Chap. 10, we will introduce several archetype models… These will be the 'master' archetypes of complex adaptive and evolvable systems (CAS/CAES) and three sub-archetypes that constitute the major subsystems of any CAS/CAES. These are: **agent (with agency), economy, and governance**…" (Ch. 9, p. 394)

### 1.2 Key Principle: Recursive, Scale-Dependent Framework

The relationship between these archetypes is fundamentally recursive and scale-dependent:

> "Systems (and systemness) are naturally recursive structures (and concepts)." (Ch. 9, p. 394)

This means the same entity can function as:
- An **agent** when viewed from a higher hierarchical level
- A **system composed of agents** when analyzed at its own level

---

## 2. Theoretical Framework

### 2.1 Hierarchical System Structure

From the mathematical framework:

> "Any component, c_{i,j} ∈ C_{i}, is considered a subsystem by Eq. 4.3, meaning it is also a system and therefore a member of the System table." (Ch. 4)

This establishes that components simultaneously exist as:
- Subsystems of larger systems
- Systems in their own right
- Potentially agents from certain perspectives

### 2.2 Fuzzy Boundaries and Membership Functions

Mobus acknowledges the challenge of clearly delineating these subsystems:

> "The key here is to look at individual components within any subsystem and determine their membership function with respect to each subsystem. Many components might have membership functions that simply return unity (1) meaning they are always present with probability 1. But many components can effectively multiplex or serve roles in multiple subsystems at different times and with different probabilities." (Ch. 9, p. 404)

---

## 3. The Three Model Archetypes

### 3.1 Agents: The Fundamental Decision-Making Units

#### Definition
Agents are entities capable of:
- Receiving and processing information
- Making decisions based on that information
- Taking actions that affect their environment

#### Key Characteristics
From Chapter 11:
> "the meta-system called a 'decision agent.' We will see that this is a **special case of an adaptive (and evolvable) system** that relies on having a storehouse of experientially based, implicit knowledge."

### 3.2 Governance Systems: Dual Nature

#### As Agents (Macro Perspective)
Mobus explicitly describes governance functioning as an agent:

> "Figure 7.17 also shows the flows of messages from entities in the environment to the governance subsystem of the HSS. **The latter is considered as the agent that makes the decisions about how the HSS should behave** relative to the resources and waste dumps." (Ch. 7, p. 368)

#### As Systems of Agents (Internal Perspective)
Governance systems require human agents:

> "the economy transfers some of its labor force to the governing process—**it needs people to be the decision agents at all levels**." (Ch. 9)

#### Fractal Structure
> "Each subsystem has its own internal governance sub-subsystem that is for its own self-regulation... each process is seen to have its own internal governance subsystem reflecting the **fractal-like structure** of the HCGS" (Ch. 9, Fig. 9.10 caption)

### 3.3 Economic Systems: Emergent Networks

#### Primary Characteristic: Composed of Agents
Economic systems emerge from agent interactions:

> "In these earliest societies, the individual agents probably had a pretty good idea of the value of products and services since they might have had to produce and serve themselves at times." (Ch. 9, p. 434)

#### Relationship to Agency
While not explicitly described as agents themselves, economic systems are driven by agent behavior:

> "The ontogenesis of the HSS economy and its continuing evolution was a result of (1) residual (from the biological mandate) motivation to possess, (2) intentional creations meant to solve some local problem at a given time, and (3) ignorance on the part of intentional agents..." (Ch. 9, p. 434)

#### Human Participation
> "People become economic decision agents when they play the role of a consumer or that of a producer (in their job, for instance)." (Ch. 9, p. 404)

---

## 4. Scale-Dependent Relationships

### 4.1 Multi-Scale Analysis Framework

| Scale Level | Agent Perspective | System Perspective | Example |
|------------|-------------------|-------------------|----------|
| **Micro** | Individual humans as agents | Agents interacting within subsystems | Worker in a company |
| **Meso** | Organizations/subsystems as agents | Systems composed of human agents | Corporation in economy |
| **Macro** | Governance/economy as agents | CAS/CAES composed of subsystems | HSS governance directing resource use |

### 4.2 The Recursive Pattern

1. **Individual Level**: Humans act as agents
2. **Organizational Level**: Collections of humans form subsystems that can act as agents
3. **System Level**: Subsystems like governance can function as agents for larger systems
4. **Meta-System Level**: Entire CAS/CAES interacts with environment

---

## 5. Practical Applications

### 5.1 When to Apply Each Perspective

#### Use the "Agent" Perspective When:
- Analyzing decision-making at any scale
- Modeling system-to-system interactions
- Examining information flows and responses
- Designing control mechanisms

**Example**: Analyzing how a governance system responds to environmental changes

#### Use the "System of Agents" Perspective When:
- Designing internal structures
- Analyzing emergent behaviors
- Understanding decision distribution
- Troubleshooting dysfunction

**Example**: Designing a new organizational structure with appropriate decision rights

### 5.2 Analysis Guidelines

#### For Governance Systems:
1. **As Agent**: How does it sense, decide, and act for the larger system?
2. **As System**: How are decision rights distributed among internal agents?

#### For Economic Systems:
1. **Primary Focus**: How do agent interactions create value flows?
2. **Secondary Focus**: What emergent behaviors arise from these interactions?

#### For Complex Analysis:
1. Start with the scale most relevant to your question
2. Consider both perspectives (agent/system)
3. Look for recursive patterns
4. Account for fuzzy boundaries and temporal variations

---

## 6. Quick Reference Guide

### 6.1 Summary Table

| Archetype | Functions as Agent? | Contains Agents? | Key Quote |
|-----------|-------------------|------------------|-----------|
| **Agent** | Yes (by definition) | Can be decomposed | "special case of an adaptive (and evolvable) system" |
| **Governance** | Yes (explicitly stated) | Yes (needs human agents) | "considered as the agent that makes the decisions" |
| **Economy** | Not explicitly stated | Yes (emerges from agents) | "result of... intentional agents" |

### 6.2 Key Relationships

```
CAS/CAES
├── Governance (can function as agent)
│   └── Contains decision agents
├── Economy (emerges from agents)
│   └── Contains economic agents
└── Other subsystems
    └── Contains various agents

Scale-dependent view:
- From above: Subsystems may appear as agents
- From within: Subsystems contain interacting agents
```

### 6.3 Decision Tree for Analysis

```
Is your focus on decision-making?
├── Yes → Consider agent perspective
│   └── What scale? (individual/subsystem/system)
└── No → Focus on structure/emergence
    └── Analyze as system of agents
```

---

## 7. Glossary

**Agent**: An entity capable of sensing, deciding, and acting based on information and internal knowledge

**CAS**: Complex Adaptive System - a system capable of learning and adapting to environmental changes

**CAES**: Complex Adaptive and Evolvable System - a CAS that can also evolve new structures and functions

**Fuzzy Boundary**: A boundary where components may belong partially or temporally to multiple systems

**HCGS**: Hierarchical Cybernetic Governance System - the fractal-like governance structure in complex systems

**HSS**: Human Social System - the entirety of human society and its institutions

**Membership Function**: Mathematical function determining the degree to which a component belongs to a system

**Recursive Structure**: Pattern where the same organizational principle repeats at different scales

**Scale-Dependent**: Properties or perspectives that change based on the level of analysis

---

## References and Further Reading

### Core Chapters:
- **Chapter 4**: Mathematical framework for system hierarchies
- **Chapter 7**: Human Social System analysis (pp. 368-370)
- **Chapter 9**: Economic subsystem analysis (pp. 394-434)
- **Chapter 10**: Model archetypes (referenced but details not in excerpts)
- **Chapter 11**: Decision agents as adaptive systems
- **Chapter 12**: Hierarchical Cybernetic Governance System

### Key Concepts by Chapter:
- Recursive systems (Ch. 9, p. 394)
- Fuzzy boundaries (Ch. 9, p. 404)
- Governance as agent (Ch. 7, p. 368)
- Economic emergence (Ch. 9, p. 434)
- Distributed governance (Ch. 12)

---

*Note: This synthesis is based on excerpts from George Mobus's "Systems Science: Theory, Analysis, Modeling, and Design" (2022). Page numbers are approximate based on the provided text chunks.*