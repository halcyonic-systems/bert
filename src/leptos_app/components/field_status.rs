use leptos::prelude::*;
use leptos::context::Provider;
use leptos::{component, view, IntoView};

// Global context for research fields toggle
#[derive(Clone, Copy)]
pub struct ResearchFieldsContext {
    pub show: RwSignal<bool>,
}

impl ResearchFieldsContext {
    pub fn new() -> Self {
        Self {
            show: RwSignal::new(false),
        }
    }
}

#[component]
pub fn ResearchFieldProvider(children: Children) -> impl IntoView {
    let context = ResearchFieldsContext::new();
    
    view! {
        <Provider value=context>
            {children()}
        </Provider>
    }
}

#[component]
pub fn ResearchField(
    children: Children,
) -> impl IntoView
{
    let context = expect_context::<ResearchFieldsContext>();
    
    view! {
        <div 
            class="research-field-wrapper relative"
            class:hidden=move || !context.show.get()
        >
            <div class="flex items-center gap-2 mb-1">
                <span class="text-amber-600 text-xs font-medium bg-amber-100 px-2 py-1 rounded flex items-center gap-1">
                    "! Research Field"
                    <div class="group relative">
                        <svg class="w-3 h-3 text-amber-600 cursor-help" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path>
                        </svg>
                        <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-gray-900 text-white text-xs rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none whitespace-nowrap z-50">
                            "This field is under active development and may change in future versions"
                            <div class="absolute top-full left-1/2 transform -translate-x-1/2 border-4 border-transparent border-t-gray-900"></div>
                        </div>
                    </div>
                </span>
                <span class="text-xs text-gray-500">
                    "Under development - may change"
                </span>
            </div>
            <div class="research-field-content border-2 border-dashed border-amber-300 bg-amber-50 rounded-lg p-3">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn CoreField(
    children: Children,
) -> impl IntoView
{
    view! {
        <div class="core-field-wrapper">
            {children()}
        </div>
    }
}

#[component]
pub fn ResearchFieldToggle() -> impl IntoView {
    let context = expect_context::<ResearchFieldsContext>();
    
    view! {
        <div class="research-toggle-wrapper mb-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
            <div class="flex items-center gap-3">
                <input 
                    type="checkbox"
                    id="show-research-fields"
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 focus:ring-2"
                    prop:checked=move || context.show.get()
                    on:change=move |ev| {
                        context.show.set(event_target_checked(&ev));
                    }
                />
                <label for="show-research-fields" class="text-sm font-medium text-gray-900 cursor-pointer">
                    "Show Research Fields"
                </label>
                <div class="group relative">
                    <svg class="w-4 h-4 text-blue-600 cursor-help" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path>
                    </svg>
                    <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-3 py-2 bg-gray-900 text-white text-xs rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none whitespace-nowrap z-50">
                        "Toggle to show/hide experimental fields that are still under development"
                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 border-4 border-transparent border-t-gray-900"></div>
                    </div>
                </div>
            </div>
            <p class="text-xs text-gray-600 mt-2 ml-7">
                "Research fields may change or be removed in future versions. Use for experimentation only."
            </p>
        </div>
    }
}

// Field help metadata system
#[derive(Clone, Debug)]
pub struct FieldHelp {
    pub title: String,
    pub description: String,
    pub examples: Vec<String>,
    pub when_to_use: String,
    pub field_type: FieldType,
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Core,
    Research,
}

impl FieldHelp {
    pub fn new_core(title: &str, description: &str, examples: Vec<&str>, when_to_use: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            examples: examples.iter().map(|s| s.to_string()).collect(),
            when_to_use: when_to_use.to_string(),
            field_type: FieldType::Core,
        }
    }

    pub fn new_research(title: &str, description: &str, examples: Vec<&str>, when_to_use: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            examples: examples.iter().map(|s| s.to_string()).collect(),
            when_to_use: when_to_use.to_string(),
            field_type: FieldType::Research,
        }
    }
}

