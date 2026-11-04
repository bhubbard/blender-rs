//! Blender Nodes (Geometry & Composite) node implementations ported to Rust
//! Corresponds to:
//! node_composite_combine_color.cc, node_composite_separate_color.cc,
//! node_geo_curve_topology_points_of_curve.cc, node_geo_edge_paths_to_curves.cc,
//! node_geo_edge_split.cc, node_geo_flip_faces.cc, node_geo_input_instance_bounds.cc,
//! node_geo_input_mesh_edge_neighbors.cc, node_geo_mesh_to_density_grid.cc,
//! node_geo_mesh_to_sdf_grid.cc, node_geo_mesh_topology_vertex_of_corner.cc,
//! node_geo_separate_components.cc, node_geo_string_to_curves.cc,
//! node_geo_tool_set_face_set.cc

use blender_dna::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeCombineColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl NodeCombineColor {
    pub fn evaluate(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeSeparateColor {
    pub color: [f32; 4],
}

impl NodeSeparateColor {
    pub fn evaluate(&self) -> (f32, f32, f32, f32) {
        (self.color[0], self.color[1], self.color[2], self.color[3])
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeGeoEdgeSplit {
    pub split_angle: f32,
    pub use_sharp_edges: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeGeoFlipFaces {
    pub selection: Vec<bool>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeGeoMeshToSdfGrid {
    pub voxel_size: f32,
    pub exterior_band_width: f32,
    pub interior_band_width: f32,
}
