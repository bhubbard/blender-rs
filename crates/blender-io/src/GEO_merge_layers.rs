//! Auto-transpiled C/C++ header module: GEO_merge_layers
//! Source: upstream/source/blender/geometry/GEO_merge_layers.hh

use blender_dna::DNA_gpencil_legacy_types::bGPdata;
use blender_dna::BKE_attribute_filter::AttributeFilter;

extern "C" {
    pub fn merge_layers(
        src_grease_pencil: *const bGPdata,
        layers_to_merge_offsets: *const i32,
        layers_to_merge_data: *const i32,
        groups_num: usize,
        attribute_filter: *const AttributeFilter,
    ) -> *mut bGPdata;

    pub fn merge_layers_by_name(
        src_grease_pencil: *const bGPdata,
        selection: *const bool,
        selection_len: usize,
        attribute_filter: *const AttributeFilter,
    ) -> *mut bGPdata;
}
