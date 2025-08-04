# BERT Dynamic Simulations Implementation Guide

## Table of Contents
- [Overview](#overview)
- [Design Principles](#design-principles)
- [Core Dependencies](#core-dependencies)
- [Architecture Overview](#architecture-overview)
- [Implementation Phases](#implementation-phases)
- [Multi-Scale Component Design](#multi-scale-component-design)
- [Physics & Information Theory Integration](#physics--information-theory-integration)
- [Cross-Scale Interaction Framework](#cross-scale-interaction-framework)
- [Spatial Optimization & Performance](#spatial-optimization--performance)
- [System Integration](#system-integration)
- [Development & Debugging Tools](#development--debugging-tools)
- [Research Methodology Framework](#research-methodology-framework)
- [Testing Strategy](#testing-strategy)
- [Extension Pathways](#extension-pathways)

## Overview

This guide outlines how to integrate dynamic multi-scale simulations into the BERT project while preserving all existing static analysis capabilities. The approach treats agents as specialized subsystem components that coexist seamlessly with your current system-level architecture, unifying Systems Dynamics, Agent-Based Modeling, and Network Analysis in a single high-performance environment.

### Key Benefits
- **Unified Multi-Scale Architecture**: Systems, groups, and individuals as different scales of the same component type
- **SD+ABM+Networks Integration**: The holy grail of systems science modeling
- **Physics & Information Theory Constraints**: Scientifically rigorous behavioral foundations
- **Seamless Integration**: Agents emerge from and aggregate back to your existing subsystems
- **Performance Optimized**: 2D spatial optimization for your existing layout
- **Research Ready**: Complete methodology framework for complex systems research

## Design Principles

### 1. **Scale-Agnostic Component Design**
- All entities (systems, groups, agents) use the same base component structure
- Behavior varies by scale, not by component type
- Seamless transitions between scales

### 2. **Preserve Existing Architecture**
- Additive approach - don't replace current systems
- Your JSON configurations become parameter sources for all scales
- Current visualization and UI systems extend naturally

### 3. **Physics & Information Theory Foundations**
- Energy conservation applies at all scales
- Information processing constrained by bandwidth and entropy
- Thermodynamic principles govern system efficiency

### 4. **Cross-Scale Interaction Model**
- Explicit taxonomy of interaction types
- Clear aggregation and decomposition rules
- Emergent system properties from agent behaviors

### 5. **Theory-Informed Implementation**
- Framework supports but doesn't prescribe specific agent types
- Behavioral parameters derived from your existing system properties
- Implementation guided by theoretical foundations and domain expertise

## Core Dependencies

### Add to `Cargo.toml`:

```toml
[dependencies]
# Existing dependencies (keep all current ones)
bevy = "0.16"
petgraph = "0.6"
egui = "0.29"
bevy_egui = "0.31"

# New dependencies for dynamic simulations
rand = "0.8"
rand_distr = "0.4"  # For probability distributions
serde_json = "1.0"  # For configuration serialization
dashmap = "6.1"     # For concurrent data structures
rayon = "1.10"      # For parallel processing
nalgebra = "0.33"   # For linear algebra operations

# Physics and information theory
noise = "0.9"       # For procedural generation and noise
ordered-float = "4.2"  # For deterministic floating point

# Optional: For advanced analytics
plotters = "0.3"    # For data visualization
hdf5 = "0.8"        # For scientific data storage (if available)
```

## Architecture Overview

### Unified Component Hierarchy

```rust
Subsystem (unified component)
├── SubsystemType (Mining, Development, Logistics, Individual, Group)
├── SubsystemScale (System, Group, Individual)
├── OperationalState (Active, Degraded, Adapting)
└── ResourceFlows (inputs, outputs, constraints)

Physics2D (spatial behavior)
├── Position (current, previous)
├── Dynamics (velocity, acceleration)
├── Energy (current, max, consumption_rate)
└── Constraints (movement, boundaries)

InformationState (cognitive/communication)
├── Knowledge (key-value store)
├── Processing (capacity, bandwidth)
├── Communication (range, error_rate)
└── Entropy (information decay)

ThermodynamicState (energy/efficiency)
├── Internal Energy (heat accumulation)
├── Entropy (disorder level)
├── Temperature (efficiency effects)
└── Heat Capacity (thermal properties)
```

### System Architecture Flow

```
Your Existing JSON Data
├── System-Level Parameters → System-Scale Subsystems
├── Decomposition Parameters → Group & Agent-Scale Subsystems  
├── SD Stock/Flow Parameters → Dynamic resource modeling
└── Interaction Rules → Cross-Scale Dynamics

Simulation Loop
├── Update All Subsystems (scale-appropriate behavior)
├── Apply Physics Constraints (energy, thermodynamics)
├── Process Information Flow (Shannon capacity limits)
├── Handle Cross-Scale Interactions
├── Aggregate Agent Effects to Systems
└── Update Visualization (scale-aware LOD)
```

## Implementation Phases

### Phase 1: Unified Component Foundation

#### 1.1 Core Component Structure
```rust
// src/simulation/components/mod.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component, Debug, Clone)]
pub struct Subsystem {
    pub id: String,
    pub subsystem_type: SubsystemType,
    pub scale: SubsystemScale,
    pub operational_state: OperationalState,
}

#[derive(Debug, Clone)]
pub enum SubsystemScale {
    System,      // Your current mining, development, logistics systems
    Group,       // Teams, departments, coordinated units
    Individual,  // Single agents with specific roles
}

#[derive(Debug, Clone)]
pub enum SubsystemType {
    // System-level (preserve your current types)
    Mining(MiningSystem),
    Development(DevelopmentSystem), 
    Logistics(LogisticsSystem),
    Infrastructure(InfrastructureSystem),
    
    // SD Components (stocks and flows)
    Stock(StockComponent),
    Flow(FlowComponent),
    
    // Agent-level (to be defined based on your domain)
    Individual(IndividualAgent),
    Group(GroupAgent),
}

// Systems Dynamics Integration
#[derive(Debug, Clone)]
pub struct StockComponent {
    pub value: f32,
    pub capacity: f32,
    pub inflow_sources: Vec<Entity>,
    pub outflow_sinks: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct FlowComponent {
    pub rate: f32,
    pub source: Option<Entity>,
    pub sink: Option<Entity>,
    pub rate_function: FlowFunction,
}

#[derive(Debug, Clone)]
pub enum FlowFunction {
    Constant(f32),
    Linear { base: f32, multiplier: f32, input: Entity },
    Nonlinear { formula: String }, // For complex SD equations
    AgentDriven, // Rate emerges from agent behaviors
}

// Flexible agent definition - implement based on your needs
#[derive(Debug, Clone)]
pub struct IndividualAgent {
    pub role: String,              // Define based on your domain
    pub capabilities: HashMap<String, f32>,  // Flexible capability system
    pub behavioral_params: HashMap<String, f32>, // From your JSON configs
    pub current_state: String,     // State machine for behaviors
    pub parent_system: Option<Entity>, // Link to decomposed system
}

#[derive(Component, Debug, Clone)]
pub struct Physics2D {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub energy: f32,
    pub max_energy: f32,
    pub mass: f32,
}

#[derive(Component, Debug, Clone)]
pub struct InformationState {
    pub knowledge: HashMap<String, f32>,
    pub processing_capacity: f32,     // bits per second
    pub bandwidth: f32,               // communication bandwidth
    pub communication_range: f32,
    pub noise_tolerance: f32,
    pub information_entropy: f32,
}

#[derive(Component, Debug, Clone)]
pub struct ThermodynamicState {
    pub internal_energy: f32,
    pub entropy: f32,
    pub temperature: f32,
    pub heat_capacity: f32,
}
```

#### 1.2 Scale-Aware Update System
```rust
// src/simulation/systems/unified_dynamics.rs
use bevy::prelude::*;

pub fn unified_subsystem_update(
    mut subsystems: Query<(
        Entity, 
        &mut Subsystem, 
        &mut Physics2D, 
        &mut InformationState,
        &mut ThermodynamicState,
    )>,
    time: Res<Time>,
) {
    for (entity, mut subsystem, mut physics, mut info, mut thermo) in &mut subsystems {
        match &subsystem.scale {
            SubsystemScale::System => {
                update_system_scale_behavior(&mut subsystem, &mut physics, &mut info, &time);
            },
            SubsystemScale::Group => {
                update_group_scale_behavior(&mut subsystem, &mut physics, &mut info, &time);
            },
            SubsystemScale::Individual => {
                update_individual_scale_behavior(&mut subsystem, &mut physics, &mut info, &time);
            },
        }
        
        // Apply universal constraints
        apply_thermodynamic_constraints(&mut physics, &mut thermo, &info, &time);
    }
}
```

### Phase 2: Physics & Information Theory Integration

#### 2.1 Energy Conservation System
```rust
// src/simulation/physics/energy.rs
use bevy::prelude::*;

pub fn energy_conservation_system(
    mut entities: Query<(
        &mut Physics2D,
        &mut ThermodynamicState,
        &InformationState,
        &Subsystem,
    )>,
    time: Res<Time>,
) {
    for (mut physics, mut thermo, info, subsystem) in &mut entities {
        // Movement energy costs
        let kinetic_energy = 0.5 * physics.mass * physics.velocity.length_squared();
        let movement_cost = kinetic_energy * 0.01 * time.delta_seconds();
        
        // Information processing costs (Landauer's principle)
        let info_processing_cost = calculate_information_processing_cost(info, &time);
        
        // Scale-specific energy costs
        let operational_cost = match &subsystem.scale {
            SubsystemScale::System => calculate_system_operational_cost(subsystem),
            SubsystemScale::Group => calculate_group_coordination_cost(subsystem),
            SubsystemScale::Individual => calculate_individual_maintenance_cost(subsystem),
        };
        
        let total_cost = movement_cost + info_processing_cost + operational_cost;
        physics.energy = (physics.energy - total_cost).max(0.0);
        
        // Convert energy consumption to heat
        thermo.internal_energy += total_cost;
        thermo.temperature = thermo.internal_energy / thermo.heat_capacity;
        
        // High temperature reduces efficiency
        if thermo.temperature > 100.0 {
            physics.velocity *= 0.95; // Thermal damage
            // Reduce information processing capacity
        }
        
        // Entropy increases with energy dissipation
        thermo.entropy += total_cost / thermo.temperature.max(1.0);
    }
}

fn calculate_information_processing_cost(info: &InformationState, time: &Time) -> f32 {
    // Landauer's principle: kT ln(2) per bit erased
    let bits_processed = info.processing_capacity * time.delta_seconds();
    let erasure_cost = info.knowledge.len() as f32 * 0.01; // Simplified
    bits_processed * erasure_cost
}
```

#### 2.2 Shannon Information Capacity
```rust
// src/simulation/information/shannon.rs
use bevy::prelude::*;
use std::collections::HashMap;

pub fn information_communication_system(
    mut agents: Query<(Entity, &Physics2D, &mut InformationState)>,
    time: Res<Time>,
    mut transfer_events: EventWriter<InformationTransfer>,
) {
    let agent_positions: Vec<_> = agents.iter().map(|(e, p, i)| (e, p.position, i.communication_range)).collect();
    
    for (entity, physics, mut info) in &mut agents {
        // Information decay (entropy increases over time)
        apply_information_decay(&mut info.knowledge, &time);
        
        // Find communication partners within range
        for &(other_entity, other_pos, _) in &agent_positions {
            if entity == other_entity { continue; }
            
            let distance = physics.position.distance(other_pos);
            if distance <= info.communication_range {
                // Calculate Shannon channel capacity
                let channel_capacity = calculate_channel_capacity_2d(
                    distance,
                    info.bandwidth,
                    info.noise_tolerance,
                    1.0, // Signal power
                );
                
                // Attempt information transfer
                if let Some((key, value)) = select_information_to_share(&info.knowledge) {
                    let transmission_cost = calculate_transmission_cost(value, distance);
                    
                    if transmission_cost <= channel_capacity * time.delta_seconds() {
                        transfer_events.send(InformationTransfer {
                            sender: entity,
                            receiver: other_entity,
                            information: key,
                            value,
                            transmission_cost,
                        });
                    }
                }
            }
        }
        
        // Update information entropy
        info.information_entropy = calculate_information_entropy(&info.knowledge);
    }
}

pub fn calculate_channel_capacity_2d(
    distance: f32,
    bandwidth: f32,
    noise_power: f32,
    signal_power: f32,
) -> f32 {
    // Shannon-Hartley theorem: C = B * log2(1 + S/N)
    // In 2D, signal attenuation follows 1/distance rather than 1/distance²
    let signal_attenuation = 1.0 / (1.0 + distance);
    let effective_signal_power = signal_power * signal_attenuation;
    let signal_to_noise_ratio = effective_signal_power / noise_power;
    
    bandwidth * (1.0 + signal_to_noise_ratio).log2()
}

pub fn calculate_information_entropy(knowledge: &HashMap<String, f32>) -> f32 {
    let total: f32 = knowledge.values().sum();
    if total <= 0.0 { return 0.0; }
    
    knowledge.values()
        .map(|&value| {
            let probability = value / total;
            if probability > 0.0 {
                -probability * probability.log2()
            } else {
                0.0
            }
        })
        .sum()
}

fn apply_information_decay(knowledge: &mut HashMap<String, f32>, time: &Time) {
    for (_, value) in knowledge.iter_mut() {
        *value *= 0.999_f32.powf(time.delta_seconds()); // Exponential decay
    }
    knowledge.retain(|_, &mut value| value > 0.01);
}

#[derive(Event)]
pub struct InformationTransfer {
    pub sender: Entity,
    pub receiver: Entity,
    pub information: String,
    pub value: f32,
    pub transmission_cost: f32,
}
```

## Cross-Scale Interaction Framework

### Interaction Taxonomy

```rust
// src/simulation/interactions/mod.rs
use bevy::prelude::*;

#[derive(Event)]
pub struct CrossScaleInteraction {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub interaction_type: InteractionType,
    pub strength: f32,
}

#[derive(Debug, Clone)]
pub enum InteractionType {
    // System ↔ System
    SystemResource { resource_type: String, flow_rate: f32 },
    SystemDependency { dependency_type: String, strength: f32 },
    
    // System ↔ Agent
    SystemEnvironment { environmental_effect: String, magnitude: f32 },
    AgentSystemFeedback { feedback_type: String, impact: f32 },
    
    // Agent ↔ Agent
    LocalCoordination { coordination_type: String, effectiveness: f32 },
    ResourceSharing { resource: String, amount: f32 },
    InformationExchange { information: String, reliability: f32 },
    
    // Group Dynamics
    GroupFormation { cohesion_strength: f32 },
    GroupDissolving { fragmentation_rate: f32 },
    GroupCoordination { efficiency: f32 },
    
    // SD ↔ ABM Bridge
    StockToAgent { stock_level: f32, agent_effect: f32 },
    AgentToFlow { agent_count: f32, flow_contribution: f32 },
}

pub fn cross_scale_interaction_system(
    all_entities: Query<(Entity, &Subsystem, &Physics2D, &InformationState)>,
    spatial_index: Res<SpatialIndex2D>,
    mut interaction_events: EventWriter<CrossScaleInteraction>,
) {
    for (entity_a, subsystem_a, physics_a, info_a) in &all_entities {
        // Find potential interaction partners
        let nearby_entities = spatial_index.query_nearby(
            physics_a.position, 
            info_a.communication_range.max(100.0)
        );
        
        for entity_b in nearby_entities {
            if entity_a == entity_b { continue; }
            
            if let Ok((_, subsystem_b, physics_b, info_b)) = all_entities.get(entity_b) {
                let distance = physics_a.position.distance(physics_b.position);
                
                // Determine interaction type based on scales
                if let Some(interaction) = determine_interaction_type(
                    &subsystem_a.scale,
                    &subsystem_b.scale,
                    distance,
                    subsystem_a,
                    subsystem_b,
                ) {
                    interaction_events.send(interaction);
                }
            }
        }
    }
}

fn determine_interaction_type(
    scale_a: &SubsystemScale,
    scale_b: &SubsystemScale,
    distance: f32,
    subsystem_a: &Subsystem,
    subsystem_b: &Subsystem,
) -> Option<CrossScaleInteraction> {
    match (scale_a, scale_b) {
        (SubsystemScale::System, SubsystemScale::System) => {
            // System-to-system resource flows or dependencies
            check_system_system_interaction(subsystem_a, subsystem_b, distance)
        },
        
        (SubsystemScale::Individual, SubsystemScale::Individual) if distance < 50.0 => {
            // Local agent interactions
            check_agent_agent_interaction(subsystem_a, subsystem_b, distance)
        },
        
        (SubsystemScale::System, SubsystemScale::Individual) |
        (SubsystemScale::Individual, SubsystemScale::System) => {
            // System environment affects agents
            check_system_agent_interaction(subsystem_a, subsystem_b, distance)
        },
        
        (SubsystemScale::Group, _) | (_, SubsystemScale::Group) => {
            // Group coordination dynamics
            check_group_interaction(subsystem_a, subsystem_b, distance)
        },
        
        _ => None,
    }
}
```

### Systems Dynamics Integration

```rust
// src/simulation/systems_dynamics/mod.rs
use bevy::prelude::*;

pub fn stocks_and_flows_system(
    mut stocks: Query<(&mut StockComponent, &mut Physics2D), With<SubsystemType::Stock>>,
    flows: Query<&FlowComponent, With<SubsystemType::Flow>>,
    agents: Query<(&Subsystem, &Physics2D), With<SubsystemScale::Individual>>,
    time: Res<Time>,
) {
    // Update stock levels based on flows
    for (mut stock, mut stock_physics) in &mut stocks {
        let mut net_flow = 0.0;
        
        // Calculate inflows and outflows
        for flow in &flows {
            let flow_rate = match &flow.rate_function {
                FlowFunction::Constant(rate) => *rate,
                FlowFunction::Linear { base, multiplier, input } => {
                    // Get input value from connected entity
                    *base + (*multiplier * get_entity_value(*input))
                },
                FlowFunction::AgentDriven => {
                    // Flow rate emerges from agent behaviors
                    calculate_agent_driven_flow_rate(&agents, &flow)
                },
                FlowFunction::Nonlinear { formula } => {
                    // Evaluate complex formula (simplified)
                    evaluate_formula(formula)
                },
            };
            
            if flow.sink == Some(stock_physics.position.into()) {
                net_flow += flow_rate;
            }
            if flow.source == Some(stock_physics.position.into()) {
                net_flow -= flow_rate;
            }
        }
        
        // Update stock value
        stock.value += net_flow * time.delta_seconds();
        stock.value = stock.value.clamp(0.0, stock.capacity);
        
        // Stock level affects visualization
        let fill_ratio = stock.value / stock.capacity;
        // Update visual representation based on fill ratio
    }
}

fn calculate_agent_driven_flow_rate(
    agents: &Query<(&Subsystem, &Physics2D), With<SubsystemScale::Individual>>,
    flow: &FlowComponent,
) -> f32 {
    // Aggregate agent activities that contribute to this flow
    let mut total_contribution = 0.0;
    
    for (agent_subsystem, agent_physics) in agents {
        if let SubsystemType::Individual(individual) = &agent_subsystem.subsystem_type {
            // Agent activity contributes to flow rate
            if let Some(activity_level) = individual.capabilities.get("productivity") {
                total_contribution += activity_level;
            }
        }
    }
    
    total_contribution
}
```

## Spatial Optimization & Performance

### 2D Spatial Indexing

```rust
// src/simulation/spatial/mod.rs
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SpatialIndex2D {
    grid_size: f32,
    cells: HashMap<(i32, i32), Vec<Entity>>,
    dirty: bool,
}

impl SpatialIndex2D {
    pub fn new(grid_size: f32) -> Self {
        Self {
            grid_size,
            cells: HashMap::new(),
            dirty: true,
        }
    }
    
    pub fn update(&mut self, entities: Query<(Entity, &Physics2D)>) {
        if self.dirty {
            self.cells.clear();
            
            for (entity, physics) in &entities {
                let cell = self.position_to_cell(physics.position);
                self.cells.entry(cell).or_insert_with(Vec::new).push(entity);
            }
            
            self.dirty = false;
        }
    }
    
    pub fn query_nearby(&self, position: Vec2, radius: f32) -> Vec<Entity> {
        let cell_radius = (radius / self.grid_size).ceil() as i32;
        let center_cell = self.position_to_cell(position);
        
        let mut nearby = Vec::new();
        for x in -cell_radius..=cell_radius {
            for y in -cell_radius..=cell_radius {
                let cell = (center_cell.0 + x, center_cell.1 + y);
                if let Some(entities) = self.cells.get(&cell) {
                    nearby.extend(entities);
                }
            }
        }
        nearby
    }
    
    fn position_to_cell(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / self.grid_size).floor() as i32,
            (position.y / self.grid_size).floor() as i32,
        )
    }
    
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

pub fn spatial_index_update_system(
    mut spatial_index: ResMut<SpatialIndex2D>,
    entities: Query<(Entity, &Physics2D), Changed<Physics2D>>,
) {
    if !entities.is_empty() {
        spatial_index.mark_dirty();
    }
    
    let all_entities = entities.iter().map(|(e, p)| (e, p)).collect::<Vec<_>>();
    if !all_entities.is_empty() {
        spatial_index.update(entities);
    }
}
```

### Level-of-Detail System

```rust
// src/simulation/performance/lod.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct LevelOfDetail {
    pub current_level: LODLevel,
    pub update_frequency: f32,
    pub last_update: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LODLevel {
    High,    // Full simulation updates
    Medium,  // Reduced update frequency
    Low,     // Minimal updates, statistical behavior
    Culled,  // No updates, outside view
}

pub fn lod_management_system(
    mut entities: Query<(Entity, &mut LevelOfDetail, &Physics2D, &Subsystem)>,
    camera: Query<&Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    if let Ok(camera_transform) = camera.get_single() {
        let camera_pos = Vec2::new(camera_transform.translation.x, camera_transform.translation.y);
        
        for (entity, mut lod, physics, subsystem) in &mut entities {
            let distance = physics.position.distance(camera_pos);
            let importance = calculate_entity_importance(subsystem);
            
            // Determine LOD level based on distance and importance
            let new_level = match (distance, importance) {
                (d, i) if d < 100.0 || i > 0.8 => LODLevel::High,
                (d, i) if d < 300.0 || i > 0.5 => LODLevel::Medium,
                (d, i) if d < 500.0 || i > 0.2 => LODLevel::Low,
                _ => LODLevel::Culled,
            };
            
            lod.current_level = new_level;
            
            // Adjust update frequency based on LOD level
            lod.update_frequency = match lod.current_level {
                LODLevel::High => 60.0,   // 60 FPS
                LODLevel::Medium => 30.0, // 30 FPS
                LODLevel::Low => 10.0,    // 10 FPS
                LODLevel::Culled => 1.0,  // 1 FPS
            };
        }
    }
}

fn calculate_entity_importance(subsystem: &Subsystem) -> f32 {
    match &subsystem.scale {
        SubsystemScale::System => 1.0,      // Always important
        SubsystemScale::Group => 0.7,       // Moderately important
        SubsystemScale::Individual => 0.3,  // Less important individually
    }
}

pub fn lod_aware_update_system(
    mut entities: Query<(
        &mut Subsystem,
        &mut Physics2D,
        &LevelOfDetail,
    )>,
    time: Res<Time>,
) {
    for (mut subsystem, mut physics, lod) in &mut entities {
        // Only update if enough time has passed for this LOD level
        let update_interval = 1.0 / lod.update_frequency;
        if time.elapsed_seconds() - lod.last_update < update_interval {
            continue;
        }
        
        match lod.current_level {
            LODLevel::High => {
                // Full simulation update
                update_entity_full(&mut subsystem, &mut physics, &time);
            },
            LODLevel::Medium => {
                // Reduced complexity update
                update_entity_medium(&mut subsystem, &mut physics, &time);
            },
            LODLevel::Low => {
                // Statistical/simplified update
                update_entity_statistical(&mut subsystem, &mut physics, &time);
            },
            LODLevel::Culled => {
                // Minimal or no update
            },
        }
    }
}
```

## System Integration

### Configuration-Driven Parameters

```rust
// src/simulation/config.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Resource, Serialize, Deserialize)]
pub struct SimulationConfiguration {
    pub decomposition_rules: HashMap<String, DecompositionRule>,
    pub physics_parameters: PhysicsParameters,
    pub information_parameters: InformationParameters,
    pub interaction_rules: Vec<InteractionRule>,
    pub systems_dynamics: SystemsDynamicsConfig,
}

#[derive(Serialize, Deserialize)]
pub struct DecompositionRule {
    pub system_type: String,
    pub agent_count_formula: String,
    pub agent_type_distribution: HashMap<String, f32>,
    pub initial_energy: f32,
    pub spawn_pattern: SpawnPattern,
}

#[derive(Serialize, Deserialize)]
pub struct PhysicsParameters {
    pub energy_conservation: bool,
    pub thermodynamics_enabled: bool,
    pub friction_coefficient: f32,
    pub boundary_constraints: BoundaryConfig,
}

#[derive(Serialize, Deserialize)]
pub struct InformationParameters {
    pub enable_shannon_limits: bool,
    pub default_bandwidth: f32,
    pub noise_floor: f32,
    pub entropy_decay_rate: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SystemsDynamicsConfig {
    pub enable_stocks_flows: bool,
    pub default_stock_capacity: f32,
    pub flow_update_frequency: f32,
    pub feedback_delay: f32,
}

impl SimulationConfiguration {
    pub fn from_existing_config(existing: &YourExistingBertConfig) -> Self {
        Self {
            decomposition_rules: derive_decomposition_from_json(existing),
            physics_parameters: PhysicsParameters {
                energy_conservation: true,
                thermodynamics_enabled: true,
                friction_coefficient: 0.02,
                boundary_constraints: BoundaryConfig::from_shapes(&existing.shapes),
            },
            information_parameters: InformationParameters {
                enable_shannon_limits: true,
                default_bandwidth: existing.default_communication_bandwidth.unwrap_or(10.0),
                noise_floor: 0.1,
                entropy_decay_rate: 0.001,
            },
            systems_dynamics: SystemsDynamicsConfig {
                enable_stocks_flows: true,
                default_stock_capacity: 1000.0,
                flow_update_frequency: 60.0,
                feedback_delay: 1.0,
            },
            interaction_rules: derive_interaction_rules(existing),
        }
    }
}
```

### Boundary Integration with Existing Shapes

```rust
// src/simulation/boundaries.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct BoundaryConstraints {
    pub avoid_shapes: bool,
    pub avoidance_strength: f32,
    pub screen_boundaries: Vec2,
}

pub fn boundary_avoidance_system(
    mut agents: Query<(
        &mut Physics2D,
        &BoundaryConstraints,
    ), With<SubsystemScale::Individual>>,
    shapes: Query<&YourExistingShapeComponent>, // Your existing shape data
) {
    for (mut physics, constraints) in &mut agents {
        let mut avoidance_force = Vec2::ZERO;
        
        if constraints.avoid_shapes {
            // Calculate avoidance force from your existing shapes
            for shape in &shapes {
                let distance = calculate_distance_to_shape(physics.position, shape);
                if distance < 30.0 { // Avoidance threshold
                    let direction = calculate_avoidance_direction(physics.position, shape);
                    let force_magnitude = (30.0 - distance) / 30.0 * constraints.avoidance_strength;
                    avoidance_force += direction * force_magnitude;
                }
            }
        }
        
        // Screen boundary avoidance
        let margin = 50.0;
        if physics.position.x < margin {
            avoidance_force.x += constraints.avoidance_strength;
        } else if physics.position.x > constraints.screen_boundaries.x - margin {
            avoidance_force.x -= constraints.avoidance_strength;
        }
        
        if physics.position.y < margin {
            avoidance_force.y += constraints.avoidance_strength;
        } else if physics.position.y > constraints.screen_boundaries.y - margin {
            avoidance_force.y -= constraints.avoidance_strength;
        }
        
        physics.acceleration += avoidance_force;
    }
}

// Implement based on your shape types
fn calculate_distance_to_shape(position: Vec2, shape: &YourExistingShapeComponent) -> f32 {
    // Distance calculation depends on your shape representation
    // Could be rectangle, circle, polygon, etc.
    0.0 // Placeholder
}

fn calculate_avoidance_direction(position: Vec2, shape: &YourExistingShapeComponent) -> Vec2 {
    // Direction vector pointing away from shape
    Vec2::ZERO // Placeholder
}
```

## Development & Debugging Tools

### Visual Debugging System

```rust
// src/simulation/debug/mod.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct DebugVisualization {
    pub show_velocity_vectors: bool,
    pub show_communication_ranges: bool,
    pub show_energy_levels: bool,
    pub show_information_flow: bool,
    pub show_system_boundaries: bool,
    pub show_spatial_grid: bool,
}

impl Default for DebugVisualization {
    fn default() -> Self {
        Self {
            show_velocity_vectors: false,
            show_communication_ranges: false,
            show_energy_levels: false,
            show_information_flow: false,
            show_system_boundaries: false,
            show_spatial_grid: false,
        }
    }
}

pub fn debug_input_system(
    mut debug_viz: ResMut<DebugVisualization>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        debug_viz.show_velocity_vectors = !debug_viz.show_velocity_vectors;
    }
    if keyboard.just_pressed(KeyCode::F2) {
        debug_viz.show_communication_ranges = !debug_viz.show_communication_ranges;
    }
    if keyboard.just_pressed(KeyCode::F3) {
        debug_viz.show_energy_levels = !debug_viz.show_energy_levels;
    }
    if keyboard.just_pressed(KeyCode::F4) {
        debug_viz.show_information_flow = !debug_viz.show_information_flow;
    }
    if keyboard.just_pressed(KeyCode::F5) {
        debug_viz.show_system_boundaries = !debug_viz.show_system_boundaries;
    }
    if keyboard.just_pressed(KeyCode::F6) {
        debug_viz.show_spatial_grid = !debug_viz.show_spatial_grid;
    }
}

pub fn debug_visualization_system(
    debug_viz: Res<DebugVisualization>,
    entities: Query<(
        &Physics2D,
        &InformationState,
        &Subsystem,
    )>,
    spatial_index: Res<SpatialIndex2D>,
    mut gizmos: Gizmos,
) {
    for (physics, info, subsystem) in &entities {
        // Velocity vectors
        if debug_viz.show_velocity_vectors && physics.velocity.length() > 0.1 {
            let end_pos = physics.position + physics.velocity.normalize() * 20.0;
            gizmos.line_2d(physics.position, end_pos, Color::YELLOW);
        }
        
        // Communication ranges
        if debug_viz.show_communication_ranges {
            gizmos.circle_2d(
                physics.position,
                info.communication_range,
                Color::srgba(0.0, 1.0, 0.0, 0.3),
            );
        }
        
        // Energy levels as colored halos
        if debug_viz.show_energy_levels {
            let energy_ratio = physics.energy / physics.max_energy;
            let color = Color::srgb(1.0 - energy_ratio, energy_ratio, 0.0);
            gizmos.circle_2d(
                physics.position,
                15.0,
                color.with_alpha(0.5),
            );
        }
    }
    
    // Spatial grid
    if debug_viz.show_spatial_grid {
        draw_spatial_grid(&spatial_index, &mut gizmos);
    }
}

fn draw_spatial_grid(spatial_index: &SpatialIndex2D, gizmos: &mut Gizmos) {
    let grid_size = spatial_index.grid_size;
    let bounds = Vec2::new(1000.0, 800.0); // Adjust to your screen size
    
    // Draw grid lines
    let mut x = 0.0;
    while x <= bounds.x {
        gizmos.line_2d(Vec2::new(x, 0.0), Vec2::new(x, bounds.y), Color::srgba(0.3, 0.3, 0.3, 0.5));
        x += grid_size;
    }
    
    let mut y = 0.0;
    while y <= bounds.y {
        gizmos.line_2d(Vec2::new(0.0, y), Vec2::new(bounds.x, y), Color::srgba(0.3, 0.3, 0.3, 0.5));
        y += grid_size;
    }
}
```

### System Stepping for Analysis

```rust
// src/simulation/debug/stepping.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct SystemStepping {
    pub enabled: bool,
    pub step_mode: StepMode,
    pub steps_remaining: u32,
}

#[derive(Debug, Clone)]
pub enum StepMode {
    Paused,
    SingleStep,
    MultiStep(u32),
    Running,
}

pub fn stepping_input_system(
    mut stepping: ResMut<SystemStepping>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        stepping.enabled = !stepping.enabled;
        if stepping.enabled {
            stepping.step_mode = StepMode::Paused;
        } else {
            stepping.step_mode = StepMode::Running;
        }
    }
    
    if stepping.enabled {
        if keyboard.just_pressed(KeyCode::ArrowRight) {
            stepping.step_mode = StepMode::SingleStep;
            stepping.steps_remaining = 1;
        }
        
        if keyboard.just_pressed(KeyCode::Enter) {
            stepping.step_mode = StepMode::MultiStep(10);
            stepping.steps_remaining = 10;
        }
    }
}

pub fn stepping_control_system(
    mut stepping: ResMut<SystemStepping>,
    mut simulation_time: ResMut<Time<Virtual>>,
) {
    match stepping.step_mode {
        StepMode::Paused => {
            simulation_time.pause();
        },
        StepMode::SingleStep => {
            if stepping.steps_remaining > 0 {
                simulation_time.unpause();
                stepping.steps_remaining -= 1;
                if stepping.steps_remaining == 0 {
                    stepping.step_mode = StepMode::Paused;
                }
            }
        },
        StepMode::MultiStep(_) => {
            if stepping.steps_remaining > 0 {
                simulation_time.unpause();
                stepping.steps_remaining -= 1;
                if stepping.steps_remaining == 0 {
                    stepping.step_mode = StepMode::Paused;
                }
            }
        },
        StepMode::Running => {
            simulation_time.unpause();
        },
    }
}
```

## Research Methodology Framework

### Emergence Detection and Metrics

```rust
// src/research/emergence.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Resource, Serialize, Deserialize)]
pub struct EmergenceMetrics {
    // Network-level emergence
    pub clustering_coefficient: f32,
    pub path_length_efficiency: f32,
    pub network_modularity: f32,
    
    // Information-level emergence  
    pub information_propagation_speed: f32,
    pub knowledge_concentration: f32,
    pub communication_efficiency: f32,
    
    // Energy-level emergence
    pub energy_distribution_inequality: f32,
    pub thermodynamic_efficiency: f32,
    pub energy_flow_patterns: Vec<f32>,
    
    // System-level emergence
    pub system_coherence: f32,
    pub adaptation_rate: f32,
    pub resilience_measure: f32,
    
    // Cross-scale emergence
    pub scale_coupling_strength: f32,
    pub emergence_cascade_effects: f32,
    
    // Time series for trend analysis
    pub metric_history: HashMap<String, Vec<(f32, f32)>>, // (time, value)
}

pub fn emergence_analysis_system(
    entities: Query<(
        &Subsystem,
        &Physics2D,
        &InformationState,
        &ThermodynamicState,
    )>,
    network: Res<YourNetworkResource>,
    mut metrics: ResMut<EmergenceMetrics>,
    time: Res<Time>,
) {
    // Collect data for analysis
    let agent_data: Vec<_> = entities.iter()
        .filter(|(s, _, _, _)| matches!(s.scale, SubsystemScale::Individual))
        .collect();
    
    let system_data: Vec<_> = entities.iter()
        .filter(|(s, _, _, _)| matches!(s.scale, SubsystemScale::System))
        .collect();
    
    // Calculate network emergence metrics
    metrics.clustering_coefficient = calculate_dynamic_clustering(&agent_data);
    metrics.network_modularity = calculate_network_modularity(&agent_data, &network);
    
    // Calculate information emergence
    metrics.information_propagation_speed = calculate_info_propagation_speed(&agent_data);
    metrics.knowledge_concentration = calculate_knowledge_concentration(&agent_data);
    
    // Calculate energy emergence
    metrics.energy_distribution_inequality = calculate_energy_inequality(&agent_data);
    metrics.thermodynamic_efficiency = calculate_system_efficiency(&agent_data, &system_data);
    
    // Calculate system-level emergence
    metrics.system_coherence = calculate_system_coherence(&system_data);
    metrics.scale_coupling_strength = calculate_scale_coupling(&agent_data, &system_data);
    
    // Store historical data
    let current_time = time.elapsed_seconds();
    store_metric_history(&mut metrics.metric_history, "clustering", current_time, metrics.clustering_coefficient);
    store_metric_history(&mut metrics.metric_history, "info_speed", current_time, metrics.information_propagation_speed);
    store_metric_history(&mut metrics.metric_history, "energy_inequality", current_time, metrics.energy_distribution_inequality);
    
    // Detect emergence events
    detect_emergence_events(&metrics);
}

fn calculate_dynamic_clustering(
    agents: &[(
        &Subsystem,
        &Physics2D,
        &InformationState,
        &ThermodynamicState,
    )]
) -> f32 {
    // Calculate how agents cluster in space over time
    let positions: Vec<Vec2> = agents.iter().map(|(_, p, _, _)| p.position).collect();
    
    if positions.len() < 3 { return 0.0; }
    
    let mut total_clustering = 0.0;
    let mut count = 0;
    
    for (i, pos_i) in positions.iter().enumerate() {
        let neighbors: Vec<_> = positions.iter().enumerate()
            .filter(|(j, pos_j)| i != *j && pos_i.distance(**pos_j) < 100.0)
            .collect();
        
        if neighbors.len() < 2 { continue; }
        
        // Calculate local clustering coefficient
        let mut connections = 0;
        for (j, _) in &neighbors {
            for (k, _) in &neighbors {
                if j != k && positions[*j].distance(positions[*k]) < 100.0 {
                    connections += 1;
                }
            }
        }
        
        let max_connections = neighbors.len() * (neighbors.len() - 1);
        if max_connections > 0 {
            total_clustering += connections as f32 / max_connections as f32;
            count += 1;
        }
    }
    
    if count > 0 { total_clustering / count as f32 } else { 0.0 }
}

fn calculate_info_propagation_speed(
    agents: &[(
        &Subsystem,
        &Physics2D,
        &InformationState,
        &ThermodynamicState,
    )]
) -> f32 {
    // Measure how quickly information spreads through the agent network
    let mut total_entropy = 0.0;
    let mut max_entropy = 0.0;
    
    for (_, _, info, _) in agents {
        total_entropy += info.information_entropy;
        max_entropy += info.knowledge.len() as f32; // Maximum possible entropy
    }
    
    if max_entropy > 0.0 {
        1.0 - (total_entropy / max_entropy) // Higher spread = lower entropy concentration
    } else {
        0.0
    }
}

fn calculate_energy_inequality(
    agents: &[(
        &Subsystem,
        &Physics2D,
        &InformationState,
        &ThermodynamicState,
    )]
) -> f32 {
    // Calculate Gini coefficient of energy distribution
    let energies: Vec<f32> = agents.iter().map(|(_, p, _, _)| p.energy).collect();
    calculate_gini_coefficient(&energies)
}

fn calculate_gini_coefficient(values: &[f32]) -> f32 {
    if values.len() < 2 { return 0.0; }
    
    let mut sorted_values = values.to_vec();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted_values.len() as f32;
    let mean = sorted_values.iter().sum::<f32>() / n;
    
    if mean == 0.0 { return 0.0; }
    
    let mut sum = 0.0;
    for (i, value) in sorted_values.iter().enumerate() {
        sum += (2.0 * (i as f32 + 1.0) - n - 1.0) * value;
    }
    
    sum / (n * n * mean)
}

fn store_metric_history(
    history: &mut HashMap<String, Vec<(f32, f32)>>,
    metric_name: &str,
    time: f32,
    value: f32,
) {
    let entry = history.entry(metric_name.to_string()).or_insert_with(Vec::new);
    entry.push((time, value));
    
    // Keep only last 1000 data points to prevent unbounded growth
    if entry.len() > 1000 {
        entry.remove(0);
    }
}

fn detect_emergence_events(metrics: &EmergenceMetrics) {
    // Detect significant changes in metrics that indicate emergence
    // Phase transitions, sudden clustering, cascade effects, etc.
    
    for (metric_name, history) in &metrics.metric_history {
        if history.len() >= 10 {
            let recent_slope = calculate_trend_slope(&history[history.len()-10..]);
            if recent_slope.abs() > 0.1 { // Threshold for significant change
                println!("Emergence event detected in {}: slope = {}", metric_name, recent_slope);
            }
        }
    }
}

fn calculate_trend_slope(data: &[(f32, f32)]) -> f32 {
    if data.len() < 2 { return 0.0; }
    
    let n = data.len() as f32;
    let sum_x: f32 = data.iter().map(|(t, _)| *t).sum();
    let sum_y: f32 = data.iter().map(|(_, v)| *v).sum();
    let sum_xy: f32 = data.iter().map(|(t, v)| t * v).sum();
    let sum_x2: f32 = data.iter().map(|(t, _)| t * t).sum();
    
    let denominator = n * sum_x2 - sum_x * sum_x;
    if denominator.abs() < 0.001 { return 0.0; }
    
    (n * sum_xy - sum_x * sum_y) / denominator
}
```

### Parameter Sensitivity Analysis

```rust
// src/research/sensitivity.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Resource)]
pub struct ParameterSweep {
    pub current_experiment: usize,
    pub parameter_space: ParameterSpace,
    pub experiment_results: Vec<ExperimentResult>,
    pub sweep_state: SweepState,
}

#[derive(Debug, Clone)]
pub enum SweepState {
    Setup,
    Running { experiments_completed: usize, total_experiments: usize },
    Analysis,
    Complete,
}

#[derive(Debug, Clone)]
pub struct ParameterSpace {
    pub parameters: HashMap<String, ParameterRange>,
    pub combinations: Vec<HashMap<String, f32>>,
}

#[derive(Debug, Clone)]
pub struct ParameterRange {
    pub min: f32,
    pub max: f32,
    pub steps: usize,
    pub scale: ParameterScale,
}

#[derive(Debug, Clone)]
pub enum ParameterScale {
    Linear,
    Logarithmic,
    Custom(Vec<f32>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub parameters: HashMap<String, f32>,
    pub final_metrics: EmergenceMetrics,
    pub convergence_time: f32,
    pub stability_measure: f32,
    pub phase_transitions: Vec<PhaseTransition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTransition {
    pub time: f32,
    pub metric: String,
    pub magnitude: f32,
    pub transition_type: TransitionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    Gradual,
    Sudden,
    Oscillatory,
    Chaotic,
}

pub fn parameter_sweep_orchestration(
    mut sweep: ResMut<ParameterSweep>,
    metrics: Res<EmergenceMetrics>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<SimulationState>>,
    mut config: ResMut<SimulationConfiguration>,
) {
    match &sweep.sweep_state {
        SweepState::Setup => {
            generate_parameter_combinations(&mut sweep);
            sweep.sweep_state = SweepState::Running { 
                experiments_completed: 0, 
                total_experiments: sweep.parameter_space.combinations.len() 
            };
        },
        
        SweepState::Running { experiments_completed, total_experiments } => {
            // Check if current experiment is complete
            if time.elapsed_seconds() > 300.0 || check_convergence(&metrics) {
                // Record results
                let result = ExperimentResult {
                    parameters: sweep.parameter_space.combinations[sweep.current_experiment].clone(),
                    final_metrics: metrics.clone(),
                    convergence_time: time.elapsed_seconds(),
                    stability_measure: calculate_stability_measure(&metrics),
                    phase_transitions: detect_phase_transitions(&metrics),
                };
                
                sweep.experiment_results.push(result);
                
                // Move to next experiment
                sweep.current_experiment += 1;
                if sweep.current_experiment >= *total_experiments {
                    sweep.sweep_state = SweepState::Analysis;
                } else {
                    // Apply next parameter combination
                    apply_parameter_combination(
                        &sweep.parameter_space.combinations[sweep.current_experiment],
                        &mut config
                    );
                    next_state.set(SimulationState::Reset);
                }
            }
        },
        
        SweepState::Analysis => {
            perform_sensitivity_analysis(&sweep.experiment_results);
            export_sweep_results(&sweep.experiment_results);
            sweep.sweep_state = SweepState::Complete;
        },
        
        SweepState::Complete => {
            // Sweep finished, return to normal operation
        },
    }
}

fn generate_parameter_combinations(sweep: &mut ParameterSweep) {
    sweep.parameter_space.combinations.clear();
    
    let parameter_names: Vec<String> = sweep.parameter_space.parameters.keys().cloned().collect();
    let mut value_sets: Vec<Vec<f32>> = Vec::new();
    
    for name in &parameter_names {
        let range = &sweep.parameter_space.parameters[name];
        let values = generate_parameter_values(range);
        value_sets.push(values);
    }
    
    // Generate all combinations (Cartesian product)
    let combinations = cartesian_product(&value_sets);
    
    for combination in combinations {
        let mut param_map = HashMap::new();
        for (i, name) in parameter_names.iter().enumerate() {
            param_map.insert(name.clone(), combination[i]);
        }
        sweep.parameter_space.combinations.push(param_map);
    }
}

fn generate_parameter_values(range: &ParameterRange) -> Vec<f32> {
    match &range.scale {
        ParameterScale::Linear => {
            let step_size = (range.max - range.min) / (range.steps - 1) as f32;
            (0..range.steps)
                .map(|i| range.min + i as f32 * step_size)
                .collect()
        },
        ParameterScale::Logarithmic => {
            let log_min = range.min.ln();
            let log_max = range.max.ln();
            let step_size = (log_max - log_min) / (range.steps - 1) as f32;
            (0..range.steps)
                .map(|i| (log_min + i as f32 * step_size).exp())
                .collect()
        },
        ParameterScale::Custom(values) => values.clone(),
    }
}

fn cartesian_product(sets: &[Vec<f32>]) -> Vec<Vec<f32>> {
    if sets.is_empty() {
        return vec![vec![]];
    }
    
    let mut result = vec![vec![]];
    
    for set in sets {
        let mut new_result = Vec::new();
        for existing in &result {
            for &value in set {
                let mut new_combination = existing.clone();
                new_combination.push(value);
                new_result.push(new_combination);
            }
        }
        result = new_result;
    }
    
    result
}

fn perform_sensitivity_analysis(results: &[ExperimentResult]) {
    // Calculate parameter sensitivity indices
    // Identify which parameters have the strongest effect on which metrics
    // Detect parameter interactions and nonlinear effects
    
    println!("Sensitivity Analysis Results:");
    
    for metric_name in ["clustering_coefficient", "information_propagation_speed", "energy_distribution_inequality"] {
        println!("\nMetric: {}", metric_name);
        
        // Calculate correlation between each parameter and this metric
        for param_name in get_parameter_names(results) {
            let correlation = calculate_parameter_metric_correlation(results, &param_name, metric_name);
            println!("  {} correlation: {:.3}", param_name, correlation);
        }
    }
}

fn calculate_parameter_metric_correlation(
    results: &[ExperimentResult],
    param_name: &str,
    metric_name: &str,
) -> f32 {
    let param_values: Vec<f32> = results.iter()
        .filter_map(|r| r.parameters.get(param_name).copied())
        .collect();
    
    let metric_values: Vec<f32> = results.iter()
        .map(|r| extract_metric_value(&r.final_metrics, metric_name))
        .collect();
    
    if param_values.len() != metric_values.len() || param_values.len() < 2 {
        return 0.0;
    }
    
    calculate_correlation(&param_values, &metric_values)
}

fn calculate_correlation(x: &[f32], y: &[f32]) -> f32 {
    if x.len() != y.len() || x.len() < 2 { return 0.0; }
    
    let n = x.len() as f32;
    let sum_x: f32 = x.iter().sum();
    let sum_y: f32 = y.iter().sum();
    let sum_xy: f32 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let sum_x2: f32 = x.iter().map(|a| a * a).sum();
    let sum_y2: f32 = y.iter().map(|b| b * b).sum();
    
    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();
    
    if denominator.abs() < 0.001 { 0.0 } else { numerator / denominator }
}
```

## Testing Strategy

### Multi-Scale Validation

```rust
// src/simulation/tests/integration.rs
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    
    #[test]
    fn test_energy_conservation_across_scales() {
        let mut app = create_test_app();
        
        // Spawn mixed-scale entities
        spawn_test_system(&mut app, Vec2::new(100.0, 100.0));
        spawn_test_agents(&mut app, 10, Vec2::new(100.0, 100.0));
        
        let initial_energy = calculate_total_system_energy(&app);
        
        // Run simulation
        for _ in 0..100 {
            app.update();
        }
        
        let final_energy = calculate_total_system_energy(&app);
        
        // Energy should be conserved (within reasonable numerical error)
        let energy_loss = (initial_energy - final_energy) / initial_energy;
        assert!(energy_loss < 0.05, "Energy loss too high: {}", energy_loss);
    }
    
    #[test]
    fn test_information_entropy_bounds() {
        let mut app = create_test_app();
        spawn_test_agents(&mut app, 20, Vec2::new(200.0, 200.0));
        
        for _ in 0..200 {
            app.update();
            
            // Check that information entropy stays within bounds
            let agents = app.world().query::<&InformationState>();
            for info in agents.iter(app.world()) {
                assert!(info.information_entropy >= 0.0);
                assert!(info.information_entropy <= info.knowledge.len() as f32);
            }
        }
    }
    
    #[test]
    fn test_scale_transition_consistency() {
        let mut app = create_test_app();
        
        // Start with system-level entity
        let system_entity = spawn_test_system(&mut app, Vec2::new(150.0, 150.0));
        
        // Measure system properties
        let initial_properties = measure_system_properties(&app, system_entity);
        
        // Decompose to agents
        trigger_system_decomposition(&mut app, system_entity);
        app.update();
        
        // Run for some time to let agents settle
        for _ in 0..50 {
            app.update();
        }
        
        // Measure aggregated agent properties
        let aggregated_properties = measure_aggregated_agent_properties(&app, system_entity);
        
        // Properties should be approximately conserved
        assert_properties_similar(&initial_properties, &aggregated_properties, 0.1);
    }
    
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_systems(Update, (
               unified_subsystem_update,
               energy_conservation_system,
               information_communication_system,
           ));
        app
    }
    
    fn calculate_total_system_energy(app: &App) -> f32 {
        let physics_query = app.world().query::<&Physics2D>();
        physics_query.iter(app.world()).map(|p| p.energy).sum()
    }
}
```

## Extension Pathways

### Future Integration Opportunities

#### 1. **Machine Learning Integration**
```rust
// Future: Train agent behaviors from real data
#[derive(Component)]
pub struct LearnedBehavior {
    pub neural_network: Box<dyn NeuralNetwork>,
    pub training_data: Vec<StateActionPair>,
    pub learning_rate: f32,
}

pub fn adaptive_learning_system(
    mut agents: Query<(&mut LearnedBehavior, &InformationState, &Physics2D)>,
) {
    // Agents adapt their behaviors based on outcomes
    // Could integrate with reinforcement learning libraries
}
```

#### 2. **GPU Acceleration**
```rust
// Future: Offload heavy computations to GPU
use bevy::render::render_resource::*;

#[derive(Component)]
pub struct GPUAccelerated {
    pub compute_shader: Handle<Shader>,
    pub data_buffer: Buffer,
}

pub fn gpu_interaction_system(
    gpu_entities: Query<&GPUAccelerated>,
    render_device: Res<RenderDevice>,
) {
    // Use Bevy's compute shaders for massive parallel agent updates
    // Particularly useful for large-scale information propagation
    // and spatial neighbor finding
}
```

#### 3. **Distributed Simulation**
```rust
// Future: Multi-machine simulation for massive scale
#[derive(Resource)]
pub struct DistributedSimulation {
    pub node_id: u32,
    pub peer_nodes: Vec<NetworkNode>,
    pub entity_partitioning: HashMap<Entity, u32>,
}

pub fn distributed_synchronization_system(
    mut distributed: ResMut<DistributedSimulation>,
    cross_boundary_entities: Query<(&Subsystem, &Physics2D), With<CrossBoundaryMarker>>,
) {
    // Synchronize entities that cross node boundaries
    // Enable simulations with millions of agents across clusters
}
```

#### 4. **Real-Time Collaboration**
```rust
// Future: Multi-user simulation sessions
#[derive(Resource)]
pub struct CollaborativeSession {
    pub session_id: String,
    pub connected_users: Vec<UserId>,
    pub user_viewports: HashMap<UserId, ViewportState>,
}

pub fn collaborative_interaction_system(
    mut session: ResMut<CollaborativeSession>,
    user_inputs: Res<MultiUserInput>,
) {
    // Multiple researchers can interact with same simulation
    // Each user can control different aspects or scales
    // Real-time parameter adjustment by different team members
}
```

#### 5. **Advanced Network Dynamics**
```rust
// Future: Dynamic network topology
#[derive(Component)]
pub struct DynamicNetworkNode {
    pub connection_rules: ConnectionRules,
    pub formation_threshold: f32,
    pub decay_rate: f32,
}

#[derive(Debug, Clone)]
pub enum ConnectionRules {
    Proximity { max_distance: f32 },
    Similarity { threshold: f32, attributes: Vec<String> },
    Activity { min_interaction_frequency: f32 },
    Preferential { attachment_probability: f32 },
}

pub fn dynamic_network_formation(
    mut network: ResMut<YourNetworkResource>,
    agents: Query<(&Physics2D, &InformationState, &DynamicNetworkNode)>,
) {
    // Network topology evolves based on agent behaviors
    // New connections form, old ones decay
    // Network structure becomes emergent property of agent dynamics
}
```

#### 6. **Environmental Integration**
```rust
// Future: Environmental constraints and dynamics
#[derive(Resource)]
pub struct Environment {
    pub temperature_field: Vec<Vec<f32>>,
    pub resource_density: Vec<Vec<f32>>,
    pub flow_patterns: Vec<Vec<Vec2>>,
    pub seasonal_cycles: SeasonalModel,
}

#[derive(Component)]
pub struct EnvironmentalSensitivity {
    pub temperature_tolerance: f32,
    pub resource_requirements: HashMap<String, f32>,
    pub adaptation_rate: f32,
}

pub fn environmental_effects_system(
    environment: Res<Environment>,
    mut agents: Query<(
        &mut Physics2D,
        &mut InformationState,
        &EnvironmentalSensitivity,
    )>,
) {
    // Agents respond to environmental conditions
    // Environmental changes drive system dynamics
    // Climate effects on complex systems
}
```

## Advanced Research Applications

### Complex Systems Phase Transitions

```rust
// src/research/phase_transitions.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct PhaseTransitionDetector {
    pub monitoring_metrics: Vec<String>,
    pub detection_windows: HashMap<String, Vec<f32>>,
    pub transition_thresholds: HashMap<String, f32>,
    pub detected_transitions: Vec<PhaseTransition>,
}

pub fn phase_transition_detection(
    mut detector: ResMut<PhaseTransitionDetector>,
    metrics: Res<EmergenceMetrics>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds();
    
    for metric_name in &detector.monitoring_metrics {
        let current_value = extract_metric_value(&metrics, metric_name);
        
        // Add to sliding window
        let window = detector.detection_windows.entry(metric_name.clone()).or_insert_with(Vec::new);
        window.push(current_value);
        
        // Keep window size manageable
        if window.len() > 100 {
            window.remove(0);
        }
        
        // Detect sudden changes (phase transitions)
        if window.len() >= 20 {
            let recent_mean = window[window.len()-10..].iter().sum::<f32>() / 10.0;
            let previous_mean = window[window.len()-20..window.len()-10].iter().sum::<f32>() / 10.0;
            
            let change_magnitude = (recent_mean - previous_mean).abs();
            let threshold = detector.transition_thresholds.get(metric_name).unwrap_or(&0.1);
            
            if change_magnitude > *threshold {
                let transition_type = classify_transition(window);
                
                detector.detected_transitions.push(PhaseTransition {
                    time: current_time,
                    metric: metric_name.clone(),
                    magnitude: change_magnitude,
                    transition_type,
                });
                
                println!("Phase transition detected in {} at time {}: magnitude {:.3}", 
                    metric_name, current_time, change_magnitude);
            }
        }
    }
}

fn classify_transition(window: &[f32]) -> TransitionType {
    if window.len() < 10 { return TransitionType::Gradual; }
    
    // Calculate variance to detect oscillatory behavior
    let mean = window.iter().sum::<f32>() / window.len() as f32;
    let variance = window.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / window.len() as f32;
    
    // Calculate trend to detect sudden vs gradual changes
    let trend_slope = calculate_trend_slope(&window.iter().enumerate()
        .map(|(i, &v)| (i as f32, v)).collect::<Vec<_>>());
    
    match (variance, trend_slope.abs()) {
        (v, s) if v > 0.5 && s > 0.1 => TransitionType::Chaotic,
        (v, s) if v > 0.2 && s < 0.05 => TransitionType::Oscillatory,
        (_, s) if s > 0.1 => TransitionType::Sudden,
        _ => TransitionType::Gradual,
    }
}
```

### Multi-Objective System Optimization

```rust
// src/research/optimization.rs
use bevy::prelude::*;

#[derive(Resource)]
pub struct MultiObjectiveOptimizer {
    pub objectives: Vec<OptimizationObjective>,
    pub current_solution: ParameterSet,
    pub pareto_front: Vec<Solution>,
    pub generation: usize,
}

#[derive(Debug, Clone)]
pub struct OptimizationObjective {
    pub name: String,
    pub target: ObjectiveTarget,
    pub weight: f32,
    pub metric_extractor: fn(&EmergenceMetrics) -> f32,
}

#[derive(Debug, Clone)]
pub enum ObjectiveTarget {
    Maximize,
    Minimize,
    Target(f32),
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub parameters: HashMap<String, f32>,
    pub objective_values: Vec<f32>,
    pub fitness: f32,
}

pub fn multi_objective_optimization_system(
    mut optimizer: ResMut<MultiObjectiveOptimizer>,
    metrics: Res<EmergenceMetrics>,
    mut config: ResMut<SimulationConfiguration>,
    time: Res<Time>,
) {
    // Evaluate current solution
    if time.elapsed_seconds() > 60.0 { // Run each configuration for 1 minute
        let objective_values = evaluate_objectives(&optimizer.objectives, &metrics);
        
        let solution = Solution {
            parameters: extract_current_parameters(&config),
            objective_values: objective_values.clone(),
            fitness: calculate_fitness(&objective_values, &optimizer.objectives),
        };
        
        // Update Pareto front
        update_pareto_front(&mut optimizer.pareto_front, solution);
        
        // Generate next solution using genetic algorithm
        optimizer.current_solution = generate_next_solution(
            &optimizer.pareto_front,
            optimizer.generation,
        );
        
        apply_parameter_set(&optimizer.current_solution, &mut config);
        optimizer.generation += 1;
        
        // Log progress
        if optimizer.generation % 10 == 0 {
            print_optimization_progress(&optimizer);
        }
    }
}

fn evaluate_objectives(
    objectives: &[OptimizationObjective],
    metrics: &EmergenceMetrics,
) -> Vec<f32> {
    objectives.iter()
        .map(|obj| (obj.metric_extractor)(metrics))
        .collect()
}

fn update_pareto_front(pareto_front: &mut Vec<Solution>, candidate: Solution) {
    // Remove dominated solutions
    pareto_front.retain(|existing| !dominates(&candidate, existing));
    
    // Add candidate if it's not dominated
    if !pareto_front.iter().any(|existing| dominates(existing, &candidate)) {
        pareto_front.push(candidate);
    }
    
    // Keep Pareto front size manageable
    if pareto_front.len() > 50 {
        pareto_front.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        pareto_front.truncate(50);
    }
}

fn dominates(solution_a: &Solution, solution_b: &Solution) -> bool {
    let mut better_in_any = false;
    
    for (a_val, b_val) in solution_a.objective_values.iter().zip(&solution_b.objective_values) {
        if a_val < b_val {
            return false; // A is worse in this objective
        }
        if a_val > b_val {
            better_in_any = true; // A is better in this objective
        }
    }
    
    better_in_any
}
```

### Real-World Data Integration

```rust
// src/research/data_integration.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct RealWorldDataSource {
    pub data_streams: HashMap<String, DataStream>,
    pub calibration_parameters: CalibrationParameters,
    pub validation_metrics: ValidationMetrics,
}

#[derive(Debug, Clone)]
pub struct DataStream {
    pub source_type: DataSourceType,
    pub update_frequency: f32,
    pub data_buffer: Vec<DataPoint>,
    pub preprocessing: PreprocessingPipeline,
}

#[derive(Debug, Clone)]
pub enum DataSourceType {
    CsvFile { path: String },
    DatabaseConnection { connection_string: String },
    ApiEndpoint { url: String, headers: HashMap<String, String> },
    SensorNetwork { device_ids: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: f64,
    pub values: HashMap<String, f32>,
    pub metadata: HashMap<String, String>,
}

pub fn real_world_data_integration(
    mut data_source: ResMut<RealWorldDataSource>,
    mut agents: Query<(&mut Subsystem, &mut Physics2D, &mut InformationState)>,
    time: Res<Time>,
) {
    // Update data streams
    for (stream_name, stream) in &mut data_source.data_streams {
        if should_update_stream(stream, &time) {
            let new_data = fetch_data_from_stream(stream);
            stream.data_buffer.extend(new_data);
            
            // Keep buffer size manageable
            if stream.data_buffer.len() > 1000 {
                stream.data_buffer.drain(0..100);
            }
        }
    }
    
    // Apply real-world data to simulation
    for (mut subsystem, mut physics, mut info) in &mut agents {
        if let Some(calibration) = find_calibration_for_agent(&subsystem, &data_source.calibration_parameters) {
            apply_real_world_calibration(&mut subsystem, &mut physics, &mut info, &calibration, &data_source);
        }
    }
    
    // Validate simulation against real-world data
    validate_simulation_accuracy(&data_source, &agents.iter().collect::<Vec<_>>());
}

fn apply_real_world_calibration(
    subsystem: &mut Subsystem,
    physics: &mut Physics2D,
    info: &mut InformationState,
    calibration: &AgentCalibration,
    data_source: &RealWorldDataSource,
) {
    // Map real-world observations to agent parameters
    // For example: social media activity → information processing capacity
    // Economic indicators → energy levels
    // Geographic mobility data → movement patterns
    
    if let Some(activity_stream) = data_source.data_streams.get(&calibration.activity_metric) {
        if let Some(latest_data) = activity_stream.data_buffer.last() {
            if let Some(activity_level) = latest_data.values.get("activity") {
                // Scale agent parameters based on real-world activity
                info.processing_capacity = calibration.base_processing * (1.0 + activity_level * 0.5);
                physics.max_energy = calibration.base_energy * (1.0 + activity_level * 0.3);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentCalibration {
    pub agent_type: String,
    pub activity_metric: String,
    pub base_processing: f32,
    pub base_energy: f32,
    pub scaling_factors: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub struct CalibrationParameters {
    pub agent_calibrations: Vec<AgentCalibration>,
    pub temporal_alignment: f32, // How to align simulation time with real time
    pub spatial_scaling: f32,    // How to scale spatial coordinates
}

#[derive(Debug, Clone)]
pub struct ValidationMetrics {
    pub correlation_targets: Vec<CorrelationTarget>,
    pub accuracy_thresholds: HashMap<String, f32>,
    pub validation_history: Vec<ValidationResult>,
}

#[derive(Debug, Clone)]
pub struct CorrelationTarget {
    pub real_world_metric: String,
    pub simulation_metric: String,
    pub expected_correlation: f32,
    pub tolerance: f32,
}
```

## Production Deployment Considerations

### Configuration Management

```rust
// src/deployment/config_management.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Resource, Serialize, Deserialize)]
pub struct ProductionConfig {
    pub performance_profile: PerformanceProfile,
    pub logging_config: LoggingConfig,
    pub monitoring_config: MonitoringConfig,
    pub safety_limits: SafetyLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceProfile {
    Development { max_agents: usize },
    Research { max_agents: usize, gpu_enabled: bool },
    Production { max_agents: usize, gpu_enabled: bool, distributed: bool },
    Demo { max_agents: usize, visual_quality: VisualQuality },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyLimits {
    pub max_simulation_time: f32,
    pub max_memory_usage: usize,
    pub max_cpu_usage: f32,
    pub emergency_shutdown_triggers: Vec<EmergencyTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyTrigger {
    MemoryExceeded { threshold_mb: usize },
    InfiniteLoop { max_iterations: usize },
    DivergentBehavior { metric: String, threshold: f32 },
}

pub fn production_monitoring_system(
    config: Res<ProductionConfig>,
    metrics: Res<EmergenceMetrics>,
    system_info: Res<SystemInformation>,
    mut emergency_shutdown: EventWriter<EmergencyShutdown>,
) {
    // Monitor system resources
    for trigger in &config.safety_limits.emergency_shutdown_triggers {
        match trigger {
            EmergencyTrigger::MemoryExceeded { threshold_mb } => {
                if system_info.memory_usage_mb > *threshold_mb {
                    emergency_shutdown.send(EmergencyShutdown {
                        reason: format!("Memory usage exceeded: {} MB", system_info.memory_usage_mb),
                        timestamp: system_info.current_time,
                    });
                }
            },
            EmergencyTrigger::DivergentBehavior { metric, threshold } => {
                let current_value = extract_metric_value(&metrics, metric);
                if current_value > *threshold {
                    emergency_shutdown.send(EmergencyShutdown {
                        reason: format!("Divergent behavior in {}: {:.3}", metric, current_value),
                        timestamp: system_info.current_time,
                    });
                }
            },
            _ => {} // Handle other triggers
        }
    }
}
```

### Data Export and Analysis Pipeline

```rust
// src/export/data_pipeline.rs
use bevy::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Resource)]
pub struct DataExportPipeline {
    pub export_schedule: ExportSchedule,
    pub output_formats: Vec<OutputFormat>,
    pub data_collectors: Vec<Box<dyn DataCollector>>,
}

#[derive(Debug, Clone)]
pub enum ExportSchedule {
    Continuous { interval_seconds: f32 },
    OnEvent { events: Vec<String> },
    OnCompletion,
    Custom { schedule_function: String },
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json { pretty_print: bool },
    Csv { delimiter: char },
    HDF5 { compression: bool },
    Parquet { compression: String },
    Custom { exporter_name: String },
}

pub trait DataCollector: Send + Sync {
    fn collect_data(
        &self,
        world: &World,
        time: f32,
    ) -> serde_json::Value;
    
    fn get_schema(&self) -> DataSchema;
}

pub struct AgentStateCollector;

impl DataCollector for AgentStateCollector {
    fn collect_data(&self, world: &World, time: f32) -> serde_json::Value {
        let mut agent_data = Vec::new();
        
        let mut query = world.query::<(
            Entity,
            &Subsystem,
            &Physics2D,
            &InformationState,
            &ThermodynamicState,
        )>();
        
        for (entity, subsystem, physics, info, thermo) in query.iter(world) {
            agent_data.push(serde_json::json!({
                "entity_id": format!("{:?}", entity),
                "subsystem_type": format!("{:?}", subsystem.subsystem_type),
                "scale": format!("{:?}", subsystem.scale),
                "position": [physics.position.x, physics.position.y],
                "velocity": [physics.velocity.x, physics.velocity.y],
                "energy": physics.energy,
                "information_entropy": info.information_entropy,
                "temperature": thermo.temperature,
                "timestamp": time,
            }));
        }
        
        serde_json::json!({
            "timestamp": time,
            "agents": agent_data
        })
    }
    
    fn get_schema(&self) -> DataSchema {
        DataSchema {
            name: "AgentStates".to_string(),
            version: "1.0".to_string(),
            fields: vec![
                FieldSchema { name: "entity_id".to_string(), field_type: "string".to_string() },
                FieldSchema { name: "position".to_string(), field_type: "array<float>".to_string() },
                FieldSchema { name: "energy".to_string(), field_type: "float".to_string() },
                // ... more fields
            ],
        }
    }
}

pub fn data_export_system(
    mut pipeline: ResMut<DataExportPipeline>,
    world: &World,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds();
    
    let should_export = match &pipeline.export_schedule {
        ExportSchedule::Continuous { interval_seconds } => {
            current_time % interval_seconds < time.delta_seconds()
        },
        ExportSchedule::OnCompletion => false, // Handle in different system
        _ => false, // Implement other schedules
    };
    
    if should_export {
        for collector in &pipeline.data_collectors {
            let data = collector.collect_data(world, current_time);
            
            for format in &pipeline.output_formats {
                export_data_in_format(&data, format, current_time);
            }
        }
    }
}

fn export_data_in_format(
    data: &serde_json::Value,
    format: &OutputFormat,
    timestamp: f32,
) {
    match format {
        OutputFormat::Json { pretty_print } => {
            let filename = format!("simulation_data_{:.0}.json", timestamp);
            let mut file = File::create(&filename).expect("Failed to create file");
            
            let json_string = if *pretty_print {
                serde_json::to_string_pretty(data).unwrap()
            } else {
                serde_json::to_string(data).unwrap()
            };
            
            file.write_all(json_string.as_bytes()).expect("Failed to write data");
        },
        
        OutputFormat::Csv { delimiter } => {
            // Convert JSON to CSV format
            let filename = format!("simulation_data_{:.0}.csv", timestamp);
            export_json_as_csv(data, &filename, *delimiter);
        },
        
        _ => {
            println!("Export format not yet implemented: {:?}", format);
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataSchema {
    pub name: String,
    pub version: String,
    pub fields: Vec<FieldSchema>,
}

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub name: String,
    pub field_type: String,
}

#[derive(Event)]
pub struct EmergencyShutdown {
    pub reason: String,
    pub timestamp: f32,
}
```

## Quick Start Implementation Roadmap

### Week-by-Week Implementation Plan

#### **Week 1: Foundation**
1. ✅ Add unified `Subsystem` component structure
2. ✅ Implement basic `Physics2D` and `InformationState` components  
3. ✅ Create scale-aware update system
4. ✅ Test with one existing system type

#### **Week 2: Physics Integration**
1. ✅ Implement energy conservation system
2. ✅ Add thermodynamic constraints
3. ✅ Create Shannon information capacity limits
4. ✅ Test energy balance across simulation runs

#### **Week 3: Agent Spawning**
1. ✅ Implement system decomposition (KeyA)
2. ✅ Create agent spawning from your JSON configs
3. ✅ Add basic agent behaviors (movement, communication)
4. ✅ Test agent-system linkage

#### **Week 4: Cross-Scale Interactions**
1. ✅ Implement agent-to-system aggregation
2. ✅ Add system-to-agent environmental effects
3. ✅ Create basic emergence metrics
4. ✅ Test full multi-scale simulation loop

#### **Week 5: Systems Dynamics Integration**
1. ✅ Add SD stock and flow components
2. ✅ Implement agent-driven flow rates
3. ✅ Connect SD variables to agent behaviors
4. ✅ Test SD+ABM+Network integration

#### **Week 6: Optimization & Polish**
1. ✅ Add spatial indexing for performance
2. ✅ Implement LOD system
3. ✅ Add debugging visualization tools
4. ✅ Create basic data export

### Validation Checkpoints

#### **After Week 2**: Energy Conservation
- [ ] Total system energy remains constant (±5%)
- [ ] Information entropy stays within theoretical bounds
- [ ] Thermodynamic temperature correlates with activity

#### **After Week 4**: Multi-Scale Consistency  
- [ ] Agent behaviors aggregate to expected system properties
- [ ] System changes propagate to agent behaviors
- [ ] Emergence metrics show expected patterns

#### **After Week 6**: Full Integration
- [ ] SD stocks respond to agent activities
- [ ] Network structure affects both SD and ABM dynamics
- [ ] Parameter changes create predictable system responses

## Conclusion

This hybrid implementation guide provides both the architectural framework for unified Systems Dynamics + Agent-Based Modeling + Network Analysis and the detailed implementation guidance needed to build it in your BERT project. The approach preserves your existing architecture while adding the multi-scale dynamic capabilities that could revolutionize how complex systems research is conducted.

The key innovations of this approach are:

1. **Unified Component Architecture**: All scales use the same base components
2. **Physics + Information Theory Foundation**: Scientifically rigorous constraints
3. **Cross-Scale Interaction Framework**: Explicit modeling of system-agent dynamics  
4. **Theory-Informed Flexibility**: Framework supports but doesn't prescribe specific behaviors
5. **Production-Ready Design**: Built for real research applications, not just demos

By following this implementation guide, you'll create not just a better tool, but potentially a new standard for how systems scientists study complex, multi-scale phenomena.