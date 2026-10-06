//! Fast I/O importers and exporters for Blender meshes mirroring `source/blender/io`.

use blender_bmesh::{BMesh, VertHandle};
use blender_math::Vec3;
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("Failed to parse mesh file: {0}")]
    ParseError(String),
    #[error("BMesh construction error: {0}")]
    BMeshError(#[from] blender_bmesh::BMeshError),
}

/// Exports a BMesh to Wavefront OBJ format string.
pub fn export_obj(bmesh: &BMesh) -> String {
    let mut out = String::new();
    let mut vert_map: HashMap<u32, usize> = HashMap::new();
    let mut v_idx = 1;

    // Export vertices
    for (handle, vert) in bmesh.vpool.iter() {
        let _ = writeln!(out, "v {} {} {}", vert.co.x, vert.co.y, vert.co.z);
        vert_map.insert(handle.index, v_idx);
        v_idx += 1;
    }

    // Export faces
    for (_f_handle, face) in bmesh.fpool.iter() {
        if face.len < 3 {
            continue;
        }
        let mut face_str = String::from("f");
        let mut curr_l = face.l_first;
        for _ in 0..face.len {
            if let Some(loop_elem) = bmesh.lpool.get(curr_l) {
                if let Some(&idx) = vert_map.get(&loop_elem.v.index) {
                    let _ = write!(face_str, " {}", idx);
                }
                curr_l = loop_elem.next;
            }
        }
        let _ = writeln!(out, "{}", face_str);
    }

    out
}

/// Imports a Wavefront OBJ string into a BMesh.
pub fn import_obj(obj_data: &str) -> Result<BMesh, IoError> {
    let mut bmesh = BMesh::new();
    let mut verts: Vec<VertHandle> = Vec::new();

    for line in obj_data.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let tag = match tokens.next() {
            Some(t) => t,
            None => continue,
        };

        match tag {
            "v" => {
                let x: f32 = tokens.next().unwrap_or("0").parse().map_err(|e| IoError::ParseError(format!("{:?}", e)))?;
                let y: f32 = tokens.next().unwrap_or("0").parse().map_err(|e| IoError::ParseError(format!("{:?}", e)))?;
                let z: f32 = tokens.next().unwrap_or("0").parse().map_err(|e| IoError::ParseError(format!("{:?}", e)))?;
                let vh = bmesh.vert_create(Some(Vec3::new(x, y, z)), None);
                verts.push(vh);
            }
            "f" => {
                let mut face_verts = Vec::new();
                for token in tokens {
                    let v_idx_str = token.split('/').next().unwrap_or("0");
                    let v_idx: usize = v_idx_str.parse().map_err(|e| IoError::ParseError(format!("{:?}", e)))?;
                    if v_idx > 0 && v_idx <= verts.len() {
                        face_verts.push(verts[v_idx - 1]);
                    }
                }
                if face_verts.len() >= 3 {
                    bmesh.face_create(&face_verts, None)?;
                }
            }
            _ => {}
        }
    }

    Ok(bmesh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obj_roundtrip() {
        let mut bmesh = BMesh::new();
        let v1 = bmesh.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
        let v2 = bmesh.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
        let v3 = bmesh.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);
        bmesh.face_create(&[v1, v2, v3], None).unwrap();

        let obj_str = export_obj(&bmesh);
        assert!(obj_str.contains("v 0 0 0"));
        assert!(obj_str.contains("f 1 2 3"));

        let imported = import_obj(&obj_str).unwrap();
        assert_eq!(imported.totvert(), 3);
        assert_eq!(imported.totface(), 1);
    }
}






















































































































































































































































pub mod GEO_set_curve_type;


pub mod GEO_uv_parametrizer;





pub mod BKE_autoexec;



pub mod BKE_callbacks;
























pub mod NOD_nested_node_id;


pub mod NOD_partial_eval;


pub mod NOD_socket_items;

pub mod NOD_string_pattern;















pub mod eevee_gbuffer;










pub mod eevee_material_shared;


pub mod eevee_motion_blur_shared;




pub mod wm_gizmo_group;







pub mod wm_gesture;



























pub mod BLI_delaunay_2d;















































































































pub mod eevee_lightprobe_shared;








































pub mod BLI_color_types;






pub mod BLI_csv_parse;












































































pub mod BKE_id_hash;

pub mod BKE_image_save;






















pub mod bake;








































































































































































































pub mod DNA_lineart_types;






pub mod BLI_atomic_disjoint_set;


pub mod ANIM_action;

pub mod AS_asset_file_status;



pub mod blendthumb;

pub mod BLF_api;

pub mod BLF_enums;

pub mod BKE_mesh;








































































































































































































































































































pub mod BLI_concurrent_set;

pub mod BLI_console;

pub mod BLI_dial_2d;

pub mod BLI_dynstr;

pub mod BLI_endian_defines;

pub mod BLI_enum_flags;


pub mod BLI_fixed_string;



pub mod BLI_gsqueue;








pub mod obj_export_nurbs;


pub mod obj_exporter;


pub mod importer_mesh_utils;


pub mod obj_import_file_reader;


pub mod obj_import_mtl;


pub mod obj_import_nurbs;







pub mod ply_export;


pub mod ply_export_data;


pub mod ply_export_header;

pub mod ply_export_load_plydata;














pub mod stl_export;


pub mod stl_export_writer;

pub mod stl_import;








pub mod BKE_asset_edit;

pub mod BKE_blender_copybuffer;

pub mod BKE_blender_project;

pub mod BKE_blender_version;











































































































pub mod BKE_grease_pencil;

pub mod BKE_grease_pencil_legacy_convert;

pub mod BKE_main_idmap;

pub mod BKE_main_namemap;

pub mod BKE_multires;

pub mod BKE_node_enum;

pub mod BKE_node_legacy_types;


pub mod BKE_object_deform;


pub mod ANIM_driver;

pub mod ANIM_versioning;

pub mod ANIM_visualkey;



pub mod action_runtime;

pub mod bone_collections_internal;



pub mod node_composite_util;
















pub mod BLI_heap_simple;

pub mod BLI_index_range;




pub mod BLI_map_slots;



































pub mod BKE_compute_contexts;


pub mod BKE_curveprofile;



pub mod BKE_pose_backup;

pub mod BKE_recents;

pub mod BKE_volume_grid_file_cache;

pub mod BKE_volume_openvdb;

pub mod BKE_volume_render;











pub mod eevee_cryptomatte;


pub mod DNA_curve_enums;


























































pub mod icons;





















pub mod eevee_shader;


pub mod WM_message;

pub mod wm_gizmo_wmapi;




pub mod wm_panel_type;
















































































pub mod wm_dragdrop;


















































































pub mod wm_playanim;








































pub mod IMB_cache;



pub mod IMB_openexr;

pub mod IMB_partial_update;

pub mod IMB_thumbs;


pub mod BLI_array_state;


pub mod BLI_assert;


pub mod BLI_bit_group_vector;

pub mod BLI_bit_ref;

pub mod BLI_bit_span;

pub mod BLI_cache_mutex;


pub mod BLI_dynamic_stack_buffer;

pub mod BLI_function_ref;

pub mod BLI_listbase_wrapper;

pub mod BLI_lazy_threading;

pub mod BLI_index_ranges_builder;





pub mod BLI_concurrent_map;



pub mod BLI_generic_key;













pub mod BLI_math_matrix_types;

pub mod BLI_math_mpq;







pub mod BKE_anonymous_attribute_id;

pub mod BKE_attribute_storage_blend_write;


pub mod BKE_compute_context_cache;

pub mod BKE_compute_context_cache_fwd;
