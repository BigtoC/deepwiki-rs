use anyhow::Result;
use serde_json;

use crate::config::Config;
use crate::metadata::ProjectMetadata;

/// 生成用户提示，包含项目元数据（将 metadata 转为可读 JSON 并附加格式说明）
pub async fn generate_user_prompt(metadata: &ProjectMetadata, _config: &Config) -> Result<String> {
    Ok(format!(
        include_str!("prompts/project_analyst_user.tpl"),
        &serde_json::to_string_pretty(&metadata.structure)?,
        &serde_json::to_string_pretty(&metadata.core_components)?,
        &serde_json::to_string_pretty(&metadata.dependencies)?
    ))
}
