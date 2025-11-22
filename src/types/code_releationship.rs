use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Streamlined relationship analysis result
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RelationshipAnalysis {
    /// Core dependency relationships (only keep important ones)
    pub core_dependencies: Vec<CoreDependency>,

    /// Architecture layer information
    pub architecture_layers: Vec<ArchitectureLayer>,

    /// Key issues and recommendations
    pub key_insights: Vec<String>,
}

/// Core dependency (simplified version)
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CoreDependency {
    /// Source component
    pub from: String,

    /// Target component
    pub to: String,

    /// Dependency type
    pub dependency_type: DependencyType,

    /// Importance score (1-5, only keep important ones)
    pub importance: u8,

    /// Brief description
    pub description: Option<String>,
}

/// Architecture layer
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ArchitectureLayer {
    /// Layer name
    pub name: String,

    /// Components in this layer
    pub components: Vec<String>,

    /// Layer level (smaller number means lower level)
    pub level: u8,
}

/// Dependency type enumeration
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum DependencyType {
    /// Import dependency (use, import statements)
    Import,
    /// Function call dependency
    FunctionCall,
    /// Inheritance relationship
    Inheritance,
    /// Composition relationship
    Composition,
    /// Data flow dependency
    DataFlow,
    /// Module dependency
    Module,
}

impl DependencyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DependencyType::Import => "import",
            DependencyType::FunctionCall => "function_call",
            DependencyType::Inheritance => "inheritance",
            DependencyType::Composition => "composition",
            DependencyType::DataFlow => "data_flow",
            DependencyType::Module => "module",
        }
    }
}
