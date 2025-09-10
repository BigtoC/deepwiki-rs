pub mod ai_analysis_types;
pub mod ai_component_type_analyzer;
pub mod ai_documentation_types;
pub mod ai_research_types;
pub mod component_extractor;
pub mod component_types;
pub mod language_processors;
pub mod research_extractor;
pub mod structure_extractor;

pub use ai_analysis_types::*;
pub use ai_research_types::*;
pub use component_extractor::{ComponentAnalysis, ComponentExtractor};
pub use component_types::ComponentType;
pub use language_processors::LanguageProcessorManager;
pub use research_extractor::{ResearchExtractor, ResearchReport};
pub use structure_extractor::{CoreComponent, ProjectStructure, StructureExtractor};
