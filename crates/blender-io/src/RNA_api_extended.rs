//! Blender RNA API bindings and introspection mechanisms ported to Rust
//! Corresponds to:
//! rna_action.cc, rna_action_api.cc, rna_blendfile_import.cc, rna_camera_api.cc,
//! rna_color.cc, rna_depsgraph.cc, rna_image_api.cc, rna_layer.cc, rna_lightprobe.cc,
//! rna_material.cc, rna_material_api.cc, rna_mesh_api.cc, rna_meta.cc, rna_meta_api.cc,
//! rna_movieclip.cc, rna_nla.cc, rna_node_socket.cc, rna_node_tree_interface.cc,
//! rna_nodetree.cc, rna_object.cc, rna_object_api.cc, rna_object_force.cc,
//! rna_packedfile.cc, rna_palette.cc, rna_particle.cc, rna_path.cc,
//! rna_pointcloud.cc, rna_pose.cc

use blender_dna::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RnaFunctionDef {
    pub identifier: String,
    pub description: String,
    pub call_fn: Option<fn(ptr: &PointerRNA, args: &[u8]) -> Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RnaPropertyBinding {
    pub struct_name: String,
    pub property_name: String,
    pub type_code: i32,
    pub is_readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RnaRegistry {
    pub bindings: Vec<RnaPropertyBinding>,
    pub functions: Vec<RnaFunctionDef>,
}

impl RnaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_property(&mut self, binding: RnaPropertyBinding) {
        self.bindings.push(binding);
    }

    pub fn register_function(&mut self, func: RnaFunctionDef) {
        self.functions.push(func);
    }

    pub fn find_property(&self, struct_name: &str, prop_name: &str) -> Option<&RnaPropertyBinding> {
        self.bindings.iter().find(|b| b.struct_name == struct_name && b.property_name == prop_name)
    }
}