// Field help database
pub fn get_field_help(field_id: &str) -> Option<FieldHelp> {
    match field_id {
        // System Core Fields
        "system-name" => Some(FieldHelp::new_core(
            "System Name",
            "The primary identifier for your system. Use clear, descriptive names that communicate the system's purpose.",
            vec!["Payment Processing System", "User Authentication", "Data Analytics Pipeline"],
            "Always required. Choose names that stakeholders will immediately understand."
        )),
        "system-description" => Some(FieldHelp::new_core(
            "System Description", 
            "A detailed explanation of what the system does, its purpose, and key characteristics.",
            vec!["Processes customer payments securely", "Manages user login and access control"],
            "Essential for documentation and stakeholder communication. Be specific about system behavior."
        )),
        "complexity" => Some(FieldHelp::new_core(
            "Complexity",
            "Indicates the system's structural and behavioral complexity. Adaptable systems can change their structure, evolveable systems can develop new capabilities.",
            vec!["Simple systems: Adaptable=false, Evolveable=false", "Organizations: Both true"],
            "Helps understand system behavior and change potential. Critical for planning interventions."
        )),

        // System Research Fields  
        "system-equivalence" => Some(FieldHelp::new_research(
            "Equivalence",
            "Mathematical representation of system behavior. Captures functional relationships and invariants.",
            vec!["f(x) = mx + b for linear systems", "Conservation equations"],
            "When you need formal analysis or want to model system mathematically. Advanced users."
        )),
        "system-time-unit" => Some(FieldHelp::new_research(
            "Time Unit",
            "The fundamental time scale at which the system operates or is measured.",
            vec!["seconds", "days", "quarters", "milliseconds"],
            "For temporal analysis and simulation. Specify when system behavior has time-dependent characteristics."
        )),
        "system-history" => Some(FieldHelp::new_research(
            "History",
            "Records of past states, behaviors, and changes. Enables path-dependent analysis.",
            vec!["Previous configurations", "Change log", "Performance trends"],
            "When system behavior depends on past states or you need to track evolution over time."
        )),
        "transformation" => Some(FieldHelp::new_research(
            "Transformation",
            "Rules or functions that describe how the system changes inputs to outputs or states.",
            vec!["Data processing rules", "State transition functions"],
            "For formal system modeling and when precise transformation logic is important."
        )),

        // Boundary Research Fields
        "boundary-name" => Some(FieldHelp::new_research(
            "Boundary Name",
            "Identifier for the system boundary. Useful when analyzing multiple boundary definitions.",
            vec!["Legal Boundary", "Operational Boundary", "Physical Boundary"],
            "When working with complex systems having multiple boundary interpretations."
        )),
        "boundary-description" => Some(FieldHelp::new_research(
            "Boundary Description", 
            "Detailed explanation of what constitutes the system boundary and boundary conditions.",
            vec!["Includes all employees", "Excludes external contractors"],
            "For precise boundary definition in complex systems or formal analysis."
        )),

        // Environment Research Fields
        "environment-name" => Some(FieldHelp::new_research(
            "Environment Name",
            "Identifier for the system's operating environment.",
            vec!["Market Environment", "Regulatory Environment", "Technical Environment"],
            "When analyzing system-environment interactions or multiple environmental contexts."
        )),
        "environment-description" => Some(FieldHelp::new_research(
            "Environment Description",
            "Characteristics and properties of the environment in which the system operates.",
            vec!["Highly regulated market", "Fast-changing technology landscape"],
            "For environmental analysis and understanding external constraints and opportunities."
        )),

        // Core Boundary Fields
        "boundary-porosity" => Some(FieldHelp::new_core(
            "Boundary Porosity",
            "How permeable the system boundary is. 0 = completely closed, 1 = completely open.",
            vec!["0.1 for secure systems", "0.8 for collaborative systems"],
            "Indicates how easily things cross the boundary. Important for understanding system openness."
        )),
        "boundary-perceptive-fuzziness" => Some(FieldHelp::new_core(
            "Perceptive Fuzziness",
            "How clearly defined the boundary appears to observers. 0 = crystal clear, 1 = very fuzzy.",
            vec!["0.1 for legal entities", "0.7 for cultural movements"],
            "Measures boundary clarity. Higher values indicate disputed or unclear boundaries."
        )),

        // Interaction Core Fields
        "interaction-name" => Some(FieldHelp::new_core(
            "Interaction Name",
            "Descriptive name for this specific interaction or flow between system elements.",
            vec!["Data Transfer", "Payment Flow", "User Request"],
            "Always specify. Helps track and understand system dynamics and relationships."
        )),
        "interaction-description" => Some(FieldHelp::new_core(
            "Interaction Description",
            "Detailed explanation of what this interaction involves and how it works.",
            vec!["User submits form data to server", "System sends payment to vendor"],
            "Essential for understanding system behavior and documenting interactions."
        )),
        "interaction-type" => Some(FieldHelp::new_core(
            "Interaction Type",
            "Category of interaction: Inflow (into system), Outflow (from system), or Internal (within system).",
            vec!["Inflow: Customer orders", "Outflow: Product delivery", "Internal: Data processing"],
            "Critical for understanding system structure and mapping flows correctly."
        )),
        "substance-type" => Some(FieldHelp::new_core(
            "Substance Type",
            "What type of 'stuff' flows in this interaction: Information, Material, Energy, or People.",
            vec!["Information: data, messages", "Material: products, resources", "Energy: power, fuel"],
            "Fundamental classification. Essential for understanding what the system processes."
        )),
        "substance-unit" => Some(FieldHelp::new_core(
            "Substance Unit",
            "Unit of measurement for the substance in this interaction.",
            vec!["GB (for data)", "USD (for money)", "people/hour", "kg/day"],
            "Enables quantitative analysis and measurement of system flows."
        )),
        "substance-amount" => Some(FieldHelp::new_core(
            "Substance Amount",
            "Quantity of substance involved in this interaction, using the specified unit.",
            vec!["100 (if unit is USD)", "50 (if unit is people/day)"],
            "For capacity planning, performance analysis, and quantitative system understanding."
        )),
        "interaction-usability" => Some(FieldHelp::new_core(
            "Interaction Usability",
            "How the interaction serves the system: Resource (input needed), Product (valuable output), Waste (unwanted output), or Disruption (harmful input).",
            vec!["Resource: Raw materials, data inputs", "Product: Services, deliverables", "Waste: Byproducts, emissions", "Disruption: Attacks, errors"],
            "Critical for understanding value flows and system purpose. Helps identify what adds vs. subtracts value."
        )),

        // Interaction Research Fields
        "substance-sub-type" => Some(FieldHelp::new_research(
            "Substance Sub Type",
            "More specific categorization within the main substance type.",
            vec!["Personal Data (under Information)", "Raw Materials (under Material)"],
            "For detailed analysis when the main substance type is too broad."
        )),

        // External Entity Core Fields
        "external-entity-name" => Some(FieldHelp::new_core(
            "External Entity Name",
            "Name of the entity outside your system that interacts with it.",
            vec!["Customer", "Supplier", "Regulatory Agency", "Partner System"],
            "Always required. Clearly identify all external actors and systems."
        )),
        "external-entity-description" => Some(FieldHelp::new_core(
            "External Entity Description",
            "Description of the external entity, its role, and relationship to your system.",
            vec!["Primary customer segment", "Key supplier for raw materials"],
            "Important for stakeholder analysis and understanding system context."
        )),
        "model" => Some(FieldHelp::new_core(
            "Model",
            "Reference to external models or representations of this entity.",
            vec!["Customer persona document", "Vendor profile ID", "API specification"],
            "Links to detailed external documentation or models when available."
        )),

        // External Entity Research Fields
        "equivalence" => Some(FieldHelp::new_research(
            "Equivalence",
            "Mathematical or logical representation of the entity's behavior or characteristics.",
            vec!["Statistical model", "Behavioral equations", "Rule sets"],
            "For formal analysis and when precise modeling of external entities is needed."
        )),

        // SubSystem Research Fields
        "system-complexity" => Some(FieldHelp::new_research(
            "Complexity Type",
            "Detailed complexity classification for subsystems, including multiset membership.",
            vec!["Complex with adaptation", "Multiset with autonomy", "Atomic"],
            "Advanced modeling of subsystem internal structure and behavior."
        )),
        "system-membership" => Some(FieldHelp::new_research(
            "Member Autonomy",
            "For multiset subsystems, indicates how autonomous individual members are (0-1 scale).",
            vec!["0.2 for tightly controlled teams", "0.8 for autonomous agents"],
            "When modeling collections of semi-independent entities within the subsystem."
        )),

        // Interface Core Fields
        "interface-name" => Some(FieldHelp::new_core(
            "Interface Name",
            "Name of the connection point where system elements interact.",
            vec!["API Endpoint", "Physical Port", "Communication Channel"],
            "Always specify. Interfaces are critical connection points in system architecture."
        )),
        "interface-description" => Some(FieldHelp::new_core(
            "Interface Description",
            "Detailed description of how this interface works and what it enables.",
            vec!["REST API for data exchange", "Physical connector for power"],
            "Essential for technical documentation and system integration understanding."
        )),

        _ => None,
    }
}

