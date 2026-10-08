//! Vertex custom-data interpolation, porting the role of `BM_data_interp_from_verts`
//! and `BM_data_interp_from_verts_weighted` from `bmesh_interp.cc`.
//!
//! Only float layers exist in the kernel today; a layer is interpolated when *every*
//! source vertex carries it (Blender requires matching layers across sources too).

use crate::*;

impl BMesh {
    /// Sets `dst`'s float layers to the weighted average of the `srcs`.
    /// Weights are normalised by their sum; non-positive totals leave `dst` unchanged.
    pub fn vert_data_interp(&mut self, dst: VertHandle, srcs: &[(VertHandle, f32)]) {
        let total: f32 = srcs.iter().map(|&(_, w)| w).sum();
        if srcs.is_empty() || !(total > 0.0) || self.vpool.get(dst).is_none() {
            return;
        }
        // Layer types present on every source.
        let mut types: Vec<CustomDataType> = self
            .vdata
            .float_layers
            .keys()
            .filter(|(i, _)| *i == srcs[0].0.index)
            .map(|&(_, t)| t)
            .collect();
        types.retain(|t| {
            srcs.iter()
                .all(|&(v, _)| self.vdata.float_layers.contains_key(&(v.index, *t)))
        });
        for t in types {
            let value: f32 = srcs
                .iter()
                .map(|&(v, w)| self.vdata.float_layers[&(v.index, t)] * w)
                .sum::<f32>()
                / total;
            self.vdata.float_layers.insert((dst.index, t), value);
        }
    }

    /// Interpolates between two vertices: `t = 0` gives `a`, `t = 1` gives `b`.
    pub fn vert_data_interp_pair(&mut self, dst: VertHandle, a: VertHandle, b: VertHandle, t: f32) {
        let t = t.clamp(0.0, 1.0);
        // A zero weight must still require the layer on that source, so use tiny-safe weights.
        self.vert_data_interp(dst, &[(a, 1.0 - t), (b, t)]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_verts() -> (BMesh, VertHandle, VertHandle) {
        let mut bm = BMesh::new();
        let a = bm.vert_create(Some(Vec3::ZERO), None);
        let b = bm.vert_create(Some(Vec3::X * 4.0), None);
        bm.elem_float_data_set(a, CustomDataType::PropFloat, 0.0);
        bm.elem_float_data_set(b, CustomDataType::PropFloat, 10.0);
        (bm, a, b)
    }

    #[test]
    fn pair_interpolation() {
        let (mut bm, a, b) = two_verts();
        let d = bm.vert_create(None, None);
        bm.vert_data_interp_pair(d, a, b, 0.25);
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), Some(2.5));
        bm.vert_data_interp_pair(d, a, b, 1.0);
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), Some(10.0));
        bm.vert_data_interp_pair(d, a, b, -3.0); // clamped
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), Some(0.0));
    }

    #[test]
    fn weighted_average_normalises_weights() {
        let (mut bm, a, b) = two_verts();
        let d = bm.vert_create(None, None);
        bm.vert_data_interp(d, &[(a, 1.0), (b, 3.0)]);
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), Some(7.5));
        bm.vert_data_interp(d, &[(a, 2.0), (b, 2.0)]);
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), Some(5.0));
    }

    #[test]
    fn degenerate_inputs_leave_data_alone() {
        let (mut bm, a, b) = two_verts();
        let d = bm.vert_create(None, None);
        bm.elem_float_data_set(d, CustomDataType::PropFloat, 42.0);
        bm.vert_data_interp(d, &[]);
        bm.vert_data_interp(d, &[(a, 0.0), (b, 0.0)]);
        bm.vert_data_interp(d, &[(a, f32::NAN)]);
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), Some(42.0));
    }

    #[test]
    fn layer_must_exist_on_all_sources() {
        let (mut bm, a, b) = two_verts();
        let c = bm.vert_create(None, None); // no layer
        let d = bm.vert_create(None, None);
        bm.vert_data_interp(d, &[(a, 1.0), (c, 1.0)]);
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), None);
        bm.vert_data_interp(d, &[(a, 1.0), (b, 1.0)]);
        assert_eq!(bm.elem_float_data_get(d, CustomDataType::PropFloat), Some(5.0));
    }

    #[test]
    fn edge_split_interpolates_custom_data() {
        let (mut bm, a, b) = two_verts();
        let e = bm.edge_create(a, b, None).unwrap();
        let (vn, _) = bm.edge_split(e, 0.25).unwrap();
        assert_eq!(bm.elem_float_data_get(vn, CustomDataType::PropFloat), Some(2.5));
        assert_eq!(bm.vpool.get(vn).unwrap().co, Vec3::X);
    }
}
