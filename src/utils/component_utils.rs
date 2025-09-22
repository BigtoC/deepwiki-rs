use std::cmp::Ordering;

use crate::types::FeatureModule;

/// 组件排序工具类
pub struct ComponentSorter;

impl ComponentSorter {
    /// 从CoreComponent向量获取按重要性排序的前N个组件
    ///
    /// # Arguments
    /// * `components` - CoreComponent向量的引用
    /// * `limit` - 要取的组件数量
    ///
    /// # Returns
    /// 按重要性分数降序排列的前N个组件引用向量
    pub fn get_top_n_components(components: &[FeatureModule], limit: usize) -> Vec<&FeatureModule> {
        let mut component_refs: Vec<&FeatureModule> = components.iter().collect();
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
        components: &[FeatureModule],
        min_score: f64,
        limit: Option<usize>,
    ) -> Vec<&FeatureModule> {
        let mut filtered_components: Vec<&FeatureModule> = components
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
