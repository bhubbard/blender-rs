//! Auto-transpiled C/C++ header module: DNA_gpencil_legacy_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDspoint {
    pub x: f32,
    pub pressure: f32,
    pub strength: f32,
    pub time: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDtriangle {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDpalettecolor {
    pub info: [i8; 64],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDpalette {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDcurve_point {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDcurve {
    pub tot_curve_points: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDstroke_Runtime {
    pub tmp_layerinfo: [i8; 128],
    pub multi_frame_falloff: f32,
    pub stroke_start: i32,
    pub fill_start: i32,
    pub vertex_start: i32,
    pub curve_start: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDstroke {
    pub totpoints: i32,
    pub tot_triangles: i32,
    pub thickness: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDframe_Runtime {
    pub frameid: i32,
    pub onion_id: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDframe {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDlayer_Mask {
    pub name: [i8; 128],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDlayer_Runtime {
    pub icon_id: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPDlayer {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPdata_Runtime {
    pub playing: i16,
    pub matid: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPgrid {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bGPdata {
    pub id: ID,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDspoint_Flag {
    GP_SPOINT_SELECT = (1 << 0),
    GP_SPOINT_TAG = (1 << 1),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDpalettecolor_Flag {
    PC_COLOR_HIDE = (1 << 1),
    PC_COLOR_LOCKED = (1 << 2),
    PC_COLOR_ONIONSKIN = (1 << 3),
    PC_COLOR_VOLUMETRIC = (1 << 4),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDpalette_Flag {
    PL_PALETTE_ACTIVE = (1 << 0),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDcurve_point_Flag {
    GP_CURVE_POINT_SELECT = (1 << 0),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum bGPDcurve_Flag {
    GP_CURVE_NEEDS_STROKE_UPDATE = (1 << 0),
    GP_CURVE_SELECT = (1 << 1),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDstroke_Flag {
    GP_STROKE_3DSPACE = (1 << 0),
    GP_STROKE_2DSPACE = (1 << 1),
    GP_STROKE_2DIMAGE = (1 << 2),
    GP_STROKE_SELECT = (1 << 3),
    GP_STROKE_CYCLIC = (1 << 7),
    GP_STROKE_NOFILL = (1 << 8),
    GP_STROKE_NEEDS_CURVE_UPDATE = (1 << 9),
    GP_STROKE_HELP = (1 << 10),
    GP_STROKE_COLLIDE = (1 << 11),
    GP_STROKE_USE_ARROW_START = (1 << 12),
    GP_STROKE_USE_ARROW_END = (1 << 13),
    GP_STROKE_TAG = (1 << 14),
    GP_STROKE_ERASER = static_cast<short>(1 << 15),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDstroke_Caps {
    GP_STROKE_CAP_ROUND = 0,
    GP_STROKE_CAP_FLAT = 1,
    GP_STROKE_CAP_MAX,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDstroke_Arrowstyle {
    GP_STROKE_ARROWSTYLE_NONE = 0,
    GP_STROKE_ARROWSTYLE_SEGMENT = 2,
    GP_STROKE_ARROWSTYLE_OPEN = 3,
    GP_STROKE_ARROWSTYLE_CLOSED = 4,
    GP_STROKE_ARROWSTYLE_SQUARE = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDframe_Flag {
    GP_FRAME_PAINT = (1 << 0),
    GP_FRAME_SELECT = (1 << 1),
    GP_FRAME_LRT_CLEARED = (1 << 2),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ebGPDlayer_Mask_Flag {
    GP_MASK_HIDE = (1 << 0),
    GP_MASK_INVERT = (1 << 1),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDlayer_Flag {
    GP_LAYER_HIDE = (1 << 0),
    GP_LAYER_LOCKED = (1 << 1),
    GP_LAYER_ACTIVE = (1 << 2),
    GP_LAYER_DRAWDEBUG = (1 << 3),
    GP_LAYER_SOLO_MODE = (1 << 4),
    GP_LAYER_SELECT = (1 << 5),
    GP_LAYER_FRAMELOCK = (1 << 6),
    GP_LAYER_NO_XRAY = (1 << 7),
    GP_LAYER_VOLUMETRIC = (1 << 10),
    GP_LAYER_USE_LIGHTS = (1 << 11),
    GP_LAYER_UNLOCK_COLOR = (1 << 12),
    GP_LAYER_USE_MASK = (1 << 13), /* TODO: DEPRECATED */,
    GP_LAYER_IS_RULER = (1 << 14),
    GP_LAYER_DISABLE_MASKS_IN_VIEWLAYER = static_cast<short>(1 << 15),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPDlayer_OnionFlag {
    GP_LAYER_ONIONSKIN = (1 << 0),
    GP_LAYER_ONIONSKIN_CUSTOM_COLOR = (1 << 1),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPLayerBlendModes {
    eGplBlendMode_Regular = 0,
    eGplBlendMode_HardLight = 1,
    eGplBlendMode_Add = 2,
    eGplBlendMode_Subtract = 3,
    eGplBlendMode_Multiply = 4,
    eGplBlendMode_Divide = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPdata_Flag {
    GP_DATA_ANNOTATIONS = (1 << 0),
    GP_DATA_DISPINFO = (1 << 1),
    GP_DATA_EXPAND = (1 << 2),
    GP_DATA_VIEWALIGN = (1 << 4),
    GP_DATA_DEPTH_VIEW = (1 << 5),
    GP_DATA_DEPTH_STROKE = (1 << 6),
    GP_DATA_DEPTH_STROKE_ENDPOINTS = (1 << 7),
    GP_DATA_STROKE_EDITMODE = (1 << 8),
    GP_DATA_SHOW_ONIONSKINS = (1 << 9),
    GP_DATA_CACHE_IS_DIRTY = (1 << 11),
    GP_DATA_STROKE_PAINTMODE = (1 << 12),
    GP_DATA_STROKE_SCULPTMODE = (1 << 13),
    GP_DATA_STROKE_WEIGHTMODE = (1 << 14),
    GP_DATA_STROKE_KEEPTHICKNESS = (1 << 15),
    GP_DATA_STROKE_MULTIEDIT = (1 << 16),
    GP_DATA_STROKE_VERTEXMODE = (1 << 18),
    GP_DATA_AUTOLOCK_LAYERS = (1 << 20),
    GP_DATA_CURVE_EDIT_MODE = (1 << 21),
    GP_DATA_CURVE_ADAPTIVE_RESOLUTION = (1 << 22),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGPD_OnionFlag {
    GP_ONION_GHOST_PREVCOL = (1 << 0),
    GP_ONION_GHOST_NEXTCOL = (1 << 1),
    GP_ONION_GHOST_ALWAYS = (1 << 2),
    GP_ONION_FADE = (1 << 3),
    GP_ONION_LOOP = (1 << 4),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_OnionModes {
    GP_ONION_MODE_ABSOLUTE = 0,
    GP_ONION_MODE_RELATIVE = 1,
    GP_ONION_MODE_SELECTED = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eGP_DrawMode {
    GP_DRAWMODE_2D = 0,
    GP_DRAWMODE_3D = 1,
}
