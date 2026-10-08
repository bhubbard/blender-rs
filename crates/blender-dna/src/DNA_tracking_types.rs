//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingDistortionModel(pub i16);

impl TrackingDistortionModel {
    pub const TRACKING_DISTORTION_MODEL_POLYNOMIAL: Self = Self((0) as i16);
    pub const TRACKING_DISTORTION_MODEL_DIVISION: Self = Self((1) as i16);
    pub const TRACKING_DISTORTION_MODEL_NUKE: Self = Self((2) as i16);
    pub const TRACKING_DISTORTION_MODEL_BROWN: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingCameraUnits(pub i16);

impl TrackingCameraUnits {
    pub const CAMERA_UNITS_PX: Self = Self((0) as i16);
    pub const CAMERA_UNITS_MM: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingMarkerFlag(pub i32);

impl TrackingMarkerFlag {
    pub const MARKER_DISABLED: Self = Self(((1 << 0)) as i32);
    pub const MARKER_TRACKED: Self = Self(((1 << 1)) as i32);
    pub const MARKER_GRAPH_SEL_X: Self = Self(((1 << 2)) as i32);
    pub const MARKER_GRAPH_SEL_Y: Self = Self(((1 << 3)) as i32);
    pub const MARKER_GRAPH_SEL: Self = Self(4 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingTrackFlag(pub i32);

impl TrackingTrackFlag {
    pub const TRACK_SELECT: Self = Self(((1 << 0)) as i32);
    pub const TRACK_HAS_BUNDLE: Self = Self(((1 << 1)) as i32);
    pub const TRACK_DISABLE_RED: Self = Self(((1 << 2)) as i32);
    pub const TRACK_DISABLE_GREEN: Self = Self(((1 << 3)) as i32);
    pub const TRACK_DISABLE_BLUE: Self = Self(((1 << 4)) as i32);
    pub const TRACK_HIDDEN: Self = Self(((1 << 5)) as i32);
    pub const TRACK_LOCKED: Self = Self(((1 << 6)) as i32);
    pub const TRACK_CUSTOMCOLOR: Self = Self(((1 << 7)) as i32);
    pub const TRACK_USE_2D_STAB: Self = Self(((1 << 8)) as i32);
    pub const TRACK_PREVIEW_GRAYSCALE: Self = Self(((1 << 9)) as i32);
    pub const TRACK_DOPE_SEL: Self = Self(((1 << 10)) as i32);
    pub const TRACK_PREVIEW_ALPHA: Self = Self(((1 << 11)) as i32);
    pub const TRACK_USE_2D_STAB_ROT: Self = Self(((1 << 12)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingMotionModel(pub i16);

impl TrackingMotionModel {
    pub const TRACK_MOTION_MODEL_TRANSLATION: Self = Self((0) as i16);
    pub const TRACK_MOTION_MODEL_TRANSLATION_ROTATION: Self = Self((1) as i16);
    pub const TRACK_MOTION_MODEL_TRANSLATION_SCALE: Self = Self((2) as i16);
    pub const TRACK_MOTION_MODEL_TRANSLATION_ROTATION_SCALE: Self = Self((3) as i16);
    pub const TRACK_MOTION_MODEL_AFFINE: Self = Self((4) as i16);
    pub const TRACK_MOTION_MODEL_HOMOGRAPHY: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingAlgorithmFlag(pub i32);

impl TrackingAlgorithmFlag {
    pub const TRACK_ALGORITHM_FLAG_USE_BRUTE: Self = Self(((1 << 0)) as i32);
    pub const TRACK_ALGORITHM_FLAG_USE_NORMALIZATION: Self = Self(((1 << 2)) as i32);
    pub const TRACK_ALGORITHM_FLAG_USE_MASK: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTrackFrameMatch(pub i16);

impl eTrackFrameMatch {
    pub const TRACK_MATCH_KEYFRAME: Self = Self((0) as i16);
    pub const TRACK_MATCH_PREVIOUS_FRAME: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingMotionFlag(pub i16);

impl TrackingMotionFlag {
    pub const TRACKING_MOTION_TRIPOD: Self = Self(((1 << 0)) as i16);
    pub const TRACKING_MOTION_MODAL: Self = Self(1 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingSpeed(pub i16);

impl TrackingSpeed {
    pub const TRACKING_SPEED_FASTEST: Self = Self((0) as i16);
    pub const TRACKING_SPEED_REALTIME: Self = Self((1) as i16);
    pub const TRACKING_SPEED_HALF: Self = Self((2) as i16);
    pub const TRACKING_SPEED_QUARTER: Self = Self((4) as i16);
    pub const TRACKING_SPEED_DOUBLE: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingSettingsReconstructionFlag(pub i32);

impl TrackingSettingsReconstructionFlag {
    pub const TRACKING_USE_KEYFRAME_SELECTION: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingRefineCameraFlag(pub i32);

impl TrackingRefineCameraFlag {
    pub const REFINE_NO_INTRINSICS: Self = Self(((0)) as i32);
    pub const REFINE_FOCAL_LENGTH: Self = Self(((1 << 0)) as i32);
    pub const REFINE_PRINCIPAL_POINT: Self = Self(((1 << 1)) as i32);
    pub const REFINE_RADIAL_DISTORTION: Self = Self(((1 << 2)) as i32);
    pub const REFINE_TANGENTIAL_DISTORTION: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingStabilizationFlag(pub i32);

impl TrackingStabilizationFlag {
    pub const TRACKING_2D_STABILIZATION: Self = Self(((1 << 0)) as i32);
    pub const TRACKING_AUTOSCALE: Self = Self(((1 << 1)) as i32);
    pub const TRACKING_STABILIZE_ROTATION: Self = Self(((1 << 2)) as i32);
    pub const TRACKING_STABILIZE_SCALE: Self = Self(((1 << 3)) as i32);
    pub const TRACKING_SHOW_STAB_TRACKS: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingStabilizationFilter(pub i32);

impl TrackingStabilizationFilter {
    pub const TRACKING_FILTER_NEAREST: Self = Self((0) as i32);
    pub const TRACKING_FILTER_BILINEAR: Self = Self((1) as i32);
    pub const TRACKING_FILTER_BICUBIC: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingReconstructionFlag(pub i32);

impl TrackingReconstructionFlag {
    pub const TRACKING_RECONSTRUCTED: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingObjectFlag(pub i32);

impl TrackingObjectFlag {
    pub const TRACKING_OBJECT_CAMERA: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingDopesheetSort(pub i16);

impl TrackingDopesheetSort {
    pub const TRACKING_DOPE_SORT_NAME: Self = Self((0) as i16);
    pub const TRACKING_DOPE_SORT_LONGEST: Self = Self((1) as i16);
    pub const TRACKING_DOPE_SORT_TOTAL: Self = Self((2) as i16);
    pub const TRACKING_DOPE_SORT_AVERAGE_ERROR: Self = Self((3) as i16);
    pub const TRACKING_DOPE_SORT_START: Self = Self((4) as i16);
    pub const TRACKING_DOPE_SORT_END: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingDopesheetFlag(pub i16);

impl TrackingDopesheetFlag {
    pub const TRACKING_DOPE_SORT_INVERSE: Self = Self(((1 << 0)) as i16);
    pub const TRACKING_DOPE_SELECTED_ONLY: Self = Self(((1 << 1)) as i16);
    pub const TRACKING_DOPE_SHOW_HIDDEN: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingCoverage(pub i32);

impl TrackingCoverage {
    pub const TRACKING_COVERAGE_BAD: Self = Self((0) as i32);
    pub const TRACKING_COVERAGE_ACCEPTABLE: Self = Self((1) as i32);
    pub const TRACKING_COVERAGE_OK: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingPlaneMarkerFlag(pub i32);

impl TrackingPlaneMarkerFlag {
    pub const PLANE_MARKER_DISABLED: Self = Self(((1 << 0)) as i32);
    pub const PLANE_MARKER_TRACKED: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TrackingPlaneTrackFlag(pub i32);

impl TrackingPlaneTrackFlag {
    pub const PLANE_TRACK_SELECT: Self = Self(((1 << 0)) as i32);
    pub const PLANE_TRACK_HIDDEN: Self = Self(((1 << 1)) as i32);
    pub const PLANE_TRACK_LOCKED: Self = Self(((1 << 2)) as i32);
    pub const PLANE_TRACK_AUTOKEY: Self = Self(((1 << 3)) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieReconstructedCamera {
    pub framenr: i32,
    pub error: f32,
    pub mat: [[f32; 4]; 4],
}

impl Default for MovieReconstructedCamera {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingCamera {
    pub intrinsics: *mut core::ffi::c_void,
    pub distortion_model: TrackingDistortionModel,
    pub _pad: [u8; 2],
}

impl Default for MovieTrackingCamera {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingMarker {
    pub pos: [f32; 2],
    pub _0: f32,
}

impl Default for MovieTrackingMarker {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingPlaneMarker {
    pub corners: [[f32; 2]; 4],
}

impl Default for MovieTrackingPlaneMarker {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingPlaneTrack {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub point_tracks: *mut core::ffi::c_void,
    pub point_tracksnr: i32,
    pub _pad: [u8; 4],
}

impl Default for MovieTrackingPlaneTrack {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingSettings {
    pub default_motion_model: TrackingMotionModel,
    pub _pad: i16,
}

impl Default for MovieTrackingSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingStabilization {
    pub flag: TrackingStabilizationFlag,
}

impl Default for MovieTrackingStabilization {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingReconstruction {
    pub flag: TrackingReconstructionFlag,
}

impl Default for MovieTrackingReconstruction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingObject {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub flag: TrackingObjectFlag,
}

impl Default for MovieTrackingObject {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingStats {
    pub message: [u8; 256],
}

impl Default for MovieTrackingStats {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingDopesheetChannel {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub track: *mut core::ffi::c_void,
    pub _pad: [u8; 4],
}

impl Default for MovieTrackingDopesheetChannel {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingDopesheetCoverageSegment {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub coverage: TrackingCoverage,
    pub start_frame: i32,
    pub end_frame: i32,
    pub _pad: [u8; 4],
}

impl Default for MovieTrackingDopesheetCoverageSegment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTrackingDopesheet {
    pub ok: i32,
    pub sort_method: TrackingDopesheetSort,
    pub flag: TrackingDopesheetFlag,
}

impl Default for MovieTrackingDopesheet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MovieTracking {
    pub settings: MovieTrackingSettings,
    pub camera: MovieTrackingCamera,
    pub tracks_legacy: ListBaseT<MovieTrackingTrack>,
    pub nullptr: ListBaseT<MovieTrackingTrack>,
}

impl Default for MovieTracking {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

