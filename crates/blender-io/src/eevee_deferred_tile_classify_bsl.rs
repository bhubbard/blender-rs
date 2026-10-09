//! Auto-transpiled C/C++ header module: eevee_deferred_tile_classify_bsl

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TileSubpassIn {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TileClassification {
    pub use_stencil_ref_out: bool,
    pub current_bit: i32,
}
