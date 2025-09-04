//! Smart Parameters System - Enhanced parameter system with categorical variables
//!
//! Provides context-aware parameter suggestions supporting numeric, ordinal, categorical,
//! and boolean parameter types based on substance type context.

use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use crate::SubstanceType;

/// Enhanced parameter value supporting multiple data types (Cliff's framework)
#[derive(Clone, Debug, Reflect, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterValue {
    /// Numeric parameter with unit (traditional quantified measurements)
    Numeric { 
        value: String, 
        unit: String 
    },
    /// Ordinal parameter with total ordering (high/medium/low)
    Ordinal { 
        level: String, 
        options: Vec<String> 
    },
    /// Categorical parameter with discrete options (solid/liquid/gas)
    Categorical { 
        value: String, 
        options: Vec<String> 
    },
    /// Boolean parameter with custom labels (active/inactive)
    Boolean { 
        value: bool, 
        true_label: String, 
        false_label: String 
    },
}

/// Smart parameter with enhanced type system
#[derive(Clone, Debug, Reflect, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmartParameter {
    /// Unique identifier for this parameter (excluded from serialization)
    #[serde(skip)]
    #[reflect(ignore)]
    pub id: Uuid,
    /// Human-readable parameter name
    pub name: String,
    /// Parameter value with type information
    pub value: ParameterValue,
}

/// Parameter suggestion for autocomplete and intelligent input
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterSuggestion {
    /// Display name shown to user
    pub display_name: String,
    /// Parameter type for this suggestion
    pub parameter_type: ParameterType,
    /// Search terms for fuzzy matching
    pub search_terms: Vec<String>,
    /// Default parameter value template
    pub default_value: ParameterValue,
}

/// Parameter type classification
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum ParameterType {
    Numeric,
    Ordinal, 
    Categorical,
    Boolean,
}

/// Static database providing context-aware parameter suggestions
#[derive(Debug, Clone)]
pub struct SmartParameterDatabase {
    /// Parameter suggestions organized by substance type
    suggestions: HashMap<SubstanceType, Vec<ParameterSuggestion>>,
}

impl SmartParameterDatabase {
    /// Create new database with static parameter suggestions
    pub fn new() -> Self {
        let mut suggestions = HashMap::new();
        
        // Energy substance type suggestions
        suggestions.insert(SubstanceType::Energy, vec![
            ParameterSuggestion {
                display_name: "Temperature".to_string(),
                parameter_type: ParameterType::Numeric,
                search_terms: vec!["temp".to_string(), "temperature".to_string(), "heat".to_string()],
                default_value: ParameterValue::Numeric { 
                    value: "".to_string(), 
                    unit: "°C".to_string() 
                },
            },
            ParameterSuggestion {
                display_name: "Power".to_string(),
                parameter_type: ParameterType::Numeric,
                search_terms: vec!["power".to_string(), "pow".to_string(), "rate".to_string()],
                default_value: ParameterValue::Numeric { 
                    value: "".to_string(), 
                    unit: "watts".to_string() 
                },
            },
            ParameterSuggestion {
                display_name: "Efficiency".to_string(),
                parameter_type: ParameterType::Ordinal,
                search_terms: vec!["efficiency".to_string(), "eff".to_string(), "performance".to_string()],
                default_value: ParameterValue::Ordinal { 
                    level: "Good".to_string(), 
                    options: vec!["Excellent".to_string(), "Good".to_string(), "Fair".to_string(), "Poor".to_string()] 
                },
            },
            ParameterSuggestion {
                display_name: "Active".to_string(),
                parameter_type: ParameterType::Boolean,
                search_terms: vec!["active".to_string(), "enabled".to_string(), "on".to_string()],
                default_value: ParameterValue::Boolean { 
                    value: true, 
                    true_label: "Active".to_string(), 
                    false_label: "Inactive".to_string() 
                },
            },
        ]);

        // Material substance type suggestions
        suggestions.insert(SubstanceType::Material, vec![
            ParameterSuggestion {
                display_name: "Flow Rate".to_string(),
                parameter_type: ParameterType::Numeric,
                search_terms: vec!["flow".to_string(), "rate".to_string(), "throughput".to_string()],
                default_value: ParameterValue::Numeric { 
                    value: "".to_string(), 
                    unit: "kg/s".to_string() 
                },
            },
            ParameterSuggestion {
                display_name: "Material State".to_string(),
                parameter_type: ParameterType::Categorical,
                search_terms: vec!["state".to_string(), "phase".to_string(), "form".to_string()],
                default_value: ParameterValue::Categorical { 
                    value: "Liquid".to_string(), 
                    options: vec!["Solid".to_string(), "Liquid".to_string(), "Gas".to_string(), "Plasma".to_string()] 
                },
            },
            ParameterSuggestion {
                display_name: "Pressure".to_string(),
                parameter_type: ParameterType::Numeric,
                search_terms: vec!["pressure".to_string(), "force".to_string()],
                default_value: ParameterValue::Numeric { 
                    value: "".to_string(), 
                    unit: "Pa".to_string() 
                },
            },
            ParameterSuggestion {
                display_name: "Bidirectional".to_string(),
                parameter_type: ParameterType::Boolean,
                search_terms: vec!["bidirectional".to_string(), "direction".to_string(), "reversible".to_string()],
                default_value: ParameterValue::Boolean { 
                    value: false, 
                    true_label: "Bidirectional".to_string(), 
                    false_label: "Unidirectional".to_string() 
                },
            },
        ]);

        // Message substance type suggestions  
        suggestions.insert(SubstanceType::Message, vec![
            ParameterSuggestion {
                display_name: "Bandwidth".to_string(),
                parameter_type: ParameterType::Numeric,
                search_terms: vec!["bandwidth".to_string(), "rate".to_string(), "throughput".to_string()],
                default_value: ParameterValue::Numeric { 
                    value: "".to_string(), 
                    unit: "bits/s".to_string() 
                },
            },
            ParameterSuggestion {
                display_name: "Protocol".to_string(),
                parameter_type: ParameterType::Categorical,
                search_terms: vec!["protocol".to_string(), "type".to_string()],
                default_value: ParameterValue::Categorical { 
                    value: "TCP".to_string(), 
                    options: vec!["HTTP".to_string(), "TCP".to_string(), "UDP".to_string(), "WebSocket".to_string()] 
                },
            },
            ParameterSuggestion {
                display_name: "Priority".to_string(),
                parameter_type: ParameterType::Ordinal,
                search_terms: vec!["priority".to_string(), "importance".to_string()],
                default_value: ParameterValue::Ordinal { 
                    level: "Medium".to_string(), 
                    options: vec!["Critical".to_string(), "High".to_string(), "Medium".to_string(), "Low".to_string()] 
                },
            },
            ParameterSuggestion {
                display_name: "Encrypted".to_string(),
                parameter_type: ParameterType::Boolean,
                search_terms: vec!["encrypted".to_string(), "secure".to_string(), "protected".to_string()],
                default_value: ParameterValue::Boolean { 
                    value: false, 
                    true_label: "Encrypted".to_string(), 
                    false_label: "Unencrypted".to_string() 
                },
            },
        ]);

        Self { suggestions }
    }

