use super::{Dependency, LanguageProcessor};
use regex::Regex;
use std::path::Path;

pub struct VueProcessor {
    script_regex: Regex,
    import_regex: Regex,
}

impl VueProcessor {
    pub fn new() -> Self {
        Self {
            script_regex: Regex::new(r"<script[^>]*>(.*?)</script>").unwrap(),
            import_regex: Regex::new(r#"^\s*import\s+(?:.*\s+from\s+)?['"]([^'"]+)['"]"#).unwrap(),
        }
    }
    
    fn extract_script_content(&self, content: &str) -> String {
        if let Some(captures) = self.script_regex.captures(content) {
            if let Some(script_content) = captures.get(1) {
                return script_content.as_str().to_string();
            }
        }
        content.to_string()
    }
}

impl LanguageProcessor for VueProcessor {
    fn supported_extensions(&self) -> Vec<&'static str> {
        vec!["vue"]
    }
    
    fn extract_dependencies(&self, content: &str, _file_path: &Path) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        let script_content = self.extract_script_content(content);
        
        for (line_num, line) in script_content.lines().enumerate() {
            if let Some(captures) = self.import_regex.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    let path_str = import_path.as_str();
                    let is_external = !path_str.starts_with('.') && 
                                    !path_str.starts_with('/') && 
                                    !path_str.starts_with("@/");
                    
                    // 特殊标记Vue相关依赖
                    let name = if path_str == "vue" || path_str.starts_with("vue/") {
                        format!("vue::{}", path_str)
                    } else if path_str.ends_with(".vue") {
                        format!("vue_component::{}", path_str)
                    } else {
                        path_str.to_string()
                    };
                    
                    dependencies.push(Dependency {
                        name,
                        path: Some(path_str.to_string()),
                        is_external,
                        line_number: Some(line_num + 1),
                    });
                }
            }
        }
        
        dependencies
    }
    
    fn determine_component_type(&self, file_path: &Path, content: &str) -> String {
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // 检查特殊文件名
        if file_name == "App.vue" {
            return "vue_app".to_string();
        }
        
        if file_name == "index.vue" {
            return "vue_entry".to_string();
        }
        
        if file_name.to_lowercase().contains("page") || 
           file_path.to_string_lossy().contains("/pages/") ||
           file_path.to_string_lossy().contains("/views/") {
            return "vue_page".to_string();
        }
        
        if file_name.to_lowercase().contains("layout") {
            return "vue_layout".to_string();
        }
        
        // 检查内容模式
        if content.contains("<template>") && content.contains("<script>") {
            if content.contains("export default") {
                "vue_component".to_string()
            } else {
                "vue_partial".to_string()
            }
        } else if content.contains("defineComponent") {
            "vue_composition_component".to_string()
        } else if content.contains("<script setup>") {
            "vue_setup_component".to_string()
        } else {
            "vue_file".to_string()
        }
    }
    
    fn is_important_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // Vue模板标签
        if trimmed.starts_with("<template>") || trimmed.starts_with("<script>") ||
           trimmed.starts_with("<style>") || trimmed.starts_with("<script setup>") {
            return true;
        }
        
        // Vue组件定义
        if trimmed.contains("export default") || trimmed.contains("defineComponent") {
            return true;
        }
        
        // Vue Composition API
        if trimmed.contains("ref(") || trimmed.contains("reactive(") ||
           trimmed.contains("computed(") || trimmed.contains("watch(") ||
           trimmed.contains("onMounted") || trimmed.contains("onUnmounted") {
            return true;
        }
        
        // 导入语句
        if trimmed.starts_with("import ") {
            return true;
        }
        
        // Vue指令和事件
        if trimmed.contains("v-if") || trimmed.contains("v-for") ||
           trimmed.contains("v-model") || trimmed.contains("@click") ||
           trimmed.contains(":") && (trimmed.contains("=") || trimmed.contains("\"")) {
            return true;
        }
        
        // 重要注释
        if trimmed.contains("TODO") || trimmed.contains("FIXME") || 
           trimmed.contains("NOTE") || trimmed.contains("HACK") {
            return true;
        }
        
        false
    }
    
    fn language_name(&self) -> &'static str {
        "Vue"
    }
}