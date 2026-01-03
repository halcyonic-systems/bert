//! # Complexity Calculator Module
//!
//! This module implements Mobus Simonian Complexity calculation for BERT system models,
//! based on equation 4.10 from Systems Science theory.
//!
//! ## Simonian Complexity Formula
//!
//! ```
//! C = Σ(l=0 to L) Σ(i=0 to I_l) (C_il + N_il) + B + ln(f(S,T))
//! ```
//!
//! Where:
//! - **C_il**: Number of components at level l, component i
//! - **N_il**: Number of relations/interactions at level l, component i  
//! - **B**: Boundary size (number of interfaces)
//! - **f(S,T)**: State space function (states S + transitions T)
//!
//! ## Implementation Strategy
//!
//! Maps BERT WorldModel structure to Simonian complexity:
//! - Components: Systems at each hierarchy level
//! - Relations: Interactions/flows between systems
//! - Boundary: Interfaces on system boundaries
//! - State Space: Approximated from flows and parameters
//!
//! ## Usage
//!
//! ```rust
//! use bert::data_model::{WorldModel, complexity_calculator::calculate_simonian_complexity};
//!
//! let world_model = WorldModel { /* ... */ };
//! let complexity = calculate_simonian_complexity(&world_model);
//! println!("System complexity: {:.2}", complexity.total_complexity);
//! ```

use crate::bevy_app::data_model::{HasInfo, WorldModel};
use std::collections::HashMap;

/// Detailed breakdown of complexity calculation components
#[derive(Debug, Clone)]
pub struct ComplexityBreakdown {
    /// Components count by hierarchy level (C_il)
    pub components_by_level: HashMap<i32, usize>,

    /// Relations count by hierarchy level (N_il)  
    pub relations_by_level: HashMap<i32, usize>,

    /// Total boundary interfaces (B)
    pub boundary_interfaces: usize,

    /// State space approximation (S + T)
    pub state_space_size: usize,

    /// Final Simonian complexity value
    pub total_complexity: f64,

    /// Intermediate calculation values for debugging
    pub component_sum: usize,
    pub relation_sum: usize,
    pub state_space_log: f64,
}

impl ComplexityBreakdown {
    /// Creates a new empty complexity breakdown
    pub fn new() -> Self {
        Self {
            components_by_level: HashMap::new(),
            relations_by_level: HashMap::new(),
            boundary_interfaces: 0,
            state_space_size: 0,
            total_complexity: 0.0,
            component_sum: 0,
            relation_sum: 0,
            state_space_log: 0.0,
        }
    }
}

/// Calculates Mobus Simonian Complexity for a BERT WorldModel
///
/// Implements equation 4.10 from Systems Science theory by analyzing
/// the hierarchical structure, interactions, and boundary complexity
/// of the system model.
///
/// # Parameters
///
/// - `world_model`: The complete BERT system model to analyze
///
/// # Returns
///
/// A `ComplexityBreakdown` containing the detailed calculation results
/// and final complexity value.
///
/// # Examples
///
/// ```rust
/// use bert::data_model::{WorldModel, complexity_calculator::calculate_simonian_complexity};
///
/// let world_model = WorldModel::load_from_file("cell.json").unwrap();
/// let complexity = calculate_simonian_complexity(&world_model);
///
/// println!("Total complexity: {:.2}", complexity.total_complexity);
/// println!("Components by level: {:?}", complexity.components_by_level);
/// ```
///
/// # Implementation Notes
///
/// - Components are counted by their hierarchy level from the Info struct
/// - Relations include all interactions between systems and external entities
/// - Boundary size counts all interfaces across all systems
/// - State space is approximated from flow parameters and system count
pub fn calculate_simonian_complexity(world_model: &WorldModel) -> ComplexityBreakdown {
    let mut breakdown = ComplexityBreakdown::new();

    // Step 1: Count components by hierarchy level (C_il)
    count_components_by_level(&world_model, &mut breakdown);

    // Step 2: Count relations by hierarchy level (N_il)
    count_relations_by_level(&world_model, &mut breakdown);

    // Step 3: Count boundary interfaces (B)
    count_boundary_interfaces(&world_model, &mut breakdown);

    // Step 4: Approximate state space size (S + T)
    approximate_state_space(&world_model, &mut breakdown);

    // Step 5: Calculate final complexity using equation 4.10
    calculate_final_complexity(&mut breakdown);

    // Log detailed breakdown for debugging
    log_complexity_breakdown(&breakdown);

    breakdown
}

/// Counts components at each hierarchy level
fn count_components_by_level(world_model: &WorldModel, breakdown: &mut ComplexityBreakdown) {
    // Count environment as level -1
    breakdown.components_by_level.insert(-1, 1);

    // Count all systems by their hierarchy level
    for system in &world_model.systems {
        let level = system.info().level;
        *breakdown.components_by_level.entry(level).or_insert(0) += 1;
    }

    // Count external entities (sources and sinks) at environment level
    let external_count =
        world_model.environment.sources.len() + world_model.environment.sinks.len();
    *breakdown.components_by_level.entry(-1).or_insert(0) += external_count;

    // Calculate total component sum
    breakdown.component_sum = breakdown.components_by_level.values().sum();
}

