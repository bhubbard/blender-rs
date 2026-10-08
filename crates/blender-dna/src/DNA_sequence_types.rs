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
pub struct eStripFlag(pub u32);

impl eStripFlag {
    pub const SEQ_FLAG_NONE: Self = Self((0) as u32);
    pub const SEQ_SELECT: Self = Self(((1 << 0)) as u32);
    pub const SEQ_LEFTSEL: Self = Self(((1 << 1)) as u32);
    pub const SEQ_RIGHTSEL: Self = Self(((1 << 2)) as u32);
    pub const SEQ_DEINTERLACE: Self = Self(((1 << 4)) as u32);
    pub const SEQ_MUTE: Self = Self(((1 << 5)) as u32);
    pub const SEQ_FLAG_TEXT_EDITING_ACTIVE: Self = Self(((1 << 6)) as u32);
    pub const SEQ_REVERSE_FRAMES: Self = Self(((1 << 7)) as u32);
    pub const SEQ_FLIPX: Self = Self(((1 << 11)) as u32);
    pub const SEQ_FLIPY: Self = Self(((1 << 12)) as u32);
    pub const SEQ_MAKE_FLOAT: Self = Self(((1 << 13)) as u32);
    pub const SEQ_LOCK: Self = Self(((1 << 14)) as u32);
    pub const SEQ_USE_PROXY: Self = Self(((1 << 15)) as u32);
    pub const SEQ_AUTO_PLAYBACK_RATE: Self = Self(((1 << 17)) as u32);
    pub const SEQ_SINGLE_FRAME_CONTENT: Self = Self(((1 << 18)) as u32);
    pub const SEQ_SHOW_RETIMING: Self = Self(((1 << 19)) as u32);
    pub const SEQ_MULTIPLY_ALPHA: Self = Self(((1 << 21)) as u32);
    pub const SEQ_USE_EFFECT_DEFAULT_FADE: Self = Self(((1 << 22)) as u32);
    pub const SEQ_AUDIO_VOLUME_ANIMATED: Self = Self(((1 << 24)) as u32);
    pub const SEQ_AUDIO_PITCH_ANIMATED: Self = Self(((1 << 25)) as u32);
    pub const SEQ_AUDIO_PAN_ANIMATED: Self = Self(((1 << 26)) as u32);
    pub const SEQ_AUDIO_DRAW_WAVEFORM: Self = Self(((1 << 27)) as u32);
    pub const SEQ_SCENE_NO_ANNOTATION: Self = Self(((1 << 28)) as u32);
    pub const SEQ_USE_VIEWS: Self = Self(((1 << 29)) as u32);
    pub const SEQ_SCENE_STRIPS: Self = Self(((1 << 30)) as u32);
    pub const SEQ_AUDIO_PITCH_CORRECTION: Self = Self(25 as u32);
    pub const STRIP_ALLSEL: Self = Self(26 as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripProxyStorageFlag(pub u8);

impl eStripProxyStorageFlag {
    pub const SEQ_STORAGE_PROXY_NONE: Self = Self((0) as u8);
    pub const SEQ_STORAGE_PROXY_CUSTOM_FILE: Self = Self(((1 << 1)) as u8);
    pub const SEQ_STORAGE_PROXY_CUSTOM_DIR: Self = Self(((1 << 2)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripProxyBuildSize(pub u16);

impl eStripProxyBuildSize {
    pub const SEQ_PROXY_IMAGE_SIZE_NONE: Self = Self((0) as u16);
    pub const SEQ_PROXY_IMAGE_SIZE_25: Self = Self((1 << 0) as u16);
    pub const SEQ_PROXY_IMAGE_SIZE_50: Self = Self((1 << 1) as u16);
    pub const SEQ_PROXY_IMAGE_SIZE_75: Self = Self((1 << 2) as u16);
    pub const SEQ_PROXY_IMAGE_SIZE_100: Self = Self((1 << 3) as u16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripProxyBuildFlag(pub u16);

impl eStripProxyBuildFlag {
    pub const SEQ_PROXY_BUILD_FLAG_NONE: Self = Self((0) as u16);
    pub const SEQ_PROXY_SKIP_EXISTING: Self = Self((1) as u16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripAlphaMode(pub u8);

impl eStripAlphaMode {
    pub const SEQ_ALPHA_STRAIGHT: Self = Self((0) as u8);
    pub const SEQ_ALPHA_PREMUL: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StripType(pub u32);

impl StripType {
    pub const STRIP_TYPE_IMAGE: Self = Self((0) as u32);
    pub const STRIP_TYPE_META: Self = Self((1) as u32);
    pub const STRIP_TYPE_SCENE: Self = Self((2) as u32);
    pub const STRIP_TYPE_MOVIE: Self = Self((3) as u32);
    pub const STRIP_TYPE_SOUND: Self = Self((4) as u32);
    pub const STRIP_TYPE_SOUND_HD: Self = Self((5) as u32);
    pub const STRIP_TYPE_MOVIECLIP: Self = Self((6) as u32);
    pub const STRIP_TYPE_MASK: Self = Self((7) as u32);
    pub const STRIP_TYPE_CROSS: Self = Self((8) as u32);
    pub const STRIP_TYPE_ADD: Self = Self((9) as u32);
    pub const STRIP_TYPE_SUB: Self = Self((10) as u32);
    pub const STRIP_TYPE_ALPHAOVER: Self = Self((11) as u32);
    pub const STRIP_TYPE_ALPHAUNDER: Self = Self((12) as u32);
    pub const STRIP_TYPE_GAMCROSS: Self = Self((13) as u32);
    pub const STRIP_TYPE_MUL: Self = Self((14) as u32);
    pub const STRIP_TYPE_OVERDROP_REMOVED: Self = Self((15) as u32);
    pub const STRIP_TYPE_COMPOSITOR: Self = Self((16) as u32);
    pub const STRIP_TYPE_WIPE: Self = Self((25) as u32);
    pub const STRIP_TYPE_GLOW: Self = Self((26) as u32);
    pub const STRIP_TYPE_TRANSFORM_LEGACY: Self = Self((27) as u32);
    pub const STRIP_TYPE_COLOR: Self = Self((28) as u32);
    pub const STRIP_TYPE_SPEED: Self = Self((29) as u32);
    pub const STRIP_TYPE_MULTICAM: Self = Self((30) as u32);
    pub const STRIP_TYPE_ADJUSTMENT: Self = Self((31) as u32);
    pub const STRIP_TYPE_GAUSSIAN_BLUR: Self = Self((40) as u32);
    pub const STRIP_TYPE_TEXT: Self = Self((41) as u32);
    pub const STRIP_TYPE_COLORMIX: Self = Self((42) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripMovieClipFlag(pub u32);

impl eStripMovieClipFlag {
    pub const SEQ_MOVIECLIP_NONE: Self = Self((0) as u32);
    pub const SEQ_MOVIECLIP_RENDER_UNDISTORTED: Self = Self((1 << 0) as u32);
    pub const SEQ_MOVIECLIP_RENDER_STABILIZED: Self = Self((1 << 1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StripBlendMode(pub u32);

impl StripBlendMode {
    pub const STRIP_BLEND_REPLACE: Self = Self((0) as u32);
    pub const STRIP_BLEND_CROSS: Self = Self((8) as u32);
    pub const STRIP_BLEND_ADD: Self = Self((9) as u32);
    pub const STRIP_BLEND_SUB: Self = Self((10) as u32);
    pub const STRIP_BLEND_ALPHAOVER: Self = Self((11) as u32);
    pub const STRIP_BLEND_ALPHAUNDER: Self = Self((12) as u32);
    pub const STRIP_BLEND_GAMCROSS: Self = Self((13) as u32);
    pub const STRIP_BLEND_MUL: Self = Self((14) as u32);
    pub const STRIP_BLEND_OVERDROP_REMOVED: Self = Self((15) as u32);
    pub const STRIP_BLEND_SCREEN: Self = Self((43) as u32);
    pub const STRIP_BLEND_LIGHTEN: Self = Self((44) as u32);
    pub const STRIP_BLEND_DODGE: Self = Self((45) as u32);
    pub const STRIP_BLEND_DARKEN: Self = Self((46) as u32);
    pub const STRIP_BLEND_COLOR_BURN: Self = Self((47) as u32);
    pub const STRIP_BLEND_LINEAR_BURN: Self = Self((48) as u32);
    pub const STRIP_BLEND_OVERLAY: Self = Self((49) as u32);
    pub const STRIP_BLEND_HARD_LIGHT: Self = Self((50) as u32);
    pub const STRIP_BLEND_SOFT_LIGHT: Self = Self((51) as u32);
    pub const STRIP_BLEND_PIN_LIGHT: Self = Self((52) as u32);
    pub const STRIP_BLEND_LIN_LIGHT: Self = Self((53) as u32);
    pub const STRIP_BLEND_VIVID_LIGHT: Self = Self((54) as u32);
    pub const STRIP_BLEND_HUE: Self = Self((55) as u32);
    pub const STRIP_BLEND_SATURATION: Self = Self((56) as u32);
    pub const STRIP_BLEND_VALUE: Self = Self((57) as u32);
    pub const STRIP_BLEND_BLEND_COLOR: Self = Self((58) as u32);
    pub const STRIP_BLEND_DIFFERENCE: Self = Self((59) as u32);
    pub const STRIP_BLEND_EXCLUSION: Self = Self((60) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StripColorTag(pub i8);

impl StripColorTag {
    pub const STRIP_COLOR_NONE: Self = Self((-1) as i8);
    pub const STRIP_COLOR_01: Self = Self(1 as i8);
    pub const STRIP_COLOR_02: Self = Self(2 as i8);
    pub const STRIP_COLOR_03: Self = Self(3 as i8);
    pub const STRIP_COLOR_04: Self = Self(4 as i8);
    pub const STRIP_COLOR_05: Self = Self(5 as i8);
    pub const STRIP_COLOR_06: Self = Self(6 as i8);
    pub const STRIP_COLOR_07: Self = Self(7 as i8);
    pub const STRIP_COLOR_08: Self = Self(8 as i8);
    pub const STRIP_COLOR_09: Self = Self(9 as i8);
    pub const STRIP_COLOR_TOT: Self = Self(10 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripTransformFilter(pub i32);

impl eStripTransformFilter {
    pub const SEQ_TRANSFORM_FILTER_AUTO: Self = Self((-1) as i32);
    pub const SEQ_TRANSFORM_FILTER_NEAREST: Self = Self((0) as i32);
    pub const SEQ_TRANSFORM_FILTER_BILINEAR: Self = Self((1) as i32);
    pub const SEQ_TRANSFORM_FILTER_BOX: Self = Self((2) as i32);
    pub const SEQ_TRANSFORM_FILTER_CUBIC_BSPLINE: Self = Self((3) as i32);
    pub const SEQ_TRANSFORM_FILTER_CUBIC_MITCHELL: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSeqChannelFlag(pub u32);

impl eSeqChannelFlag {
    pub const SEQ_CHANNEL_NONE: Self = Self((0) as u32);
    pub const SEQ_CHANNEL_LOCK: Self = Self(((1 << 0)) as u32);
    pub const SEQ_CHANNEL_MUTE: Self = Self(((1 << 1)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eModColorBalanceMethod(pub u32);

impl eModColorBalanceMethod {
    pub const SEQ_COLOR_BALANCE_METHOD_LIFTGAMMAGAIN: Self = Self((0) as u32);
    pub const SEQ_COLOR_BALANCE_METHOD_SLOPEOFFSETPOWER: Self = Self((1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eModColorBalanceInverseFlag(pub u32);

impl eModColorBalanceInverseFlag {
    pub const SEQ_COLOR_BALANCE_INVERSE_NONE: Self = Self((0) as u32);
    pub const SEQ_COLOR_BALANCE_INVERSE_GAIN: Self = Self((1 << 0) as u32);
    pub const SEQ_COLOR_BALANCE_INVERSE_GAMMA: Self = Self((1 << 1) as u32);
    pub const SEQ_COLOR_BALANCE_INVERSE_LIFT: Self = Self((1 << 2) as u32);
    pub const SEQ_COLOR_BALANCE_INVERSE_SLOPE: Self = Self((1 << 3) as u32);
    pub const SEQ_COLOR_BALANCE_INVERSE_OFFSET: Self = Self((1 << 4) as u32);
    pub const SEQ_COLOR_BALANCE_INVERSE_POWER: Self = Self((1 << 5) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSeqRetimingKeyFlag(pub u32);

impl eSeqRetimingKeyFlag {
    pub const SEQ_RETIMING_FLAG_NONE: Self = Self((0) as u32);
    pub const SEQ_SPEED_TRANSITION_IN: Self = Self(((1 << 0)) as u32);
    pub const SEQ_SPEED_TRANSITION_OUT: Self = Self(((1 << 1)) as u32);
    pub const SEQ_FREEZE_FRAME_IN: Self = Self(((1 << 2)) as u32);
    pub const SEQ_FREEZE_FRAME_OUT: Self = Self(((1 << 3)) as u32);
    pub const SEQ_KEY_SELECTED: Self = Self(((1 << 4)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEditingOverlayFrameFlag(pub u32);

impl eEditingOverlayFrameFlag {
    pub const SEQ_EDIT_OVERLAY_FRAME_NONE: Self = Self((0) as u32);
    pub const SEQ_EDIT_OVERLAY_FRAME_SHOW: Self = Self((1) as u32);
    pub const SEQ_EDIT_OVERLAY_FRAME_ABS: Self = Self((2) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEditingShowMissingMediaFlag(pub u32);

impl eEditingShowMissingMediaFlag {
    pub const SEQ_EDIT_SHOW_NONE: Self = Self((0) as u32);
    pub const SEQ_EDIT_SHOW_MISSING_MEDIA: Self = Self((1 << 0) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEditingProxyStorageMode(pub u32);

impl eEditingProxyStorageMode {
    pub const SEQ_EDIT_PROXY_NONE: Self = Self((0) as u32);
    pub const SEQ_EDIT_PROXY_DIR_STORAGE: Self = Self((1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEditingCacheFlag(pub u32);

impl eEditingCacheFlag {
    pub const SEQ_CACHE_NONE: Self = Self((0) as u32);
    pub const SEQ_CACHE_STORE_RAW: Self = Self(((1 << 0)) as u32);
    pub const SEQ_CACHE_UNUSED_1: Self = Self(((1 << 1)) as u32);
    pub const SEQ_CACHE_UNUSED_2: Self = Self(((1 << 2)) as u32);
    pub const SEQ_CACHE_STORE_FINAL_OUT: Self = Self(((1 << 3)) as u32);
    pub const SEQ_CACHE_ALL_TYPES: Self = Self(5 as u32);
    pub const SEQ_CACHE_UNUSED_4: Self = Self(((1 << 4)) as u32);
    pub const SEQ_CACHE_UNUSED_5: Self = Self(((1 << 5)) as u32);
    pub const SEQ_CACHE_UNUSED_6: Self = Self(((1 << 6)) as u32);
    pub const SEQ_CACHE_UNUSED_7: Self = Self(((1 << 7)) as u32);
    pub const SEQ_CACHE_UNUSED_8: Self = Self(((1 << 8)) as u32);
    pub const SEQ_CACHE_UNUSED_9: Self = Self(((1 << 9)) as u32);
    pub const SEQ_CACHE_PREFETCH_ENABLE: Self = Self(((1 << 10)) as u32);
    pub const SEQ_CACHE_UNUSED_11: Self = Self(((1 << 11)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEffectWipeType(pub u16);

impl eEffectWipeType {
    pub const SEQ_WIPE_SINGLE: Self = Self(0 as u16);
    pub const SEQ_WIPE_DOUBLE: Self = Self(1 as u16);
    pub const SEQ_WIPE_IRIS: Self = Self(2 as u16);
    pub const SEQ_WIPE_CLOCK: Self = Self(3 as u16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEffectSpeedControlFlags(pub u32);

impl eEffectSpeedControlFlags {
    pub const SEQ_SPEED_NONE: Self = Self((0) as u32);
    pub const SEQ_SPEED_UNUSED_2: Self = Self((1 << 0) as u32);
    pub const SEQ_SPEED_UNUSED_1: Self = Self((1 << 1) as u32);
    pub const SEQ_SPEED_UNUSED_3: Self = Self((1 << 2) as u32);
    pub const SEQ_SPEED_USE_INTERPOLATION: Self = Self((1 << 3) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEffectSpeedControlType(pub u32);

impl eEffectSpeedControlType {
    pub const SEQ_SPEED_STRETCH: Self = Self((0) as u32);
    pub const SEQ_SPEED_MULTIPLY: Self = Self((1) as u32);
    pub const SEQ_SPEED_LENGTH: Self = Self((2) as u32);
    pub const SEQ_SPEED_FRAME_NUMBER: Self = Self((3) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEffectTextFlags(pub u8);

impl eEffectTextFlags {
    pub const SEQ_TEXT_NONE: Self = Self((0) as u8);
    pub const SEQ_TEXT_SHADOW: Self = Self(((1 << 0)) as u8);
    pub const SEQ_TEXT_BOX: Self = Self(((1 << 1)) as u8);
    pub const SEQ_TEXT_BOLD: Self = Self(((1 << 2)) as u8);
    pub const SEQ_TEXT_ITALIC: Self = Self(((1 << 3)) as u8);
    pub const SEQ_TEXT_OUTLINE: Self = Self(((1 << 4)) as u8);
    pub const SEQ_TEXT_USE_ABSOLUTE_LINE_SPACING: Self = Self(((1 << 5)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEffectTextAlignX(pub u8);

impl eEffectTextAlignX {
    pub const SEQ_TEXT_ALIGN_X_LEFT: Self = Self((0) as u8);
    pub const SEQ_TEXT_ALIGN_X_CENTER: Self = Self((1) as u8);
    pub const SEQ_TEXT_ALIGN_X_RIGHT: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEffectTextAnchorX(pub u8);

impl eEffectTextAnchorX {
    pub const SEQ_TEXT_ANCHOR_X_LEFT: Self = Self((0) as u8);
    pub const SEQ_TEXT_ANCHOR_X_CENTER: Self = Self((1) as u8);
    pub const SEQ_TEXT_ANCHOR_X_RIGHT: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eEffectTextAnchorY(pub u8);

impl eEffectTextAnchorY {
    pub const SEQ_TEXT_ANCHOR_Y_TOP: Self = Self((0) as u8);
    pub const SEQ_TEXT_ANCHOR_Y_CENTER: Self = Self((1) as u8);
    pub const SEQ_TEXT_ANCHOR_Y_BOTTOM: Self = Self((2) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eModTonemapType(pub u32);

impl eModTonemapType {
    pub const SEQ_TONEMAP_RH_SIMPLE: Self = Self((0) as u32);
    pub const SEQ_TONEMAP_RD_PHOTORECEPTOR: Self = Self((1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePitchMode(pub u32);

impl ePitchMode {
    pub const PITCH_MODE_SEMITONES: Self = Self((0) as u32);
    pub const PITCH_MODE_RATIO: Self = Self((1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePitchQuality(pub u32);

impl ePitchQuality {
    pub const PITCH_QUALITY_HIGH: Self = Self((0) as u32);
    pub const PITCH_QUALITY_FAST: Self = Self((1) as u32);
    pub const PITCH_QUALITY_CONSISTENT: Self = Self((2) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripModifierType(pub u32);

impl eStripModifierType {
    pub const eSeqModifierType_None: Self = Self((0) as u32);
    pub const eSeqModifierType_ColorBalance: Self = Self((1) as u32);
    pub const eSeqModifierType_Curves: Self = Self((2) as u32);
    pub const eSeqModifierType_HueCorrect: Self = Self((3) as u32);
    pub const eSeqModifierType_BrightContrast: Self = Self((4) as u32);
    pub const eSeqModifierType_Mask: Self = Self((5) as u32);
    pub const eSeqModifierType_WhiteBalance: Self = Self((6) as u32);
    pub const eSeqModifierType_Tonemap: Self = Self((7) as u32);
    pub const eSeqModifierType_SoundEqualizer: Self = Self((8) as u32);
    pub const eSeqModifierType_Compositor: Self = Self((9) as u32);
    pub const eSeqModifierType_Pitch: Self = Self((10) as u32);
    pub const eSeqModifierType_Echo: Self = Self((11) as u32);
    pub const NUM_STRIP_MODIFIER_TYPES: Self = Self(12 as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eStripModifierFlag(pub u32);

impl eStripModifierFlag {
    pub const STRIP_MODIFIER_FLAG_NONE: Self = Self((0) as u32);
    pub const STRIP_MODIFIER_FLAG_MUTE: Self = Self(((1 << 0)) as u32);
    pub const STRIP_MODIFIER_FLAG_EXPANDED: Self = Self(((1 << 1)) as u32);
    pub const STRIP_MODIFIER_FLAG_ACTIVE: Self = Self(((1 << 2)) as u32);
    pub const STRIP_MODIFIER_FLAG_SHOW_PREVIEW: Self = Self(((1 << 3)) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eModMaskInput(pub u32);

impl eModMaskInput {
    pub const STRIP_MASK_INPUT_STRIP: Self = Self((0) as u32);
    pub const STRIP_MASK_INPUT_ID: Self = Self((1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eModMaskTime(pub u32);

impl eModMaskTime {
    pub const STRIP_MASK_TIME_RELATIVE: Self = Self((0) as u32);
    pub const STRIP_MASK_TIME_ABSOLUTE: Self = Self((1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SequencerCompositorModifierFlag(pub u8);

impl SequencerCompositorModifierFlag {
    pub const SEQ_COMP_MOD_NONE: Self = Self((0) as u8);
    pub const SEQ_COMP_MOD_HIDE_DATABLOCK_SELECTOR: Self = Self(((1 << 0)) as u8);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripElem {
    pub filename: [u8; 256],
    pub orig_width: i32,
    pub orig_height: i32,
    pub orig_fps: f32,
}

impl Default for StripElem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripCrop {
    pub top: i32,
    pub bottom: i32,
    pub left: i32,
    pub right: i32,
}

impl Default for StripCrop {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripTransform {
    pub xofs: f32,
    pub yofs: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub origin: [f32; 2],
}

impl Default for StripTransform {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripColorBalance {
    pub method: eModColorBalanceMethod,
    pub lift: [f32; 3],
    pub _1: f32,
    pub _1_1: f32,
}

impl Default for StripColorBalance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripProxy {
    pub dirpath: [u8; 768],
    pub filename: [u8; 256],
    pub anim: *mut core::ffi::c_void,
    pub quality: i16,
    pub build_size_flags: eStripProxyBuildSize,
    pub build_flags: eStripProxyBuildFlag,
    pub storage: eStripProxyStorageFlag,
    pub _pad: i8,
}

impl Default for StripProxy {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripData {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub stripdata: *mut core::ffi::c_void,
    pub stripdata_num: i32,
    pub _pad: [u8; 4],
}

impl Default for StripData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SeqRetimingKey {
    pub strip_frame_index: f64,
    pub flag: eSeqRetimingKeyFlag,
    pub retiming_factor: f32,
    pub original_strip_frame_index: f64,
    pub original_retiming_factor: f32,
    pub _pad: [u8; 4],
}

impl Default for SeqRetimingKey {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Strip {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub flag: eStripFlag,
    pub r#type: StripType,
    pub len: i32,
    pub start: f32,
    pub startofs: f32,
    pub endofs: f32,
    pub channel: i32,
    pub startdisp: i32,
    pub enddisp: i32,
    pub sat: f32,
    pub mul: f32,
    pub streamindex: i16,
    pub _pad1: i16,
}

impl Default for Strip {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MetaStack {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub old_strip: *mut core::ffi::c_void,
    pub parent_strip: *mut core::ffi::c_void,
    pub disp_range: [i32; 2],
}

impl Default for MetaStack {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripConnection {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub strip_ref: *mut core::ffi::c_void,
}

impl Default for StripConnection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Editing {
    pub current_meta_strip: *mut core::ffi::c_void,
    pub seqbase: ListBaseT<Strip>,
    pub nullptr: ListBaseT<Strip>,
}

impl Default for Editing {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WipeVars {
    pub edgeWidth: f32,
    pub angle: f32,
    pub forward: i16,
    pub wipetype: eEffectWipeType,
}

impl Default for WipeVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GlowVars {
    pub fMini: f32,
    pub fClamp: f32,
    pub fBoost: f32,
    pub dDist: f32,
    pub dQuality: i32,
    pub bNoComp: i32,
}

impl Default for GlowVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TransformVarsLegacy {
    pub ScalexIni: f32,
}

impl Default for TransformVarsLegacy {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SolidColorVars {
    pub col: [f32; 3],
}

impl Default for SolidColorVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SpeedControlVars {
    pub flags: eEffectSpeedControlFlags,
    pub speed_control_type: eEffectSpeedControlType,
    pub speed_fader: f32,
    pub speed_fader_length: f32,
    pub speed_fader_frame_number: f32,
}

impl Default for SpeedControlVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GaussianBlurVars {
    pub size_x: f32,
    pub size_y: f32,
}

impl Default for GaussianBlurVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TextVars {
    pub text_len_bytes: i32,
    pub _pad2: [u8; 4],
}

impl Default for TextVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ColorMixVars {
    pub blend_effect: StripBlendMode,
    pub factor: f32,
}

impl Default for ColorMixVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CompositorEffectVars {
    pub node_group: *mut core::ffi::c_void,
    pub system_properties: *mut core::ffi::c_void,
}

impl Default for CompositorEffectVars {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StripModifierData {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub r#type: eStripModifierType,
    pub flag: eStripModifierFlag,
    pub name: [u8; 64],
    pub mask_input_type: eModMaskInput,
    pub mask_time: eModMaskTime,
    pub mask_strip: *mut core::ffi::c_void,
    pub mask_id: *mut core::ffi::c_void,
    pub persistent_uid: i32,
    pub layout_panel_open_flag: u16,
    pub ui_expand_flag: u16,
    pub system_properties: *mut core::ffi::c_void,
    pub runtime: *mut core::ffi::c_void,
}

impl Default for StripModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ColorBalanceModifierData {
    pub modifier: StripModifierData,
    pub color_balance: StripColorBalance,
    pub color_multiply: f32,
}

impl Default for ColorBalanceModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurvesModifierData {
    pub modifier: StripModifierData,
    pub curve_mapping: CurveMapping,
}

impl Default for CurvesModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct HueCorrectModifierData {
    pub modifier: StripModifierData,
    pub curve_mapping: CurveMapping,
}

impl Default for HueCorrectModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BrightContrastModifierData {
    pub modifier: StripModifierData,
    pub bright: f32,
    pub contrast: f32,
}

impl Default for BrightContrastModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SequencerMaskModifierData {
    pub modifier: StripModifierData,
}

impl Default for SequencerMaskModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WhiteBalanceModifierData {
    pub modifier: StripModifierData,
    pub white_value: [f32; 3],
    pub _1: f32,
    pub _1_1: f32,
}

impl Default for WhiteBalanceModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SequencerTonemapModifierData {
    pub modifier: StripModifierData,
    pub key: f32,
    pub offset: f32,
    pub gamma: f32,
    pub intensity: f32,
    pub contrast: f32,
    pub adaptation: f32,
    pub correction: f32,
    pub r#type: eModTonemapType,
}

impl Default for SequencerTonemapModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SequencerCompositorModifierData {
    pub modifier: StripModifierData,
    pub flag: SequencerCompositorModifierFlag,
    pub _pad: [u8; 7],
}

impl Default for SequencerCompositorModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct EQCurveMappingData {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub curve_mapping: CurveMapping,
}

impl Default for EQCurveMappingData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SoundEqualizerModifierData {
    pub modifier: StripModifierData,
    pub graphics: ListBaseT<EQCurveMappingData>,
    pub nullptr: ListBaseT<EQCurveMappingData>,
}

impl Default for SoundEqualizerModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PitchModifierData {
    pub modifier: StripModifierData,
    pub mode: ePitchMode,
    pub semitones: i32,
    pub cents: i32,
    pub ratio: f32,
    pub preserve_formant: i8,
    pub _pad: [u8; 3],
}

impl Default for PitchModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct EchoModifierData {
    pub modifier: StripModifierData,
    pub delay: f32,
    pub feedback: f32,
    pub mix: f32,
    pub _pad: [u8; 4],
}

impl Default for EchoModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

