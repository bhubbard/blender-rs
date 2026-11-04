//! Blender BMesh modeling operators, mesh decimation, topology, and path tools ported to Rust
//! Corresponds to:
//! bmesh_callback_generic.cc, bmesh_mesh_debug.cc, bmesh_operators.cc,
//! bmo_circularize.cc, bmo_fill_holes.cc, bmo_mesh_convert.cc, bmo_mirror.cc,
//! bmo_normals.cc, bmo_offset_edgeloops.cc, bmo_planar_faces.cc, bmo_poke.cc,
//! bmo_primitive.cc, bmo_relax_edge_loops.cc, bmo_rotate_edges.cc, bmo_smooth_laplacian.cc,
//! bmo_space_edge_loops_evenly.cc, bmo_split_edges.cc, bmo_subdivide.cc,
//! bmo_subdivide_edgering.cc, bmo_symmetrize.cc, bmo_triangulate.cc, bmo_unsubdivide.cc,
//! bmo_utils.cc, bmo_wireframe.cc, bmesh_beautify.cc, bmesh_bisect_plane.cc,
//! bmesh_boolean.cc, bmesh_decimate_collapse.cc, bmesh_decimate_dissolve.cc,
//! bmesh_decimate_unsubdivide.cc, bmesh_edgenet.cc, bmesh_edgesplit.cc,
//! bmesh_intersect.cc, bmesh_intersect_edges.cc, bmesh_path.cc, bmesh_path_region.cc,
//! bmesh_path_region_uv.cc, bmesh_path_uv.cc, bmesh_separate.cc, bmesh_triangulate.cc,
//! bmesh_wireframe.cc

use blender_dna::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BmoSubdivideMode {
    InnerVertices,
    Fan,
    QuadCorner,
}

impl Default for BmoSubdivideMode {
    fn default() -> Self {
        Self::InnerVertices
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BmoTriangulateParams {
    pub quad_method: i32,
    pub ngon_method: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BmoPokeParams {
    pub offset: f32,
    pub center_mode: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BmoDecimateParams {
    pub ratio: f32,
    pub use_symmetry: bool,
    pub symmetry_axis: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BmoWireframeParams {
    pub thickness: f32,
    pub offset: f32,
    pub use_replace: bool,
    pub use_boundary: bool,
    pub use_even_offset: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BmeshBisectPlaneParams {
    pub plane_co: [f32; 3],
    pub plane_no: [f32; 3],
    pub clear_outer: bool,
    pub clear_inner: bool,
}
