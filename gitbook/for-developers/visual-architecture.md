# Visual Architecture Guide

This guide provides clear, visual explanations of BERT's architecture using simple diagrams. Whether you're a developer getting started or trying to understand how the pieces fit together, these diagrams will help you quickly grasp the system design.

## 🏗️ High-Level Architecture Overview

BERT's architecture consists of five main layers that work together to provide real-time system modeling:

```mermaid
graph TB
    subgraph "BERT Architecture Overview"
        subgraph "Frontend Layer"
            A["Leptos UI<br/>Reactive Web Components<br/>🎨 User Interface"]
        end
        
        subgraph "Core Engine" 
            B["Bevy ECS<br/>Entity-Component-System<br/>⚡ Real-time Rendering"]
        end
        
        subgraph "Platform Layer"
            C["Tauri Framework<br/>Cross-platform Runtime<br/>💻 Desktop Integration"]
        end
        
        subgraph "Data & Logic"
            D["System Elements<br/>Components & Entities<br/>🔧 Business Logic"]
            E["JSON Persistence<br/>File System Access<br/>💾 Data Storage"]
        end
    end
    
    A <--> B
    B <--> D
    B <--> E
    C --> A
    C --> E
    
    style A fill:#e3f2fd
    style B fill:#e8f5e8
    style C fill:#fff3e0
    style D fill:#f3e5f5
    style E fill:#fce4ec
```

**What this shows:** The five main components of BERT and how they interact. The Bevy ECS engine sits at the center, coordinating between the user interface, data storage, and business logic.

**Key insight:** Unlike traditional web applications, BERT uses a game engine (Bevy) for real-time rendering and interaction management.

## 🔄 Data Flow Architecture

This diagram shows how information flows through BERT from user input to visual output:

```mermaid
graph TD
    A["User Input<br/>(Web/Desktop)"] --> B["Leptos UI Layer<br/>Reactive Components"]
    B --> C["Event System<br/>Cross-layer Communication"]
    C --> D["Bevy ECS Core<br/>Entity-Component-System"]
    D --> E["Visual Rendering<br/>Real-time Graphics"]
    D --> F["Data Persistence<br/>JSON + File System"]
    
    D --> G["System Elements<br/>Systems, Flows, Interfaces"]
    G --> H["Interactive Modeling<br/>Live System Diagrams"]
    
    style A fill:#e1f5fe
    style B fill:#f3e5f5
    style C fill:#fff3e0
    style D fill:#e8f5e8
    style E fill:#fce4ec
    style F fill:#f1f8e9
    style G fill:#e0f2f1
    style H fill:#fff8e1
```

**What this shows:** How user interactions flow through the system to create and modify system models in real-time.

**Key insight:** The event system acts as a bridge between the UI and the core engine, enabling reactive updates across the entire application.

## 🌐 Hybrid Deployment Architecture

BERT runs identically in web browsers and as a desktop application:

```mermaid
graph LR
    subgraph "Desktop App"
        A["Tauri Framework<br/>Native Integration"]
    end
    
    subgraph "Web App"  
        B["Browser<br/>WASM Runtime"]
    end
    
    subgraph "Core Application"
        C["Leptos UI<br/>Reactive Interface"]
        D["Bevy Engine<br/>ECS + Rendering"]
        E["System Model<br/>Components"]
    end
    
    subgraph "Data Layer"
        F["JSON Persistence<br/>Cross-platform"]
        G["File System<br/>Hybrid Access"]
    end
    
    A --> C
    B --> C
    C <--> D
    D <--> E
    E <--> F
    F <--> G
    
    style A fill:#e3f2fd
    style B fill:#e3f2fd  
    style C fill:#f3e5f5
    style D fill:#e8f5e8
    style E fill:#fff3e0
    style F fill:#fce4ec
    style G fill:#f1f8e9
```

**What this shows:** How BERT maintains feature parity between web and desktop environments using a single codebase.

**Key insight:** The same core application runs in both contexts, with platform-specific adapters handling differences in file access and native integration.

## 🔗 Component Interaction Lifecycle

This diagram illustrates how flows (connections) are created between system elements:

```mermaid
graph TD
    A["🎯 User Creates Flow"] --> B["📦 Stage 1: Flow Entity<br/>Created from System"]
    B --> C["🔌 Stage 2: Interface<br/>Connection Created"]
    C --> D["🎯 Stage 3: Other End<br/>Definition (Source/Sink)"]
    D --> E["🔌 Stage 4: Interface at<br/>Other End (Optional)"]
    E --> F["✅ Complete Interaction<br/>Flow Fully Connected"]
    
    F --> G["❌ Entity Removal"]
    G --> B
    G --> C
    G --> D
    
    subgraph "Component Types"
        H["FlowStartConnection<br/>FlowEndConnection"]
        I["FlowInterfaceConnection<br/>UI Button Components"]
        J["Flow + FlowCurve<br/>Permanent Components"]
    end
    
    style A fill:#e1f5fe
    style F fill:#e8f5e8
    style G fill:#ffebee
    style H fill:#f3e5f5
    style I fill:#fff3e0
    style J fill:#f1f8e9
```

**What this shows:** The four-stage process for creating connections between system elements, and how the system handles entity removal.

**Key insight:** BERT uses a component-based state machine to manage complex interaction workflows, automatically recovering from entity deletions.

## 🧬 System Language Implementation

This shows how BERT implements systems science theory through modern technology:

```mermaid
graph TD
    subgraph "System Language Implementation"
        A["Systems Science Theory<br/>Mathematical Foundation"]
        B["Component Definitions<br/>Rust Structs + ECS"]
        C["Visual Representation<br/>Interactive Diagrams"]
        D["Real-time Modeling<br/>Live System Analysis"]
    end
    
    subgraph "Technology Stack" 
        E["Rust Language<br/>Performance + Safety"]
        F["Bevy Engine<br/>ECS Architecture"]
        G["Leptos Framework<br/>Reactive UI"]
        H["Tauri Platform<br/>Cross-platform"]
    end
    
    A --> B
    B --> C  
    C --> D
    
    E --> F
    F --> G
    G --> H
    
    B -.-> F
    C -.-> G
    D -.-> H
    
    style A fill:#e1f5fe
    style B fill:#f3e5f5  
    style C fill:#e8f5e8
    style D fill:#fff3e0
    style E fill:#fce4ec
    style F fill:#f1f8e9
    style G fill:#e0f2f1
    style H fill:#fff8e1
```

**What this shows:** How theoretical systems science concepts are translated into practical software components.

**Key insight:** BERT bridges the gap between abstract theory and concrete implementation, making systems science accessible through visual, interactive tools.

## 🎯 Quick Reference

**For New Users:** Start with the [High-Level Architecture](#high-level-architecture-overview) to understand the main components.

**For Developers:** Focus on the [Data Flow](#data-flow-architecture) and [Component Interaction](#component-interaction-lifecycle) diagrams to understand how the system works internally.

**For Researchers:** The [System Language Implementation](#system-language-implementation) diagram shows how BERT translates theory into practice.

**For Platform Questions:** The [Hybrid Deployment](#hybrid-deployment-architecture) diagram explains web vs. desktop differences.

---

💡 **Next Steps:** Ready to dive deeper? Check out the [detailed architecture documentation](architecture.md) or explore the [component definitions in the codebase](https://github.com/your-repo/tree/main/src/bevy_app/components). 