    /// Get all parameter suggestions for a substance type
    pub fn get_suggestions(&self, substance_type: &SubstanceType) -> Vec<&ParameterSuggestion> {
        self.suggestions
            .get(substance_type)
            .map(|suggestions| suggestions.iter().collect())
            .unwrap_or_default()
    }

    /// Search parameter suggestions with fuzzy matching
    pub fn search_suggestions(&self, substance_type: &SubstanceType, query: &str) -> Vec<&ParameterSuggestion> {
        if query.is_empty() {
            return self.get_suggestions(substance_type);
        }

        let query_lower = query.to_lowercase();
        
        self.get_suggestions(substance_type)
            .into_iter()
            .filter(|suggestion| {
                // Check if query matches any search terms
                suggestion.search_terms.iter().any(|term| {
                    term.to_lowercase().contains(&query_lower)
                }) ||
                // Check if query matches display name
                suggestion.display_name.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

impl Default for SmartParameterDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartParameter {
    /// Create new smart parameter with generated ID
    pub fn new(name: String, value: ParameterValue) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            value,
        }
    }

    /// Get parameter type from value
    pub fn parameter_type(&self) -> ParameterType {
        match &self.value {
            ParameterValue::Numeric { .. } => ParameterType::Numeric,
            ParameterValue::Ordinal { .. } => ParameterType::Ordinal,
            ParameterValue::Categorical { .. } => ParameterType::Categorical,
            ParameterValue::Boolean { .. } => ParameterType::Boolean,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = SmartParameterDatabase::new();
        assert!(!db.suggestions.is_empty());
        
        // Test all substance types have suggestions
        assert!(db.suggestions.contains_key(&SubstanceType::Energy));
        assert!(db.suggestions.contains_key(&SubstanceType::Material));
        assert!(db.suggestions.contains_key(&SubstanceType::Message));
    }

    #[test]
    fn test_search_suggestions() {
        let db = SmartParameterDatabase::new();
        
        // Test temperature search in energy context
        let temp_results = db.search_suggestions(&SubstanceType::Energy, "temp");
        assert!(!temp_results.is_empty());
        assert_eq!(temp_results[0].display_name, "Temperature");
        
        // Test power search in energy context
        let power_results = db.search_suggestions(&SubstanceType::Energy, "pow");
        assert!(!power_results.is_empty());
        assert_eq!(power_results[0].display_name, "Power");
        
        // Test flow search in material context
        let flow_results = db.search_suggestions(&SubstanceType::Material, "flow");
        assert!(!flow_results.is_empty());
        assert_eq!(flow_results[0].display_name, "Flow Rate");
    }

    #[test]
    fn test_smart_parameter_creation() {
        let param = SmartParameter::new(
            "Temperature".to_string(),
            ParameterValue::Numeric { 
                value: "25".to_string(), 
                unit: "°C".to_string() 
            },
        );
        
        assert_eq!(param.name, "Temperature");
        assert_eq!(param.parameter_type(), ParameterType::Numeric);
        assert!(!param.id.is_nil());
    }
}