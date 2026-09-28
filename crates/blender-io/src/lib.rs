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







pub mod obj_export_file_writer;




pub mod obj_export_mtl;




































































































pub mod eevee_lightprobe_shared;
