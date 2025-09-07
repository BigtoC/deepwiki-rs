pub mod structure_extractor;
pub mod component_extractor;
pub mod research_extractor;
pub mod documentation_extractor;
pub mod language_processors;

pub use structure_extractor::{StructureExtractor, ProjectStructure, CoreComponent};
pub use component_extractor::{ComponentExtractor, ComponentAnalysis};
pub use research_extractor::{ResearchExtractor, ResearchReport};
pub use documentation_extractor::{DocumentationExtractor, C4Documentation};
pub use language_processors::LanguageProcessorManager;