// Professional tooltip component
#[component]
pub fn FieldTooltip(field_id: String, #[prop(optional, default = true)] show_help: bool, children: Children) -> impl IntoView {
    let (show_tooltip, set_show_tooltip) = signal(false);
    
    let field_help = get_field_help(&field_id);
    
    view! {
        <div class="relative inline-block">
            {children()}
            
            {if show_help {
                field_help.map(|help| view! {
                <button
                    type="button"
                    class="ml-2 inline-flex items-center justify-center w-4 h-4 text-gray-400 hover:text-gray-600 transition-colors duration-200 rounded-full hover:bg-gray-100"
                    on:mouseenter=move |_| set_show_tooltip.set(true)
                    on:mouseleave=move |_| set_show_tooltip.set(false)
                    on:focus=move |_| set_show_tooltip.set(true)
                    on:blur=move |_| set_show_tooltip.set(false)
                >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"/>
                    </svg>
                </button>
                
                <div 
                    class=move || format!("absolute left-6 bottom-full mb-2 z-[100] w-80 p-4 bg-white border border-gray-200 rounded-lg shadow-xl transition-all duration-200 {}", 
                        if show_tooltip.get() { "opacity-100 visible" } else { "opacity-0 invisible" }
                    )
                    style="pointer-events: none;"
                >
                    <div class="space-y-3">
                        <div class="flex items-center space-x-2">
                            <h3 class="font-semibold text-gray-900 text-sm">{help.title.clone()}</h3>
                            {match help.field_type {
                                FieldType::Core => view! {
                                    <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-100 text-blue-800">
                                        Core
                                    </span>
                                }.into_view(),
                                FieldType::Research => view! {
                                    <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-amber-100 text-amber-800">
                                        Research
                                    </span>
                                }.into_view(),
                            }}
                        </div>
                        
                        <p class="text-sm text-gray-700 leading-relaxed">{help.description.clone()}</p>
                        
                        {(!help.examples.is_empty()).then(|| view! {
                            <div>
                                <h4 class="text-xs font-medium text-gray-900 mb-1">Examples:</h4>
                                <ul class="text-xs text-gray-600 space-y-0.5">
                                    {help.examples.iter().map(|example| view! {
                                        <li class="flex items-start">
                                            <span class="text-gray-400 mr-1">{"•"}</span>
                                            <span>{example.clone()}</span>
                                        </li>
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                        })}
                        
                        <div class="pt-2 border-t border-gray-100">
                            <h4 class="text-xs font-medium text-gray-900 mb-1">When to use:</h4>
                            <p class="text-xs text-gray-600 leading-relaxed">{help.when_to_use.clone()}</p>
                        </div>
                    </div>
                    
                    // Tooltip arrow
                    <div class="absolute top-full left-8 transform -translate-x-1/2">
                        <div class="w-2 h-2 bg-white border-r border-b border-gray-200 transform rotate-45"></div>
                    </div>
                </div>
                })
            } else {
                None
            }}
        </div>
    }
} 