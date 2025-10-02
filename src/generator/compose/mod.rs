use crate::generator::compose::agents::architecture_editor::ArchitectureEditor;
use crate::generator::compose::agents::boundary_editor::BoundaryEditor;
use crate::generator::compose::agents::key_modules_insight_editor::KeyModulesInsightEditor;
use crate::generator::compose::agents::overview_editor::OverviewEditor;
use crate::generator::compose::agents::workflow_editor::WorkflowEditor;
use crate::generator::context::GeneratorContext;
use crate::generator::outlet::DocTree;
use crate::generator::step_forward_agent::StepForwardAgent;
use anyhow::Result;

mod agents;
pub mod memory;
pub mod types;

/// 文档生成器
#[derive(Default)]
pub struct DocumentationComposer;

impl DocumentationComposer {
    pub async fn execute(&self, context: &GeneratorContext, doc_tree: &mut DocTree) -> Result<()> {
        println!("\n🤖 执行文档生成流程...");

        let overview_editor = OverviewEditor::default();
        overview_editor.execute(context).await?;

        let architecture_editor = ArchitectureEditor::default();
        architecture_editor.execute(context).await?;

        let workflow_editor = WorkflowEditor::default();
        workflow_editor.execute(context).await?;

        let key_modules_insight_editor = KeyModulesInsightEditor::default();
        key_modules_insight_editor
            .execute(context, doc_tree)
            .await?;

        let boundary_editor = BoundaryEditor::default();
        boundary_editor.execute(context).await?;

        Ok(())
    }
}
