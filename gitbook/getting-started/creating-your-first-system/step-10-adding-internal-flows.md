# Step 10: Add Internal Processors

## How Do Your Boundary Handlers Connect?

You've added boundary handlers that manage your system's external interfaces. Now you need to understand how these handlers connect internally - what processes happen between receiving inputs and delivering outputs?

This step reveals the **internal processors** - subsystems that do the actual transformation work inside your system, connecting your boundary handlers in a processing chain.

## Creating Internal Flows Between Handlers

Start by connecting your boundary handlers to see how resources flow between them:

**Add flows between your boundary handler subsystems:**

1. **Click your input handler** - Add an output flow (what does it send internally?)
2. **Click your output handler** - Add an input flow (what does it receive internally?)  
3. **Click your waste handler** - Add an input flow (what waste does it receive internally?)

<figure><img src="../../.gitbook/assets/internal1.png" alt="" width="375"><figcaption></figcaption></figure>

## Discovering Internal Processors

As you connect these internal flows, you'll discover that some processing must happen between your handlers. These are places where transformation occurs - where inputs become outputs through some kind of work.

## Creating Purely Internal Subsystems

When you identify a transformation point between flows, create an **internal processor subsystem**:

**When you need purely internal subsystems:**
- Processing happens between your boundary handlers
- Transformation work that's not directly connected to external interfaces
- Internal routing, combining, or splitting of flows

**How to create them:**
1. **Connect your flows** between boundary handlers first
2. **Identify transformation points** - where does processing happen along those flows?
3. **Hold Shift + click on flow endpoints** that need processing between them
4. **Click the green circle** to create an internal processor subsystem at that point

<figure><img src="../../.gitbook/assets/internal2 (1).png" alt="" width="375"><figcaption></figcaption></figure>

## Defining Your Internal Processors

Name your internal processors based on what transformation they perform:

**Examples from our model library:**

**Purely internal processors** (not connected to external interfaces):
- **Cell**: "Mitochondria" (converts glucose to ATP internally), "Ribosomes" (translates RNA to proteins internally)
- **Ecosystem**: "Nutrient Cycling" (processes dead matter internally), "Primary Producers" (converts sunlight to biomass internally)
- **Solar Panel**: "Photovoltaic Cells" (converts light to DC power internally), "Inverter" (converts DC to AC internally)
- **Organization**: "Manufacturing" (transforms materials internally), "Quality Control" (validates products internally)
- **LLM**: "Attention Mechanism" (processes information internally), "Neural Networks" (transforms patterns internally)

**Key difference**: These processors work entirely within the system - they receive inputs from other internal subsystems and send outputs to other internal subsystems, never directly to the external environment.

<figure><img src="../../.gitbook/assets/internal3.png" alt="" width="375"><figcaption></figcaption></figure>

## Understanding the Processing Chain

Your complete internal system now shows:
- **Boundary Handlers**: Manage external connections (input, output, waste)
- **Internal Processors**: Transform inputs into outputs through specialized functions
- **Processing Chain**: The sequence of transformations from input to output

## Why Internal Processors Matter

Internal processors reveal:
- **Core Functions**: What your system actually DOES with its inputs
- **Processing Steps**: The sequence of transformations needed
- **Specialization**: How different components handle different aspects of the work

The magic happens in the internal processors. While boundary handlers manage external relationships, internal processors do the actual work that transforms inputs into valuable outputs. Understanding both types of subsystems reveals how your system creates value.