//! Auto-transpiled C/C++ header module: DNA_freestyle_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreestyleLineSet {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreestyleModuleConfig {
    pub is_displayed: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreestyleConfig {

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleConfig_Flags {
    FREESTYLE_SUGGESTIVE_CONTOURS_FLAG = 1 << 0,
    FREESTYLE_RIDGES_AND_VALLEYS_FLAG = 1 << 1,
    FREESTYLE_MATERIAL_BOUNDARIES_FLAG = 1 << 2,
    FREESTYLE_FACE_SMOOTHNESS_FLAG = 1 << 3,
    FREESTYLE_CULLING = 1 << 5,
    FREESTYLE_VIEW_MAP_CACHE = 1 << 6,
    FREESTYLE_AS_RENDER_PASS = 1 << 7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleControl_Mode {
    FREESTYLE_CONTROL_SCRIPT_MODE = 1,
    FREESTYLE_CONTROL_EDITOR_MODE = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleLineSet_Flags {
    FREESTYLE_LINESET_CURRENT = 1 << 0,
    FREESTYLE_LINESET_ENABLED = 1 << 1,
    FREESTYLE_LINESET_FE_NOT = 1 << 2,
    FREESTYLE_LINESET_FE_AND = 1 << 3,
    FREESTYLE_LINESET_GR_NOT = 1 << 4,
    FREESTYLE_LINESET_FM_NOT = 1 << 5,
    FREESTYLE_LINESET_FM_BOTH = 1 << 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleLineSet_Selection {
    FREESTYLE_SEL_VISIBILITY = 1 << 0,
    FREESTYLE_SEL_EDGE_TYPES = 1 << 1,
    FREESTYLE_SEL_GROUP = 1 << 2,
    FREESTYLE_SEL_IMAGE_BORDER = 1 << 3,
    FREESTYLE_SEL_FACE_MARK = 1 << 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleLineSet_EdgeTypes {
    FREESTYLE_FE_SILHOUETTE = 1 << 0,
    FREESTYLE_FE_BORDER = 1 << 1,
    FREESTYLE_FE_CREASE = 1 << 2,
    FREESTYLE_FE_RIDGE_VALLEY = 1 << 3,
    FREESTYLE_FE_SUGGESTIVE_CONTOUR = 1 << 5,
    FREESTYLE_FE_MATERIAL_BOUNDARY = 1 << 6,
    FREESTYLE_FE_CONTOUR = 1 << 7,
    FREESTYLE_FE_EXTERNAL_CONTOUR = 1 << 8,
    FREESTYLE_FE_EDGE_MARK = 1 << 9,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleLineSet_QI {
    FREESTYLE_QI_VISIBLE = 1,
    FREESTYLE_QI_HIDDEN = 2,
    FREESTYLE_QI_RANGE = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eFreestyleRaycastingAlgorithm {
    FREESTYLE_ALGO_REGULAR = 1,
    FREESTYLE_ALGO_FAST = 2,
    FREESTYLE_ALGO_VERYFAST = 3,
    FREESTYLE_ALGO_CULLED_ADAPTIVE_TRADITIONAL = 4,
    FREESTYLE_ALGO_ADAPTIVE_TRADITIONAL = 5,
    FREESTYLE_ALGO_CULLED_ADAPTIVE_CUMULATIVE = 6,
    FREESTYLE_ALGO_ADAPTIVE_CUMULATIVE = 7,
}