/// Counts relations/interactions at each hierarchy level
fn count_relations_by_level(world_model: &WorldModel, breakdown: &mut ComplexityBreakdown) {
    // All interactions are at the environment level (-1) in current BERT structure
    let interaction_count = world_model.interactions.len();
    breakdown.relations_by_level.insert(-1, interaction_count);

    // Calculate total relation sum
    breakdown.relation_sum = breakdown.relations_by_level.values().sum();
}

/// Counts all boundary interfaces across all systems
fn count_boundary_interfaces(world_model: &WorldModel, breakdown: &mut ComplexityBreakdown) {
    let mut interface_count = 0;

    // Count interfaces in all systems
    for system in &world_model.systems {
        interface_count += system.boundary.interfaces.len();
    }

    breakdown.boundary_interfaces = interface_count;
}

/// Approximates state space size from system dynamics
fn approximate_state_space(world_model: &WorldModel, breakdown: &mut ComplexityBreakdown) {
    // Simple approximation: each flow represents potential state transitions
    // Each flow parameter adds to state space dimensionality
    let mut state_space = 0;

    // Count flows (potential states)
    state_space += world_model.interactions.len();

    // Count flow parameters (state dimensions)
    for interaction in &world_model.interactions {
        state_space += interaction.parameters.len();
    }

    // Add system count (each system can be in different states)
    state_space += world_model.systems.len();

    // Minimum state space size of 1 to avoid log(0)
    breakdown.state_space_size = state_space.max(1);
}

/// Calculates final complexity using Simonian formula
fn calculate_final_complexity(breakdown: &mut ComplexityBreakdown) {
    // Calculate state space logarithm
    breakdown.state_space_log = (breakdown.state_space_size as f64).ln();

    // Apply Simonian complexity formula: C = Σ(C_il + N_il) + B + ln(f(S,T))
    breakdown.total_complexity = (breakdown.component_sum + breakdown.relation_sum) as f64
        + breakdown.boundary_interfaces as f64
        + breakdown.state_space_log;
}

/// Logs detailed complexity breakdown for debugging
fn log_complexity_breakdown(breakdown: &ComplexityBreakdown) {
    println!("=== Simonian Complexity Calculation ===");
    println!("Components by level: {:?}", breakdown.components_by_level);
    println!("Relations by level: {:?}", breakdown.relations_by_level);
    println!("Total components (ΣC_il): {}", breakdown.component_sum);
    println!("Total relations (ΣN_il): {}", breakdown.relation_sum);
    println!("Boundary interfaces (B): {}", breakdown.boundary_interfaces);
    println!("State space size (S+T): {}", breakdown.state_space_size);
    println!("State space log: {:.3}", breakdown.state_space_log);
    println!("TOTAL COMPLEXITY: {:.3}", breakdown.total_complexity);
    println!("========================================");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bevy_app::data_model::{
        Environment, Id, IdType, Info, WorldModel, CURRENT_FILE_VERSION,
    };

    #[test]
    fn test_empty_world_complexity() {
        let world_model = WorldModel {
            version: CURRENT_FILE_VERSION,
            environment: Environment {
                info: Info {
                    id: Id {
                        ty: IdType::Environment,
                        indices: vec![-1],
                    },
                    level: -1,
                    name: "Test Environment".to_string(),
                    description: "Test".to_string(),
                },
                sources: vec![],
                sinks: vec![],
            },
            systems: vec![],
            interactions: vec![],
            hidden_entities: vec![],
        };

        let complexity = calculate_simonian_complexity(&world_model);

        // Should have minimal complexity: 1 environment + 0 interfaces + ln(1) = 1.0
        assert_eq!(complexity.component_sum, 1);
        assert_eq!(complexity.relation_sum, 0);
        assert_eq!(complexity.boundary_interfaces, 0);
        assert_eq!(complexity.total_complexity, 1.0);
    }

    #[test]
    fn test_cell_json_complexity() {
        use std::fs;

        // Load and test the actual cell.json model
        if let Ok(cell_json_bytes) = fs::read("assets/models/examples/cell.json") {
            // Parse directly with serde_json since load functions are complex
            if let Ok(world_model) = serde_json::from_slice::<WorldModel>(&cell_json_bytes) {
                println!("\n=== Testing cell.json complexity ===");
                let complexity = calculate_simonian_complexity(&world_model);

                // Basic sanity checks - cell should have some complexity
                assert!(complexity.component_sum > 0, "Should have components");
                assert!(complexity.boundary_interfaces > 0, "Should have interfaces");
                assert!(
                    complexity.total_complexity > 1.0,
                    "Should have meaningful complexity"
                );

                println!("Cell complexity: {:.3}", complexity.total_complexity);
            } else {
                println!("Warning: Could not parse cell.json - skipping test");
            }
        } else {
            println!("Warning: Could not load cell.json - skipping test");
        }
    }
}
