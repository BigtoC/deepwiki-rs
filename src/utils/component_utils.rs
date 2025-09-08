use crate::extractors::CoreComponent;
use std::cmp::Ordering;

/// 组件排序工具类
pub struct ComponentSorter;

impl ComponentSorter {
    /// 按重要性分数降序排序组件
    /// 
    /// # Arguments
    /// * `components` - 要排序的组件引用向量
    /// 
    /// # Returns
    /// 按重要性分数降序排列的组件引用向量
    pub fn sort_by_importance<'a>(components: &'a [&'a CoreComponent]) -> Vec<&'a CoreComponent> {
        let mut sorted_components = components.to_vec();
        sorted_components.sort_by(|a, b| {
            b.importance_score
                .partial_cmp(&a.importance_score)
                .unwrap_or(Ordering::Equal)
        });
        sorted_components
    }
    
    /// 按重要性分数降序排序组件并取前N个
    /// 
    /// # Arguments
    /// * `components` - 要排序的组件引用向量
    /// * `limit` - 要取的组件数量
    /// 
    /// # Returns
    /// 按重要性分数降序排列的前N个组件引用向量
    pub fn get_top_n_by_importance<'a>(components: &'a [&'a CoreComponent], limit: usize) -> Vec<&'a CoreComponent> {
        Self::sort_by_importance(components)
            .into_iter()
            .take(limit)
            .collect()
    }
    
    /// 从CoreComponent向量创建引用向量并按重要性排序
    /// 
    /// # Arguments
    /// * `components` - CoreComponent向量的引用
    /// 
    /// # Returns
    /// 按重要性分数降序排列的组件引用向量
    pub fn sort_components_by_importance(components: &[CoreComponent]) -> Vec<&CoreComponent> {
        let mut component_refs: Vec<&CoreComponent> = components.iter().collect();
        component_refs.sort_by(|a, b| {
            b.importance_score
                .partial_cmp(&a.importance_score)
                .unwrap_or(Ordering::Equal)
        });
        component_refs
    }
    
    /// 从CoreComponent向量获取按重要性排序的前N个组件
    /// 
    /// # Arguments
    /// * `components` - CoreComponent向量的引用
    /// * `limit` - 要取的组件数量
    /// 
    /// # Returns
    /// 按重要性分数降序排列的前N个组件引用向量
    pub fn get_top_n_components(components: &[CoreComponent], limit: usize) -> Vec<&CoreComponent> {
        let mut component_refs: Vec<&CoreComponent> = components.iter().collect();
        component_refs.sort_by(|a, b| {
            b.importance_score
                .partial_cmp(&a.importance_score)
                .unwrap_or(Ordering::Equal)
        });
        component_refs.into_iter().take(limit).collect()
    }
    
    /// 过滤并排序组件
    /// 
    /// # Arguments
    /// * `components` - CoreComponent向量的引用
    /// * `min_score` - 最小重要性分数阈值
    /// * `limit` - 要取的组件数量（可选）
    /// 
    /// # Returns
    /// 过滤后按重要性分数降序排列的组件引用向量
    pub fn filter_and_sort_components(
        components: &[CoreComponent], 
        min_score: f64, 
        limit: Option<usize>
    ) -> Vec<&CoreComponent> {
        let mut filtered_components: Vec<&CoreComponent> = components
            .iter()
            .filter(|c| c.importance_score > min_score)
            .collect();
            
        filtered_components.sort_by(|a, b| {
            b.importance_score
                .partial_cmp(&a.importance_score)
                .unwrap_or(Ordering::Equal)
        });
        
        if let Some(limit) = limit {
            filtered_components.into_iter().take(limit).collect()
        } else {
            filtered_components
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractors::{CoreComponent, ComponentType};
    use std::path::PathBuf;

    fn create_test_component(name: &str, importance_score: f64) -> CoreComponent {
        CoreComponent {
            name: name.to_string(),
            file_path: PathBuf::from(format!("{}.rs", name)),
            component_type: ComponentType::Service,
            importance_score,
            dependencies: Vec::new(),
            description: None,
            functions: Vec::new(),
            interfaces: Vec::new(),
        }
    }

    #[test]
    fn test_sort_by_importance() {
        let components = vec![
            create_test_component("low", 0.3),
            create_test_component("high", 0.9),
            create_test_component("medium", 0.6),
        ];
        
        let sorted = ComponentSorter::sort_components_by_importance(&components);
        
        assert_eq!(sorted[0].name, "high");
        assert_eq!(sorted[1].name, "medium");
        assert_eq!(sorted[2].name, "low");
    }

    #[test]
    fn test_get_top_n_components() {
        let components = vec![
            create_test_component("comp1", 0.3),
            create_test_component("comp2", 0.9),
            create_test_component("comp3", 0.6),
            create_test_component("comp4", 0.8),
        ];
        
        let top_2 = ComponentSorter::get_top_n_components(&components, 2);
        
        assert_eq!(top_2.len(), 2);
        assert_eq!(top_2[0].name, "comp2"); // 0.9
        assert_eq!(top_2[1].name, "comp4"); // 0.8
    }

    #[test]
    fn test_filter_and_sort_components() {
        let components = vec![
            create_test_component("low", 0.3),
            create_test_component("high", 0.9),
            create_test_component("medium", 0.6),
            create_test_component("very_high", 0.95),
        ];
        
        let filtered = ComponentSorter::filter_and_sort_components(&components, 0.5, Some(2));
        
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].name, "very_high"); // 0.95
        assert_eq!(filtered[1].name, "high");      // 0.9
    }
}