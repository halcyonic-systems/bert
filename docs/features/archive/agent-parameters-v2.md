# Feature: Agent Parameters (ARCHIVED)

> **Archived from**: `feature/agent-params-v2` branch
> **Archive date**: 2026-02-07
> **Superseded by**: `docs/features/unified-agent-system.md` on `feature/unified-agent-system`
> **Reason**: Reconciled with agent-properties branch into unified agent system design

---

## Overview

**Feature Name**: Agent Parameters
**Branch**: feature/agent-params-v2
**Status**: Complete
**Contributors**: Claude + User
**Date**: 2025-10-28

## Description

Extends BERT's AgentModel data structure to support flexible agent behavior parameterization for agent-based modeling (ABM) simulations. This feature enables users to specify cognitive parameters, process configurations, and initial states for agents directly within BERT JSON models, which can then be loaded into Python ABM frameworks like Mesa 3.

**Value Proposition**: Bridges the gap between BERT's systems modeling capabilities and executable ABM simulations by providing a domain-agnostic schema for agent behavior specification. This enables users to design agent-based models in BERT's visual interface and export configurations that drive actual simulations.

## Implemented Functionality

**Core Capabilities**:
- Flexible cognitive parameter storage using `HashMap<String, f64>` for domain-agnostic numeric parameters
- Process assignment configuration with flexible parameter maps for agent behaviors
- Initial state specification using `HashMap<String, serde_json::Value>` for arbitrary agent properties
- Optional network configuration for agent interaction patterns
- Full backward compatibility with existing BERT models (optional fields with defaults)

**User Workflow**:
- Users can specify agent types using existing AgentKind (Reactive/Anticipatory/Intentional)
- Users can define semantic parameter names (e.g., `fee_threshold`, `confidence`, `learning_rate`)
- Users can assign process behaviors with configuration (e.g., `fee_optimization` with parameters)
- Users can set initial agent states (e.g., `wallet: 0`, `fee_history: []`)

**Limitations**:
- Parameters are stored but not executed within BERT (execution happens in external ABM frameworks)
- Network configuration is optional and may not apply to all agent types
- No validation of parameter semantics (domain-specific validation happens in simulation frameworks)

## Technical Implementation

### Components Added

**New Structs in `src/bevy_app/data_model/mod.rs`**:
- `ProcessAssignment`: Maps process names to flexible parameter configurations
- `NetworkConfig`: Optional network behavior configuration for agent interactions

### Components Modified

**`src/bevy_app/data_model/mod.rs`**:
- Created complete `AgentModel` struct from scratch on clean main branch
- Added `AgentKind` enum (Reactive/Anticipatory/Intentional)
- Added `ProcessPrimitive` enum (9 Mobus atomic processes)
- Added flexible parameter fields:
  - `cognitive_params: HashMap<String, f64>` - Domain-agnostic numeric parameters
  - `process_configs: Vec<ProcessAssignment>` - Process behavior configurations
  - `initial_state: HashMap<String, serde_json::Value>` - Initial agent state
  - `network_config: Option<NetworkConfig>` - Optional network behavior
- Added `agent: Option<AgentModel>` field to System struct
- All fields use `#[serde(default)]` for backward compatibility

**`src/bevy_app/data_model/save.rs`**:
- Updated System construction to include `agent: None` field

### Architecture Decisions

**1. Create Complete AgentModel from Scratch**
- **Decision**: Build complete AgentModel on clean main branch (without agent-dynamics code)
- **Rationale**: Main branch had no agent functionality. Creating minimal schema from scratch ensures clean implementation without 5,388 lines of agent-dynamics runtime code.

**2. Flexible Parameter Storage (HashMap)**
- **Decision**: Use `HashMap<String, f64>` for cognitive_params and `HashMap<String, serde_json::Value>` for initial_state
- **Rationale**: Domain-agnostic design allows BERT to support any domain (Bitcoin, ecology, economics) without hardcoding parameter schemas. ABM frameworks define domain semantics.

**3. Process Names as Strings**
- **Decision**: Store process names as strings in `ProcessAssignment` rather than typed enums
- **Rationale**: Decouples BERT schema from execution runtime. ProcessPrimitive enum provides vocabulary (9 atomic processes), but concrete implementations happen in external frameworks.

**4. Optional Fields with Defaults**
- **Decision**: All new fields use `#[serde(default, skip_serializing_if)]`
- **Rationale**: Maintains 100% backward compatibility. Existing BERT models load without changes. New models opt-in to agent parameters.

**5. Minimal Schema (~100 lines vs 5,388 in agent-dynamics)**
- **Decision**: Data schema only, no runtime execution system
- **Rationale**: BERT stores agent designs, Python frameworks execute them. Avoids reimplementing ABM runtime in Rust.

## Usage Examples

### Bitcoin Miner Agent JSON Configuration

```json
{
  "info": {"id": "C0.1", "level": 1, "name": "Mining"},
  "sources": [],
  "sinks": [],
  "parent": "C0",
  "complexity": {"type": "Atomic"},
  "boundary": {
    "info": {"id": "C0.1B", "level": 1, "name": "Mining Boundary"},
    "porosity": 0.5,
    "perceptive_fuzziness": 0.3,
    "interfaces": []
  },
  "radius": 50,
  "agent": {
    "kind": "Anticipatory",
    "primitives": ["Sensing", "Modulating", "Propelling"],
    "cognitive_params": {
      "base_hashrate": 500.0,
      "fee_threshold": 50.0,
      "efficiency": 0.000002,
      "trend_memory_length": 5.0
    },
    "process_configs": [
      {
        "name": "fee_optimization",
        "params": {
          "lookback_window": 10,
          "threshold_adjustment_rate": 0.9
        }
      },
      {
        "name": "block_construction",
        "params": {
          "size_limit": 1000000
        }
      }
    ],
    "initial_state": {
      "wallet": 0,
      "fee_history": [],
      "blocks_mined": 0
    }
  }
}
```

## Related Documentation

- Mobus Agent Hierarchy (Reactive → Anticipatory → Intentional)
- Process primitives from General Systems Theory
- Mesa 3 Documentation

---

_Originally created 2025-10-28. Archived 2026-02-07._
