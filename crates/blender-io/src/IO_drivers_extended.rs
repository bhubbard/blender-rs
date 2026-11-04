//! Blender IO drivers (PLY, STL, Wavefront OBJ importers and exporters) ported to Rust
//! Corresponds to:
//! IO_ply.cc, ply_export.cc, ply_export_data.cc, ply_export_header.cc,
//! ply_file_buffer.cc, ply_file_buffer_ascii.cc, ply_file_buffer_binary.cc,
//! ply_import.cc, ply_import_buffer.cc, ply_import_data.cc, ply_import_gsplat.cc,
//! ply_import_mesh.cc, IO_stl.cc, stl_export.cc, stl_export_writer.cc,
//! stl_import.cc, stl_import_binary_reader.cc, stl_import_mesh.cc,
//! IO_wavefront_obj.cc, obj_export_file_writer.cc, obj_export_mesh.cc,
//! obj_export_mtl.cc, obj_export_nurbs.cc, obj_exporter.cc, importer_mesh_utils.cc,
//! obj_import_file_reader.cc, obj_import_mesh.cc, obj_import_mtl.cc,
//! obj_import_nurbs.cc, obj_importer.cc

use blender_dna::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlyExportParams {
    pub is_ascii: bool,
    pub export_normals: bool,
    pub export_uvs: bool,
    pub export_colors: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StlExportParams {
    pub is_ascii: bool,
    pub use_scene_unit: bool,
    pub apply_modifiers: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjExportParams {
    pub export_animation: bool,
    pub export_materials: bool,
    pub export_uv: bool,
    pub export_normals: bool,
    pub export_curves_as_nurbs: bool,
}

impl ObjExportParams {
    pub fn new() -> Self {
        Self {
            export_animation: false,
            export_materials: true,
            export_uv: true,
            export_normals: true,
            export_curves_as_nurbs: false,
        }
    }
}
