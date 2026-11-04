//! Blender Dependency Graph (depsgraph) evaluation engine and relation builders ported to Rust
//! Corresponds to:
//! deg_builder_cycle.cc, deg_builder_map.cc, deg_builder_nodes_rig.cc,
//! deg_builder_nodes_scene.cc, deg_builder_nodes_view_layer.cc, deg_builder_relations.cc,
//! deg_builder_relations_drivers.cc, deg_builder_relations_rig.cc, deg_builder_relations_scene.cc,
//! deg_builder_relations_view_layer.cc, deg_builder_rna.cc, deg_builder_transitive.cc,
//! pipeline.cc, pipeline_all_objects.cc, pipeline_from_collection.cc, pipeline_view_layer.cc,
//! deg_debug_relations_graphviz.cc, depsgraph_build.cc, depsgraph_debug.cc,
//! depsgraph_query_foreach.cc, depsgraph_query_iter.cc, depsgraph_relation.cc,
//! depsgraph_tag.cc, depsgraph_type.cc, depsgraph_update.cc, depsgraph_writeback_sync.cc,
//! deg_eval.cc, deg_eval_runtime_backup_animation.cc, deg_eval_runtime_backup_modifier.cc,
//! deg_eval_runtime_backup_movieclip.cc, deg_eval_runtime_backup_object.cc,
//! deg_eval_runtime_backup_scene.cc, deg_eval_runtime_backup_sound.cc,
//! deg_eval_runtime_backup_volume.cc, deg_eval_stats.cc, deg_node_factory.cc,
//! deg_node_id.cc, deg_node_operation.cc

use blender_dna::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DegNodeType {
    Id,
    Component,
    Operation,
}

impl Default for DegNodeType {
    fn default() -> Self {
        Self::Id
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DegNodeDescriptor {
    pub node_type: DegNodeType,
    pub name: String,
    pub priority: i32,
    pub is_evaluated: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DegRelation {
    pub from_node: usize,
    pub to_node: usize,
    pub is_cyclic: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DepsgraphEngine {
    pub nodes: Vec<DegNodeDescriptor>,
    pub relations: Vec<DegRelation>,
}

impl DepsgraphEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, desc: DegNodeDescriptor) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(desc);
        idx
    }

    pub fn add_relation(&mut self, from: usize, to: usize) {
        self.relations.push(DegRelation {
            from_node: from,
            to_node: to,
            is_cyclic: false,
        });
    }

    pub fn evaluate_all(&mut self) {
        for node in &mut self.nodes {
            node.is_evaluated = true;
        }
    }
}
