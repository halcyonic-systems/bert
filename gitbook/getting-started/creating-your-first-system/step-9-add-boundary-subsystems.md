# Step 9: Add Boundary Handlers

## How Does Your System Manage Its Interfaces?

You've validated that your system's inputs and outputs make sense. Now it's time to look inside and understand HOW your system actually works. Every interface you've created needs a specialized subsystem to handle it - these are your **boundary handlers**.

Boundary handlers are subsystems that manage your system's connections to the external world. They're the specialized components that receive inputs, deliver outputs, and manage waste - each one linked to a specific interface.

## Creating Boundary Handlers

For each interface you've created, add a boundary handler subsystem:

**Click the green circle above each interface** to create its handler subsystem.

<figure><img src="../../.gitbook/assets/subs1 (1).png" alt="" width="375"><figcaption></figcaption></figure>

**Notice the visual change**: Your system turns from black to white - you've moved from "black box" (external view) to "white box" (internal view) analysis.

<figure><img src="../../.gitbook/assets/subs2 (1).png" alt="" width="375"><figcaption></figcaption></figure>

## Defining Your Boundary Handlers

**Create handlers for ALL your interfaces** - input, output, and waste.

When you click on each handler subsystem, give it a name that reflects what it does:

**Examples from our model library:**
- **Cell**: "Membrane Transport" (input handler), "Secretion Vesicles" (output handler), "Lysosome" (waste handler)
- **Ecosystem**: "Root Uptake" (input handler), "Predation Interface" (output handler), "Decomposer Network" (waste handler)
- **Solar Panel**: "Light Absorption Layer" (input handler), "Power Conditioning" (output handler), "Heat Dissipation" (waste handler)
- **Organization**: "Procurement" (input handler), "Sales & Delivery" (output handler), "Waste Management" (waste handler)
- **LLM**: "Prompt Processing" (input handler), "Response Generation" (output handler), "Error Handling" (waste handler)

<figure><img src="../../.gitbook/assets/subs3.png" alt="" width="375"><figcaption></figcaption></figure>

## Why Boundary Handlers Matter

Boundary handlers reveal:
- **Specialization**: How your system dedicates specific components to managing external relationships
- **Interface Design**: What capabilities your system needs at each boundary point
- **Coordination**: How your system organizes its interactions with the environment

Every interface requires a specialized handler. Systems that try to manage all external interactions through general-purpose components tend to be inefficient and fragile. Good systems have dedicated boundary handlers for each type of external relationship.
