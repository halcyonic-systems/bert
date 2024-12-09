# System Language

# Background

System Language (SL) is a formal modeling language used to describe the structures, relations, and functions of systems in all domains — from physics and biology to psychology, government, and cryptoeconomics. 

SL uses generic terms to represent the various elements of systemness — for example nouns like “process” to embody a system that does work, and “flow” to embody the movement of materials, energies, and messages. A few examples of the types of graphical icons that will be used to  represent the lexical elements of SL in a user-friendly tool are pictured below. 

<img width="350" alt="icons" src="https://github.com/user-attachments/assets/950a6f76-7b7b-444f-8e27-efd3a23faa0d">


Users would drag these icons from a palette onto a workspace in order to assemble system constructs. The tool would enforce the syntax as well as capture associated relevant data, such as the domain-specific names and types associated with the generic names and types.

SL is designed to support a holistic process of analysis and synthesis, or Deep Systems Analysis (DSA), that will allow us to better understand complex systems. The language is speakable, viewable, and computable, allowing for humans and machines to discuss all relevant aspects of systemness.

We are developing SL in five distinct layers to ensure that each can be evolved independently while also maintaining compatibility between layers. This approach serves our long term goal of bridging the gap between innate human systems thinking and formal computational implementations while providing a solid theoretical foundation.

# Layer 1 — Theoretical Framework

### Systemese Concepts

Systemese is a term coined by George Mobus to describe the innate human capacity for systems thinking. It is the subconscious language that humans use to recognize and reason about systems and it represents our fundamental cognitive capacity to perceive and understand systemic properties. Key systemese terms such as as boundaries, flows, components, and interactions  are a reflection of how humans naturally process systemic properties and interactions in the real world.

### Ontological Elements

Systemese concepts are formalized in a systems ontology that asserts what fundamental things exist in the domain of systems.  The ontology defines the fundamental elements and categories of systems and specifies the relationship between the elements. With a formal ontology we have the conceptual structure necessary to analyze and describe any system as well as a structured vocabulary for describing systems and relationships. The ontology acts as a bridge between our innate system thinking (systemese) and the rules/patterns we observe in real world systems (principles). 

### System Principles

The twelve principles of systems science describe regular consequences of nature operating according to laws which apply to systems of all forms. They are empirically-observed patterns and regularities which describe key systems properties and how they behave and evolve. The principles serve as a set of fundamental axioms about systemness and they are applied to develop a single, consistent concept of a systems approach that is transdisciplinary and comprehensive. This leads to a set of methodologies that will demonstrate how systems science can become a disciplined and principled way to tackle complex and important problems.

<img width="350" alt="principles" src="https://github.com/user-attachments/assets/8d69fff0-e787-4575-8a5f-72d3c39731c2">


---

Systems principles, systems ontology, and systemese are the theoretical foundation which inform SL’s formal specification. While there is a clear need for further fundamental research in each area, they have been sufficiently developed in Principles of Systems Science and Systems Science: Theory, Analysis, Modeling, and Design for us to start developing a first version of a formal SL. 

Learn more about the principles and the ontology on the following pages:

* [Principles](https://github.com/halcyonic-systems/bert/blob/main/research/theory/principles.md)
* [Ontology](https://github.com/halcyonic-systems/bert/blob/main/research/theory/ontology.md)

# Layer 2 — Formal Specification

A mathematical definition of system sits at the base of SL’s formal specification. The definition informs SL’s formal semantics and syntax rules along with its logical constraints and relationships. It consists a set of mathematical properties and relationships based in set theory and graph theory.  

A system *S* is defined as a 7-tuple:

$$
S_{i, l}=C, N, G, B, T, H, \Delta t_{i, l}
$$

Where:

***C*** = a set of the system's components

***N*** = a graph of internal interactions between system components

***G*** = a graph of external interactions between the system and the environment

***B*** = a boundary

***T*** = a set of transformation rules describing how components transform inputs into outputs

***H*** = an object representing the system's memory, or record of state transitions

$\Delta t$ = a time interval relevant to the level of the system of interest

It must be emphasized that that is very different from the sort of mathematical definitions of system found in classical mathematical systems theory or dynamical systems theory. The equation below is not meant to be “solved” or to facilitate reasoning about the behavior of an abstract mathematical system by performing mathematical operations. It is meant to inform the design of a computational knowledgebase that holds details of a real world system gathered through careful observation and measurement, or Deep System Analysis (DSA).

# Layer 3 —  Knowledge Representation

The systems knowledge base is an active middleware and storage layer that sits between the processes of DSA and the implementation of computational artifacts. It enforces the system structure implied by the formal mathematical definition of system. It also manages system knowledge and enables generation of various outputs such as models and simulations. 

The knowledgebase architecture depicted below includes: RDBMS (core schema based on mathematical system definition), Wiki pages (descriptive content), search engine, and core engine that manages integration. 

<img width="350" alt="kbase" src="https://github.com/user-attachments/assets/a351be91-1b51-4bee-b39c-e9bbb4ccfabd">


This knowledgebase structure makes it possible to generate systems dynamics-like simulation models and produce sysXML specifications.

(Elabaroate a bit more on the relationship between the knowledgebase (Layer 3) and the modeling/simulation capabilities).
# Layer 4 — Implementation Languages

### SysXML

SysXML is markup language that combines, in a systems framework, the descriptive and functional aspects of programming. Rather than being forced to choose between an object-oriented or a process-oriented approach, we can use a “systems-oriented” paradigm that encompasses both.

SysXML is similar in purpose to languages like HTML. It is used to describe systems structures, relations, functions, and representations in a structured format — with the relations between these elements built into the syntax. The syntax allows for the concrete expression of the abstract and conceptual mathematical definition of system while maintaining its implied semantic relationships and constraints. It includes schema and validation rules that are both human and machine readable, thus supporting interoperable human understanding via systemese concepts and machine processing via formal specifications. 

These images depict an illustrative example of sysXML output from the captured data as organized in the knowledgebase after analysis. 

<img width="350" alt="sysxml1" src="https://github.com/user-attachments/assets/452c98de-3d1b-43a4-8b1f-6daaa4d01602">

<img width="350" alt="sysxml2" src="https://github.com/user-attachments/assets/d476b6ac-d1fb-4ff7-a6fd-28be11ab55a9">


### Symbolic Graphical  Language

Each element from the ontology must be translated into operative terms to be used while constructing system descriptions. Each lexical element has a corresponding graphical representation.

This image provides an example of a system of interest in SL graphics along with some of the captured data, that is, labels. 

<img width="350" alt="sysxml3" src="https://github.com/user-attachments/assets/d6ec03d9-ae21-438a-a6b7-dd1bb3ac3753">


# Layer 5 — Application Framework

Implementing SL in a way that is practically usable for analysts required the development of a custom application framework that we are calling the bounded entity reasoning toolbox( BERT.) 

In its final form, BERT should integrate each of the four layers into a user-friendly GUI that supports systems scientists in the process of conducting deep systems analysis. These analyses should faciliate the creation of a wide variety of the types of standard models and simulations embraced by modern systems and complexity researchers such as agent-based, graph/network and system dynamics (SD) models. 

Ultimately we strive to develop hyper-realistic models that, when run dynamically, behaves so much like our actual system of interest that we can manipulate the inputs and derive anticipatory (or predictive) statements about the future state of the system under our imagined conditions. These simulation models will be similar in nature to SD model except that the systems are treated as bounded (modular) processes. And their subsystems are internal bounded processes so that a simulation of a system is a recursive simulation of lower modules each at their level’s appropriate time scale.

<img width="350" alt="sysxml3" src="https://github.com/user-attachments/assets/ec3dc4f3-a445-4977-b2a3-28f2c7386d60">

# Roadmap
