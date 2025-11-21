use anyhow::Result;

use crate::generator::context::GeneratorContext;
use crate::generator::research::agents::architecture_researcher::ArchitectureResearcher;
use crate::generator::research::agents::boundary_analyzer::BoundaryAnalyzer;
use crate::generator::research::agents::domain_modules_detector::DomainModulesDetector;
use crate::generator::research::agents::key_modules_insight::KeyModulesInsight;
use crate::generator::research::agents::system_context_researcher::SystemContextResearcher;
use crate::generator::research::agents::workflow_researcher::WorkflowResearcher;
use crate::generator::step_forward_agent::StepForwardAgent;

/// Multi-agent research orchestrator
#[derive(Default)]
pub struct ResearchOrchestrator;

impl ResearchOrchestrator {
    /// Execute all agent analysis pipelines
    pub async fn execute_research_pipeline(&self, context: &GeneratorContext) -> Result<()> {
        println!("🚀 Starting Litho Studies Research investigation pipeline...");

        // First layer: Macro analysis (C1)
        self.execute_agent(&SystemContextResearcher, context)
            .await?;

        // Second layer: Meso analysis (C2)
        self.execute_agent(&DomainModulesDetector, context)
            .await?;
        self.execute_agent(&ArchitectureResearcher, context)
            .await?;
        self.execute_agent(&WorkflowResearcher, context)
            .await?;

        // Third layer: Micro analysis (C3-C4)
        self.execute_agent(&KeyModulesInsight, context)
            .await?;

        // Boundary interface analysis
        self.execute_agent(&BoundaryAnalyzer::default(), context)
            .await?;

        println!("✓ Litho Studies Research pipeline execution completed");

        Ok(())
    }

    /// Execute a single agent
    async fn execute_agent<T>(
        &self,
        agent: &T,
        context: &GeneratorContext,
    ) -> Result<()>
    where
        T: StepForwardAgent + Send + Sync,
    {
        // Use localized agent name if available
        let agent_name = if let Some(agent_enum) = agent.agent_type_enum() {
            agent_enum.display_name(&context.config.target_language)
        } else {
            agent.agent_type()
        };
        
        println!("🤖 Executing {} agent analysis...", agent_name);

        agent.execute(context).await?;
        println!("✓ {} analysis completed", agent_name);
        Ok(())
    }
}
