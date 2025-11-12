# User Story: Flexible System Creation

## The Problem

**Users are frustrated by being forced to add a "waste" output specifically before they can add any inputs to a system.**

### Current User Experience
1. User creates a new system in BERT
2. Wants to add inputs (energy, matter, information)
3. Input buttons don't appear
4. User is confused - where are the input options?
5. Eventually discovers they must first add an output specifically categorized as "waste"
6. User adds meaningless "waste" output just to unlock inputs
7. Feels arbitrary and unnecessarily restrictive

### Real User Feedback
- "Why do I have to add waste before I can add inputs? This is strange and annoying."
- "I have to hunt for the 'waste' option before I can do anything else"
- "I just add a dummy waste output to get past this restriction"
- Multiple users have explicitly complained about this workflow

### Root Cause
The current implementation takes the deep systems analysis methodology too literally - while the methodology emphasizes that all systems have waste outputs, forcing users to define waste specifically (not just any output) creates unnecessary friction and confusion. Systems have many types of outputs - products, services, information - not just waste.

## The Solution: Remove Artificial Restrictions

### User Story
> **As a system modeler**, I want to add inputs immediately after creating a system without being forced to first define a waste output, so that I can model naturally without artificial barriers.

### Key Scenarios

#### Scenario 1: Direct Input Creation
- User creating an electrical circuit system
- Creates new system
- Immediately sees option to add inputs
- Adds power input without hunting for "waste" menu first
- Later adds outputs (products, waste, etc.) as relevant
- **Result**: Natural modeling flow without artificial gates

#### Scenario 2: Meaningful Output Modeling
- User modeling a manufacturing system
- Can start with inputs OR outputs as preferred
- When adding outputs, chooses appropriate type:
  - Product output (manufactured goods)
  - Information output (quality reports)
  - Waste output (when actually relevant)
- **Result**: Outputs are meaningful, not dummy placeholders

#### Scenario 3: Rapid System Sketching
- User quickly prototyping system ideas
- No longer blocked by "find the waste menu" step
- Can freely add inputs and outputs in any order
- **Result**: Faster ideation without menu hunting

### Success Metrics
- Elimination of "dummy" waste outputs created just to proceed
- Reduced time from system creation to first input
- No more user complaints about forced waste creation
- More meaningful output definitions

### Implementation Notes
- Remove the requirement to add waste before showing input options
- All interface elements visible from system creation
- Waste remains an important output type, just not a gate
- Could add gentle reminder about considering waste outputs

### Value Proposition
- **Removes friction**: No artificial barriers to modeling
- **Better UX**: No menu hunting or forced sequences
- **Meaningful modeling**: Outputs added when relevant, not as workarounds
- **Maintains methodology**: Still encourages complete system thinking without forcing it

## Impact on Adoption
- Eliminate a specific pain point multiple users have complained about
- Reduce confusion during initial system creation
- Smoother onboarding experience
- Better alignment between tool behavior and user expectations