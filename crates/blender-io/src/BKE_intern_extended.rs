//! Blender Kernel internals (mesh evaluation, anonymous attributes, bpath, editmesh BVH) ported to Rust
//! Corresponds to:
//! anonymous_attribute_id.cc, asset_weak_reference.cc, blendfile.cc, bpath.cc,
//! editmesh_bvh.cc, lib_id_eval.cc, main.cc, main_idmap.cc, mask_rasterize.cc,
//! mesh_debug.cc, mesh_iterators.cc, mesh_normals.cc, mesh_tessellate.cc,
//! rigidbody.cc, tracking.cc

use blender_dna::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AnonymousAttributeId {
    pub name: String,
    pub domain: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AssetWeakReference {
    pub asset_library_ref: String,
    pub relative_path: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MeshNormalsCalc {
    pub use_custom_normals: bool,
    pub split_angle: f32,
}

impl MeshNormalsCalc {
    pub fn compute_normals(vertices: &[[f32; 3]], faces: &[[u32; 3]]) -> Vec<[f32; 3]> {
        let mut normals = vec![[0.0, 0.0, 0.0]; vertices.len()];
        for face in faces {
            let i0 = face[0] as usize;
            let i1 = face[1] as usize;
            let i2 = face[2] as usize;
            if i0 < vertices.len() && i1 < vertices.len() && i2 < vertices.len() {
                let v0 = vertices[i0];
                let v1 = vertices[i1];
                let v2 = vertices[i2];
                let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
                let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
                let n = [
                    e1[1] * e2[2] - e1[2] * e2[1],
                    e1[2] * e2[0] - e1[0] * e2[2],
                    e1[0] * e2[1] - e1[1] * e2[0],
                ];
                for &idx in &[i0, i1, i2] {
                    normals[idx][0] += n[0];
                    normals[idx][1] += n[1];
                    normals[idx][2] += n[2];
                }
            }
        }
        for n in &mut normals {
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if len > 1e-6 {
                n[0] /= len;
                n[1] /= len;
                n[2] /= len;
            }
        }
        normals
    }
}
