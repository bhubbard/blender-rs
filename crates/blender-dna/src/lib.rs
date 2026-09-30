//! Blender DNA data structures mirroring `source/blender/makesdna`.
use bitflags::bitflags;
use blender_math::Vec3;
bitflags! {
/// Element flags mirroring Blender's `BM_ELEM_*` and mesh selection flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MeshElemFlags: u8 {
const SELECT = 1 << 0;
const HIDE   = 1 << 1;
const SEAM   = 1 << 2;
const SHARP  = 1 << 3;
const SMOOTH = 1 << 4;
const TAG    = 1 << 5;
}
}
/// Supported CustomData layer types in Blender meshes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CustomDataType {
PropFloat = 0,
PropInt = 1,
PropString = 2,
PropFloat2 = 3,
PropFloat3 = 4,
MDeformVert = 5,
MDisps = 6,
OrigIndex = 7,
MCol = 8,
MLoopUV = 9,
BWeight = 10,
Crease = 11,
}
/// Mesh header data mirroring DNA `Mesh`.
#[derive(Debug, Clone)]
pub struct DnaMesh {
pub name: String,
pub verts_num: usize,
pub edges_num: usize,
pub faces_num: usize,
pub corners_num: usize,
pub texspace_location: Vec3,
pub texspace_size: Vec3,
}
impl Default for DnaMesh {
fn default() -> Self {
Self {
name: "Mesh".into(),
verts_num: 0,
edges_num: 0,
faces_num: 0,
corners_num: 0,
texspace_location: Vec3::ZERO,
texspace_size: Vec3::ONE,
}
}
}
pub mod gpu_types;
pub mod AS_asset_catalog;
pub mod AS_asset_file_status;
pub mod GEO_mesh_primitive_cylinder_cone;
pub mod GEO_mesh_triangulate;
pub mod NOD_fn_format_string;
pub mod NOD_geo_closure_to_list;
pub mod NOD_geo_combine_list;
pub mod NOD_geo_field_to_grid;
pub mod NOD_geo_field_to_list;
pub mod DNA_attribute_types;
pub mod BKE_bvh;
pub mod BKE_bvhutils;
pub mod DNA_brush_types;
pub mod NOD_geo_index_switch;
pub mod NOD_geo_menu_switch;
pub mod NOD_geo_rasterize_points;
pub mod NOD_geo_repeat;
pub mod NOD_geo_simulation;
pub mod NOD_geo_viewer;
pub mod DNA_defs;
pub mod DNA_documentation;
pub mod DNA_freestyle_types;
pub mod DNA_genfile;
pub mod DNA_gpencil_legacy_types;
pub mod DNA_gpu_types;
pub mod DNA_nla_types;
pub mod DNA_object_enums;
pub mod BKE_cloth;
pub mod BKE_geometry_fields;
pub mod BKE_keyconfig;
pub mod BKE_main;
pub mod ANIM_keyframing;
pub mod BKE_anim_data;
pub mod BKE_annotations;
pub mod BKE_attribute;
pub mod BKE_attribute_filter;
pub mod BKE_bake_data_block_map;
pub mod BKE_bake_geometry_nodes_modifier_pack;




















pub mod DNA_print;


pub mod stl_import_binary_reader;


pub mod GEO_extract_elements;

pub mod GEO_mesh_copy_selection;

pub mod GEO_mesh_primitive_cuboid;

pub mod GEO_mesh_primitive_grid;

pub mod GEO_mesh_primitive_line;

pub mod GEO_mesh_primitive_uv_sphere;

pub mod GEO_mesh_split_edges;

pub mod GEO_randomize;

pub mod mesh_boolean_intern;


pub mod BKE_curve_to_mesh;

pub mod BKE_grease_pencil_modifiers;

pub mod BKE_mball_tessellate;

pub mod BKE_mesh_fair;

pub mod BKE_mesh_iterators;

pub mod ANIM_bonecolor;

pub mod NOD_defaults;


pub mod NOD_geo_bake;

pub mod NOD_geo_bundle;

pub mod NOD_geo_closure;







pub mod RE_bake;

pub mod RE_texture_margin;





























pub mod BKE_mesh_mirror;

pub mod BKE_mesh_remap;

pub mod BKE_mesh_remesh_voxel;

pub mod BKE_mesh_runtime;

pub mod BKE_mesh_tangent;

pub mod BKE_movieclip;
























pub mod BKE_shader_fx;

pub mod BKE_subdiv_deform;

pub mod BKE_subdiv_mesh;
pub mod DNA_ID_enums;
