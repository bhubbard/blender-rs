//! Auto-transpiled C/C++ header module: DNA_object_enums

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eObjectMode {
    OB_MODE_OBJECT = 0,
    OB_MODE_EDIT = 1 << 0,
    OB_MODE_SCULPT = 1 << 1,
    OB_MODE_VERTEX_PAINT = 1 << 2,
    OB_MODE_WEIGHT_PAINT = 1 << 3,
    OB_MODE_TEXTURE_PAINT = 1 << 4,
    OB_MODE_PARTICLE_EDIT = 1 << 5,
    OB_MODE_POSE = 1 << 6,
    OB_MODE_EDIT_GPENCIL_LEGACY = 1 << 7,
    OB_MODE_PAINT_GREASE_PENCIL = 1 << 8,
    OB_MODE_SCULPT_GREASE_PENCIL = 1 << 9,
    OB_MODE_WEIGHT_GREASE_PENCIL = 1 << 10,
    OB_MODE_VERTEX_GREASE_PENCIL = 1 << 11,
    OB_MODE_SCULPT_CURVES = 1 << 12,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eDrawType {
    OB_BOUNDBOX = 1,
    OB_WIRE = 2,
    OB_SOLID = 3,
    OB_MATERIAL = 4,
    OB_TEXTURE = 5,
    OB_RENDER = 6,
}
