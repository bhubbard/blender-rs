//! Auto-transpiled C/C++ header module: GEO_separate_geometry
//! Source: upstream/source/blender/geometry/GEO_separate_geometry.hh

use blender_dna::BKE_attribute_filter::AttributeFilter;
use crate::ABC_alembic::GeometrySet;
use crate::GEO_reorder::AttrDomain;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryNodeDeleteGeometryMode {
    All = 0,
    EdgeFace = 1,
    OnlyFace = 2,
}

extern "C" {
    pub fn separate_geometry(
        geometry_set: *mut GeometrySet,
        domain: AttrDomain,
        mode: GeometryNodeDeleteGeometryMode,
        selection_field: *const core::ffi::c_void,
        attribute_filter: *const AttributeFilter,
        r_is_error: *mut bool,
    );
}
