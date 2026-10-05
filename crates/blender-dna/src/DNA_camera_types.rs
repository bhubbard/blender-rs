//! Auto-transpiled C/C++ header module: DNA_camera_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CameraStereoSettings {
    pub interocular_distance: f32,
    pub convergence_distance: f32,
    pub convergence_mode: eCamera_Stereo_ConvergenceMode,
    pub pivot: eCamera_Stereo_Pivot,
    pub flag: eCamera_Stereo_Flag,
    pub _pad: [i8; 2],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CameraBGImage {
    pub next: *mut CameraBGImage,
    pub ima: *mut Image,
    pub iuser: ImageUser,
    pub clip: *mut MovieClip,
    pub cuser: MovieClipUser,
    pub offset: [f32; 2],
    pub alpha: f32,
    pub flag: eCamera_BGImage_Flag,
    pub source: eCamera_BGImage_Source,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CameraDOFSettings {
    pub focus_object: *mut Object,
    pub focus_subtarget: [i8; 64],
    pub focus_distance: f32,
    pub aperture_fstop: f32,
    pub aperture_rotation: f32,
    pub aperture_ratio: f32,
    pub aperture_blades: i32,
    pub flag: eCamera_DOF_Flag,
    pub _pad: [i8; 2],
}

impl Default for CameraDOFSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Camera_Runtime {
    pub drw_corners: [[[f32; 2]; 4]; 2],
    pub drw_tria: [[f32; 2]; 2],
    pub drw_depth: [f32; 2],
    pub drw_focusmat: [[f32; 4]; 4],
    pub drw_normalmat: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub id: ID,
    pub adt: *mut AnimData,
    pub r#type: eCamera_Type,
    pub _pad0: [i8; 1],
    pub composition_guide_flags: eCompositionGuideFlags,
    pub flag: eCamera_Flag,
    pub _pad1: [i8; 2],
    pub passepartalpha: f32,
    pub clip_start: f32,
    pub lens: f32,
    pub sensor_x: f32,
    pub shiftx: f32,
    pub dof_distance: f32,
    pub sensor_fit: eCamera_SensorFit,
    pub panorama_type: eCamera_PanoType,
    pub _pad2: [i8; 6],
    pub fisheye_fov: f32,
    pub fisheye_lens: f32,
    pub longitude_min: f32,
    pub fisheye_polynomial_k0: f32,
    pub fisheye_polynomial_k1: f32,
    pub fisheye_polynomial_k2: f32,
    pub fisheye_polynomial_k3: f32,
    pub fisheye_polynomial_k4: f32,
    pub central_cylindrical_range_v_min: f32,
    pub central_cylindrical_range_v_max: f32,
    pub central_cylindrical_radius: f32,
    pub _pad3: f32,
    pub custom_shader: *mut Text,
    pub custom_filepath: [i8; 1024],
    pub custom_bytecode_hash: [i8; 64],
    pub custom_bytecode: *mut i8,
    pub custom_mode: eCamera_CustomMode,
    pub _pad4: i32,
    pub dof_ob: *mut Object,
    pub gpu_dof: GPUDOFSettings,
    pub dof: CameraDOFSettings,
    pub bg_images: ListBaseT<CameraBGImage>,
    pub stereo: CameraStereoSettings,
    pub composition_guide_color: [f32; 4],
    pub runtime: Camera_Runtime,
}

impl Default for Camera {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Image {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageUser {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClip {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovieClipUser {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Text {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GPUDOFSettings {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_Type {
    CAM_PERSP = 0,
    CAM_ORTHO = 1,
    CAM_PANO = 2,
    CAM_CUSTOM = 3,
}

impl Default for eCamera_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_PERSP: i32 = eCamera_Type::CAM_PERSP as i32;
pub const CAM_ORTHO: i32 = eCamera_Type::CAM_ORTHO as i32;
pub const CAM_PANO: i32 = eCamera_Type::CAM_PANO as i32;
pub const CAM_CUSTOM: i32 = eCamera_Type::CAM_CUSTOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_PanoType {
    CAM_PANORAMA_EQUIRECTANGULAR = 0,
    CAM_PANORAMA_FISHEYE_EQUIDISTANT = 1,
    CAM_PANORAMA_FISHEYE_EQUISOLID = 2,
    CAM_PANORAMA_MIRRORBALL = 3,
    CAM_PANORAMA_FISHEYE_LENS_POLYNOMIAL = 4,
    CAM_PANORAMA_EQUIANGULAR_CUBEMAP_FACE = 5,
    CAM_PANORAMA_CENTRAL_CYLINDRICAL = 6,
}

impl Default for eCamera_PanoType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_PANORAMA_EQUIRECTANGULAR: i32 = eCamera_PanoType::CAM_PANORAMA_EQUIRECTANGULAR as i32;
pub const CAM_PANORAMA_FISHEYE_EQUIDISTANT: i32 = eCamera_PanoType::CAM_PANORAMA_FISHEYE_EQUIDISTANT as i32;
pub const CAM_PANORAMA_FISHEYE_EQUISOLID: i32 = eCamera_PanoType::CAM_PANORAMA_FISHEYE_EQUISOLID as i32;
pub const CAM_PANORAMA_MIRRORBALL: i32 = eCamera_PanoType::CAM_PANORAMA_MIRRORBALL as i32;
pub const CAM_PANORAMA_FISHEYE_LENS_POLYNOMIAL: i32 = eCamera_PanoType::CAM_PANORAMA_FISHEYE_LENS_POLYNOMIAL as i32;
pub const CAM_PANORAMA_EQUIANGULAR_CUBEMAP_FACE: i32 = eCamera_PanoType::CAM_PANORAMA_EQUIANGULAR_CUBEMAP_FACE as i32;
pub const CAM_PANORAMA_CENTRAL_CYLINDRICAL: i32 = eCamera_PanoType::CAM_PANORAMA_CENTRAL_CYLINDRICAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_CustomMode {
    CAM_CUSTOM_SHADER_INTERNAL = 0,
    CAM_CUSTOM_SHADER_EXTERNAL = 1,
}

impl Default for eCamera_CustomMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_CUSTOM_SHADER_INTERNAL: i32 = eCamera_CustomMode::CAM_CUSTOM_SHADER_INTERNAL as i32;
pub const CAM_CUSTOM_SHADER_EXTERNAL: i32 = eCamera_CustomMode::CAM_CUSTOM_SHADER_EXTERNAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCompositionGuideFlags {
    COMPOSITION_GUIDES_CENTER = (1 << 0),
    COMPOSITION_GUIDES_CENTER_DIAG = (1 << 1),
    COMPOSITION_GUIDES_THIRDS = (1 << 2),
    COMPOSITION_GUIDES_GOLDEN = (1 << 3),
    COMPOSITION_GUIDES_GOLDEN_TRI_A = (1 << 4),
    COMPOSITION_GUIDES_GOLDEN_TRI_B = (1 << 5),
    COMPOSITION_GUIDES_HARMONY_TRI_A = (1 << 6),
    COMPOSITION_GUIDES_HARMONY_TRI_B = (1 << 7),
}

impl Default for eCompositionGuideFlags {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const COMPOSITION_GUIDES_CENTER: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_CENTER as i32;
pub const COMPOSITION_GUIDES_CENTER_DIAG: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_CENTER_DIAG as i32;
pub const COMPOSITION_GUIDES_THIRDS: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_THIRDS as i32;
pub const COMPOSITION_GUIDES_GOLDEN: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_GOLDEN as i32;
pub const COMPOSITION_GUIDES_GOLDEN_TRI_A: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_GOLDEN_TRI_A as i32;
pub const COMPOSITION_GUIDES_GOLDEN_TRI_B: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_GOLDEN_TRI_B as i32;
pub const COMPOSITION_GUIDES_HARMONY_TRI_A: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_HARMONY_TRI_A as i32;
pub const COMPOSITION_GUIDES_HARMONY_TRI_B: i32 = eCompositionGuideFlags::COMPOSITION_GUIDES_HARMONY_TRI_B as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_Flag {
    CAM_SHOWLIMITS = (1 << 0),
    CAM_SHOWMIST = (1 << 1),
    CAM_SHOWPASSEPARTOUT = (1 << 2),
    CAM_SHOW_SAFE_MARGINS = (1 << 3),
    CAM_SHOWNAME = (1 << 4),
    CAM_ANGLETOGGLE = (1 << 5),
    CAM_DS_EXPAND = (1 << 6),
    CAM_PANORAMA = (1 << 7),
    CAM_SHOWSENSOR = (1 << 8),
    CAM_SHOW_SAFE_CENTER = (1 << 9),
    CAM_SHOW_BG_IMAGE = (1 << 10),
}

impl Default for eCamera_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_SHOWLIMITS: i32 = eCamera_Flag::CAM_SHOWLIMITS as i32;
pub const CAM_SHOWMIST: i32 = eCamera_Flag::CAM_SHOWMIST as i32;
pub const CAM_SHOWPASSEPARTOUT: i32 = eCamera_Flag::CAM_SHOWPASSEPARTOUT as i32;
pub const CAM_SHOW_SAFE_MARGINS: i32 = eCamera_Flag::CAM_SHOW_SAFE_MARGINS as i32;
pub const CAM_SHOWNAME: i32 = eCamera_Flag::CAM_SHOWNAME as i32;
pub const CAM_ANGLETOGGLE: i32 = eCamera_Flag::CAM_ANGLETOGGLE as i32;
pub const CAM_DS_EXPAND: i32 = eCamera_Flag::CAM_DS_EXPAND as i32;
pub const CAM_PANORAMA: i32 = eCamera_Flag::CAM_PANORAMA as i32;
pub const CAM_SHOWSENSOR: i32 = eCamera_Flag::CAM_SHOWSENSOR as i32;
pub const CAM_SHOW_SAFE_CENTER: i32 = eCamera_Flag::CAM_SHOW_SAFE_CENTER as i32;
pub const CAM_SHOW_BG_IMAGE: i32 = eCamera_Flag::CAM_SHOW_BG_IMAGE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_SensorFit {
    CAMERA_SENSOR_FIT_AUTO = 0,
    CAMERA_SENSOR_FIT_HOR = 1,
    CAMERA_SENSOR_FIT_VERT = 2,
}

impl Default for eCamera_SensorFit {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAMERA_SENSOR_FIT_AUTO: i32 = eCamera_SensorFit::CAMERA_SENSOR_FIT_AUTO as i32;
pub const CAMERA_SENSOR_FIT_HOR: i32 = eCamera_SensorFit::CAMERA_SENSOR_FIT_HOR as i32;
pub const CAMERA_SENSOR_FIT_VERT: i32 = eCamera_SensorFit::CAMERA_SENSOR_FIT_VERT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_Stereo_ConvergenceMode {
    CAM_S3D_OFFAXIS = 0,
    CAM_S3D_PARALLEL = 1,
    CAM_S3D_TOE = 2,
}

impl Default for eCamera_Stereo_ConvergenceMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_S3D_OFFAXIS: i32 = eCamera_Stereo_ConvergenceMode::CAM_S3D_OFFAXIS as i32;
pub const CAM_S3D_PARALLEL: i32 = eCamera_Stereo_ConvergenceMode::CAM_S3D_PARALLEL as i32;
pub const CAM_S3D_TOE: i32 = eCamera_Stereo_ConvergenceMode::CAM_S3D_TOE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_Stereo_Pivot {
    CAM_S3D_PIVOT_LEFT = 0,
    CAM_S3D_PIVOT_RIGHT = 1,
    CAM_S3D_PIVOT_CENTER = 2,
}

impl Default for eCamera_Stereo_Pivot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_S3D_PIVOT_LEFT: i32 = eCamera_Stereo_Pivot::CAM_S3D_PIVOT_LEFT as i32;
pub const CAM_S3D_PIVOT_RIGHT: i32 = eCamera_Stereo_Pivot::CAM_S3D_PIVOT_RIGHT as i32;
pub const CAM_S3D_PIVOT_CENTER: i32 = eCamera_Stereo_Pivot::CAM_S3D_PIVOT_CENTER as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_Stereo_Flag {
    CAM_S3D_SPHERICAL = (1 << 0),
    CAM_S3D_POLE_MERGE = (1 << 1),
}

impl Default for eCamera_Stereo_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_S3D_SPHERICAL: i32 = eCamera_Stereo_Flag::CAM_S3D_SPHERICAL as i32;
pub const CAM_S3D_POLE_MERGE: i32 = eCamera_Stereo_Flag::CAM_S3D_POLE_MERGE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_BGImage_Flag {
    CAM_BGIMG_FLAG_EXPANDED = (1 << 1),
    CAM_BGIMG_FLAG_CAMERACLIP = (1 << 2),
    CAM_BGIMG_FLAG_DISABLED = (1 << 3),
    CAM_BGIMG_FLAG_FOREGROUND = (1 << 4),
    CAM_BGIMG_FLAG_CAMERA_ASPECT = (1 << 5),
    CAM_BGIMG_FLAG_CAMERA_CROP = (1 << 6),
    CAM_BGIMG_FLAG_FLIP_X = (1 << 7),
    CAM_BGIMG_FLAG_FLIP_Y = (1 << 8),
    CAM_BGIMG_FLAG_OVERRIDE_LIBRARY_LOCAL = (1 << 9),
}

impl Default for eCamera_BGImage_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_BGIMG_FLAG_EXPANDED: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_EXPANDED as i32;
pub const CAM_BGIMG_FLAG_CAMERACLIP: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_CAMERACLIP as i32;
pub const CAM_BGIMG_FLAG_DISABLED: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_DISABLED as i32;
pub const CAM_BGIMG_FLAG_FOREGROUND: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_FOREGROUND as i32;
pub const CAM_BGIMG_FLAG_CAMERA_ASPECT: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_CAMERA_ASPECT as i32;
pub const CAM_BGIMG_FLAG_CAMERA_CROP: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_CAMERA_CROP as i32;
pub const CAM_BGIMG_FLAG_FLIP_X: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_FLIP_X as i32;
pub const CAM_BGIMG_FLAG_FLIP_Y: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_FLIP_Y as i32;
pub const CAM_BGIMG_FLAG_OVERRIDE_LIBRARY_LOCAL: i32 = eCamera_BGImage_Flag::CAM_BGIMG_FLAG_OVERRIDE_LIBRARY_LOCAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_BGImage_Source {
    CAM_BGIMG_SOURCE_IMAGE = 0,
    CAM_BGIMG_SOURCE_MOVIE = 1,
}

impl Default for eCamera_BGImage_Source {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_BGIMG_SOURCE_IMAGE: i32 = eCamera_BGImage_Source::CAM_BGIMG_SOURCE_IMAGE as i32;
pub const CAM_BGIMG_SOURCE_MOVIE: i32 = eCamera_BGImage_Source::CAM_BGIMG_SOURCE_MOVIE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eCamera_DOF_Flag {
    CAM_DOF_ENABLED = (1 << 0),
}

impl Default for eCamera_DOF_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const CAM_DOF_ENABLED: i32 = eCamera_DOF_Flag::CAM_DOF_ENABLED as i32;

