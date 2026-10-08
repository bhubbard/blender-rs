//! Auto-transpiled C/C++ header module: DNA_xr_types

use crate::*;

pub const XR_MAX_USER_PATH_LENGTH: i32 = 64;
pub const XR_MAX_COMPONENT_PATH_LENGTH: i32 = 192;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct XrSessionSettings {
    pub shading: View3DShading,
    pub base_scale: f32,
    pub _pad: [i8; 3],
    pub base_pose_type: eXRSessionBasePoseType,
    pub base_pose_object: *mut Object,
    pub base_pose_location: [f32; 3],
    pub base_pose_angle: f32,
    pub draw_flags: i8,
    pub controller_draw_style: i8,
    pub viewfinder_enabled: i8,
    pub viewfinder_crosshair_enabled: i8,
    pub viewfinder_hand: i8,
    pub _pad2: [i8; 3],
    pub viewfinder_scale: f32,
    pub viewfinder_passepartout_overscan: f32,
    pub viewfinder_passepartout_opacity: f32,
    pub clip_start: f32,
    pub flag: i32,
    pub object_type_exclude_viewport: i32,
    pub object_type_exclude_select: i32,
    pub fly_speed: f32,
    pub view_scale: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct XrComponentPath {
    pub next: *mut XrComponentPath,
    pub path: [i8; 192],
}

impl Default for XrComponentPath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct XrActionMapBinding {
    pub next: *mut XrActionMapBinding,
    pub name: [i8; 64],
    pub profile: [i8; 256],
    pub component_paths: ListBaseT<XrComponentPath>,
    pub float_threshold: f32,
    pub axis_flag: eXrAxisFlag,
    pub _pad: [i8; 2],
    pub pose_location: [f32; 3],
    pub pose_rotation: [f32; 3],
}

impl Default for XrActionMapBinding {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct XrUserPath {
    pub next: *mut XrUserPath,
    pub path: [i8; 64],
}

impl Default for XrUserPath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct XrActionMapItem {
    pub next: *mut XrActionMapItem,
    pub name: [i8; 64],
    pub r#type: eXrActionType,
    pub _pad: [i8; 7],
    pub user_paths: ListBaseT<XrUserPath>,
    pub op: [i8; 64],
    pub op_properties: *mut IDProperty,
    pub op_properties_ptr: *mut PointerRNA,
    pub op_flag: eXrOpFlag,
    pub action_flag: eXrActionFlag,
    pub haptic_flag: eXrHapticFlag,
    pub pose_flag: eXrPoseFlag,
    pub haptic_name: [i8; 64],
    pub haptic_duration: f32,
    pub haptic_frequency: f32,
    pub haptic_amplitude: f32,
    pub selbinding: i16,
    pub _pad3: [i8; 2],
    pub bindings: ListBaseT<XrActionMapBinding>,
}

impl Default for XrActionMapItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct XrActionMap {
    pub next: *mut XrActionMap,
    pub name: [i8; 64],
    pub items: ListBaseT<XrActionMapItem>,
    pub selitem: i16,
    pub _pad: [i8; 6],
}

impl Default for XrActionMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct shared {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct View3DShading {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PointerRNA {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrSessionFlag {
    XR_SESSION_USE_POSITION_TRACKING = (1 << 0),
    XR_SESSION_USE_ABSOLUTE_TRACKING = (1 << 1),
}

impl Default for eXrSessionFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_SESSION_USE_POSITION_TRACKING: i32 = eXrSessionFlag::XR_SESSION_USE_POSITION_TRACKING as i32;
pub const XR_SESSION_USE_ABSOLUTE_TRACKING: i32 = eXrSessionFlag::XR_SESSION_USE_ABSOLUTE_TRACKING as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXRSessionBasePoseType {
    XR_BASE_POSE_SCENE_CAMERA = 0,
    XR_BASE_POSE_OBJECT = 1,
    XR_BASE_POSE_CUSTOM = 2,
}

impl Default for eXRSessionBasePoseType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_BASE_POSE_SCENE_CAMERA: i32 = eXRSessionBasePoseType::XR_BASE_POSE_SCENE_CAMERA as i32;
pub const XR_BASE_POSE_OBJECT: i32 = eXRSessionBasePoseType::XR_BASE_POSE_OBJECT as i32;
pub const XR_BASE_POSE_CUSTOM: i32 = eXRSessionBasePoseType::XR_BASE_POSE_CUSTOM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrSessionControllerDrawStyle {
    XR_CONTROLLER_DRAW_DARK = 0,
    XR_CONTROLLER_DRAW_LIGHT = 1,
    XR_CONTROLLER_DRAW_DARK_RAY = 2,
    XR_CONTROLLER_DRAW_LIGHT_RAY = 3,
}

impl Default for eXrSessionControllerDrawStyle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_CONTROLLER_DRAW_DARK: i32 = eXrSessionControllerDrawStyle::XR_CONTROLLER_DRAW_DARK as i32;
pub const XR_CONTROLLER_DRAW_LIGHT: i32 = eXrSessionControllerDrawStyle::XR_CONTROLLER_DRAW_LIGHT as i32;
pub const XR_CONTROLLER_DRAW_DARK_RAY: i32 = eXrSessionControllerDrawStyle::XR_CONTROLLER_DRAW_DARK_RAY as i32;
pub const XR_CONTROLLER_DRAW_LIGHT_RAY: i32 = eXrSessionControllerDrawStyle::XR_CONTROLLER_DRAW_LIGHT_RAY as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrActionType {
    XR_BOOLEAN_INPUT = 1,
    XR_FLOAT_INPUT = 2,
    XR_VECTOR2F_INPUT = 3,
    XR_POSE_INPUT = 4,
    XR_VIBRATION_OUTPUT = 100,
}

impl Default for eXrActionType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_BOOLEAN_INPUT: i32 = eXrActionType::XR_BOOLEAN_INPUT as i32;
pub const XR_FLOAT_INPUT: i32 = eXrActionType::XR_FLOAT_INPUT as i32;
pub const XR_VECTOR2F_INPUT: i32 = eXrActionType::XR_VECTOR2F_INPUT as i32;
pub const XR_POSE_INPUT: i32 = eXrActionType::XR_POSE_INPUT as i32;
pub const XR_VIBRATION_OUTPUT: i32 = eXrActionType::XR_VIBRATION_OUTPUT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrOpFlag {
    XR_OP_PRESS = 0,
    XR_OP_RELEASE = 1,
    XR_OP_MODAL = 2,
}

impl Default for eXrOpFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_OP_PRESS: i32 = eXrOpFlag::XR_OP_PRESS as i32;
pub const XR_OP_RELEASE: i32 = eXrOpFlag::XR_OP_RELEASE as i32;
pub const XR_OP_MODAL: i32 = eXrOpFlag::XR_OP_MODAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrActionFlag {
    XR_ACTION_BIMANUAL = (1 << 0),
}

impl Default for eXrActionFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_ACTION_BIMANUAL: i32 = eXrActionFlag::XR_ACTION_BIMANUAL as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrHapticFlag {
    XR_HAPTIC_MATCHUSERPATHS = (1 << 0),
    XR_HAPTIC_PRESS = (1 << 1),
    XR_HAPTIC_RELEASE = (1 << 2),
    XR_HAPTIC_REPEAT = (1 << 3),
}

impl Default for eXrHapticFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_HAPTIC_MATCHUSERPATHS: i32 = eXrHapticFlag::XR_HAPTIC_MATCHUSERPATHS as i32;
pub const XR_HAPTIC_PRESS: i32 = eXrHapticFlag::XR_HAPTIC_PRESS as i32;
pub const XR_HAPTIC_RELEASE: i32 = eXrHapticFlag::XR_HAPTIC_RELEASE as i32;
pub const XR_HAPTIC_REPEAT: i32 = eXrHapticFlag::XR_HAPTIC_REPEAT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrAxisFlag {
    XR_AXIS0_POS = (1 << 0),
    XR_AXIS0_NEG = (1 << 1),
    XR_AXIS1_POS = (1 << 2),
    XR_AXIS1_NEG = (1 << 3),
}

impl Default for eXrAxisFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_AXIS0_POS: i32 = eXrAxisFlag::XR_AXIS0_POS as i32;
pub const XR_AXIS0_NEG: i32 = eXrAxisFlag::XR_AXIS0_NEG as i32;
pub const XR_AXIS1_POS: i32 = eXrAxisFlag::XR_AXIS1_POS as i32;
pub const XR_AXIS1_NEG: i32 = eXrAxisFlag::XR_AXIS1_NEG as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrPoseFlag {
    XR_POSE_GRIP = (1 << 0),
    XR_POSE_AIM = (1 << 1),
}

impl Default for eXrPoseFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_POSE_GRIP: i32 = eXrPoseFlag::XR_POSE_GRIP as i32;
pub const XR_POSE_AIM: i32 = eXrPoseFlag::XR_POSE_AIM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrViewfinderHand {
    XR_VIEWFINDER_HAND_LEFT = 0, XR_VIEWFINDER_HAND_RIGHT = 1,
}

impl Default for eXrViewfinderHand {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_VIEWFINDER_HAND_LEFT: i32 = eXrViewfinderHand::XR_VIEWFINDER_HAND_LEFT as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrViewfinderMode {
    XR_VIEWFINDER_MODE_LIVE = 0,
    XR_VIEWFINDER_MODE_PLAYBACK = 1,
    XR_VIEWFINDER_MODE_CONFIRM = 2,
}

impl Default for eXrViewfinderMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_VIEWFINDER_MODE_LIVE: i32 = eXrViewfinderMode::XR_VIEWFINDER_MODE_LIVE as i32;
pub const XR_VIEWFINDER_MODE_PLAYBACK: i32 = eXrViewfinderMode::XR_VIEWFINDER_MODE_PLAYBACK as i32;
pub const XR_VIEWFINDER_MODE_CONFIRM: i32 = eXrViewfinderMode::XR_VIEWFINDER_MODE_CONFIRM as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrViewfinderLiveAction {
    XR_VIEWFINDER_ACTION_LIVE_LENS = 0,
    XR_VIEWFINDER_ACTION_LIVE_DOF = 1,
    XR_VIEWFINDER_ACTION_LIVE_FOCUS = 2,
    XR_VIEWFINDER_ACTION_LIVE_APERTURE = 3,
}

impl Default for eXrViewfinderLiveAction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_VIEWFINDER_ACTION_LIVE_LENS: i32 = eXrViewfinderLiveAction::XR_VIEWFINDER_ACTION_LIVE_LENS as i32;
pub const XR_VIEWFINDER_ACTION_LIVE_DOF: i32 = eXrViewfinderLiveAction::XR_VIEWFINDER_ACTION_LIVE_DOF as i32;
pub const XR_VIEWFINDER_ACTION_LIVE_FOCUS: i32 = eXrViewfinderLiveAction::XR_VIEWFINDER_ACTION_LIVE_FOCUS as i32;
pub const XR_VIEWFINDER_ACTION_LIVE_APERTURE: i32 = eXrViewfinderLiveAction::XR_VIEWFINDER_ACTION_LIVE_APERTURE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrViewfinderPlaybackAction {
    XR_VIEWFINDER_ACTION_PB_BROWSE = 0,
    XR_VIEWFINDER_ACTION_PB_PREVIEW = 1,
    XR_VIEWFINDER_ACTION_PB_DELETE = 2,
}

impl Default for eXrViewfinderPlaybackAction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_VIEWFINDER_ACTION_PB_BROWSE: i32 = eXrViewfinderPlaybackAction::XR_VIEWFINDER_ACTION_PB_BROWSE as i32;
pub const XR_VIEWFINDER_ACTION_PB_PREVIEW: i32 = eXrViewfinderPlaybackAction::XR_VIEWFINDER_ACTION_PB_PREVIEW as i32;
pub const XR_VIEWFINDER_ACTION_PB_DELETE: i32 = eXrViewfinderPlaybackAction::XR_VIEWFINDER_ACTION_PB_DELETE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eXrViewfinderConfirmAction {
    XR_VIEWFINDER_ACTION_CF_CONFIRM = 0,
    XR_VIEWFINDER_ACTION_CF_CANCEL = 1,
}

impl Default for eXrViewfinderConfirmAction {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const XR_VIEWFINDER_ACTION_CF_CONFIRM: i32 = eXrViewfinderConfirmAction::XR_VIEWFINDER_ACTION_CF_CONFIRM as i32;
pub const XR_VIEWFINDER_ACTION_CF_CANCEL: i32 = eXrViewfinderConfirmAction::XR_VIEWFINDER_ACTION_CF_CANCEL as i32;
