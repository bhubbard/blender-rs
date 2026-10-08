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
pub struct eFFMpegPreset(pub i32);

impl eFFMpegPreset {
    pub const FFM_PRESET_NONE: Self = Self((0) as i32);
    pub const FFM_PRESET_ULTRAFAST: Self = Self((1) as i32);
    pub const FFM_PRESET_SUPERFAST: Self = Self((2) as i32);
    pub const FFM_PRESET_VERYFAST: Self = Self((3) as i32);
    pub const FFM_PRESET_FASTER: Self = Self((4) as i32);
    pub const FFM_PRESET_FAST: Self = Self((5) as i32);
    pub const FFM_PRESET_MEDIUM: Self = Self((6) as i32);
    pub const FFM_PRESET_SLOW: Self = Self((7) as i32);
    pub const FFM_PRESET_SLOWER: Self = Self((8) as i32);
    pub const FFM_PRESET_VERYSLOW: Self = Self((9) as i32);
    pub const FFM_PRESET_GOOD: Self = Self((10) as i32);
    pub const FFM_PRESET_BEST: Self = Self((11) as i32);
    pub const FFM_PRESET_REALTIME: Self = Self((12) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFFMpegCrf(pub i32);

impl eFFMpegCrf {
    pub const FFM_CRF_NONE: Self = Self((-1) as i32);
    pub const FFM_CRF_LOSSLESS: Self = Self((0) as i32);
    pub const FFM_CRF_PERC_LOSSLESS: Self = Self((17) as i32);
    pub const FFM_CRF_HIGH: Self = Self((20) as i32);
    pub const FFM_CRF_MEDIUM: Self = Self((23) as i32);
    pub const FFM_CRF_LOW: Self = Self((26) as i32);
    pub const FFM_CRF_VERYLOW: Self = Self((29) as i32);
    pub const FFM_CRF_LOWEST: Self = Self((32) as i32);
    pub const FFM_CRF_CUSTOM: Self = Self((128) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFFMpegAudioChannels(pub i32);

impl eFFMpegAudioChannels {
    pub const FFM_CHANNELS_MONO: Self = Self((1) as i32);
    pub const FFM_CHANNELS_STEREO: Self = Self((2) as i32);
    pub const FFM_CHANNELS_SURROUND4: Self = Self((4) as i32);
    pub const FFM_CHANNELS_SURROUND51: Self = Self((6) as i32);
    pub const FFM_CHANNELS_SURROUND71: Self = Self((8) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFFMpegProresProfile(pub i32);

impl eFFMpegProresProfile {
    pub const FFM_PRORES_PROFILE_422_PROXY: Self = Self((0) as i32);
    pub const FFM_PRORES_PROFILE_422_LT: Self = Self((1) as i32);
    pub const FFM_PRORES_PROFILE_422_STD: Self = Self((2) as i32);
    pub const FFM_PRORES_PROFILE_422_HQ: Self = Self((3) as i32);
    pub const FFM_PRORES_PROFILE_4444: Self = Self((4) as i32);
    pub const FFM_PRORES_PROFILE_4444_XQ: Self = Self((5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IMB_Ffmpeg_Codec_ID(pub i32);

impl IMB_Ffmpeg_Codec_ID {
    pub const FFMPEG_CODEC_ID_NONE: Self = Self((0) as i32);
    pub const FFMPEG_CODEC_ID_MPEG1VIDEO: Self = Self((1) as i32);
    pub const FFMPEG_CODEC_ID_MPEG2VIDEO: Self = Self((2) as i32);
    pub const FFMPEG_CODEC_ID_MPEG4: Self = Self((12) as i32);
    pub const FFMPEG_CODEC_ID_FLV1: Self = Self((21) as i32);
    pub const FFMPEG_CODEC_ID_DVVIDEO: Self = Self((24) as i32);
    pub const FFMPEG_CODEC_ID_HUFFYUV: Self = Self((25) as i32);
    pub const FFMPEG_CODEC_ID_H264: Self = Self((27) as i32);
    pub const FFMPEG_CODEC_ID_THEORA: Self = Self((30) as i32);
    pub const FFMPEG_CODEC_ID_FFV1: Self = Self((33) as i32);
    pub const FFMPEG_CODEC_ID_QTRLE: Self = Self((55) as i32);
    pub const FFMPEG_CODEC_ID_PNG: Self = Self((61) as i32);
    pub const FFMPEG_CODEC_ID_DNXHD: Self = Self((99) as i32);
    pub const FFMPEG_CODEC_ID_VP9: Self = Self((167) as i32);
    pub const FFMPEG_CODEC_ID_H265: Self = Self((173) as i32);
    pub const FFMPEG_CODEC_ID_AV1: Self = Self((226) as i32);
    pub const FFMPEG_CODEC_ID_PRORES: Self = Self((147) as i32);
    pub const FFMPEG_CODEC_ID_PCM_S16LE: Self = Self((65536) as i32);
    pub const FFMPEG_CODEC_ID_MP2: Self = Self((86016) as i32);
    pub const FFMPEG_CODEC_ID_MP3: Self = Self((86017) as i32);
    pub const FFMPEG_CODEC_ID_AAC: Self = Self((86018) as i32);
    pub const FFMPEG_CODEC_ID_AC3: Self = Self((86019) as i32);
    pub const FFMPEG_CODEC_ID_VORBIS: Self = Self((86021) as i32);
    pub const FFMPEG_CODEC_ID_FLAC: Self = Self((86028) as i32);
    pub const FFMPEG_CODEC_ID_OPUS: Self = Self((86076) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eFFMpegCodec_Flag(pub i32);

impl eFFMpegCodec_Flag {
    pub const FFMPEG_MULTIPLEX_AUDIO: Self = Self(((1 << 0)) as i32);
    pub const FFMPEG_AUTOSPLIT_OUTPUT: Self = Self(((1 << 1)) as i32);
    pub const FFMPEG_LOSSLESS_OUTPUT: Self = Self(((1 << 2)) as i32);
    pub const FFMPEG_USE_MAX_B_FRAMES: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eAudio_Flag(pub i16);

impl eAudio_Flag {
    pub const AUDIO_MUTE: Self = Self((1 << 0) as i16);
    pub const AUDIO_SYNC: Self = Self((1 << 1) as i16);
    pub const AUDIO_SCRUB: Self = Self((1 << 2) as i16);
    pub const AUDIO_VOLUME_ANIMATED: Self = Self((1 << 3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MediaType(pub i8);

impl MediaType {
    pub const MEDIA_TYPE_IMAGE: Self = Self((0) as i8);
    pub const MEDIA_TYPE_MULTI_LAYER_IMAGE: Self = Self((1) as i8);
    pub const MEDIA_TYPE_VIDEO: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_Legacy(pub i8);

impl eMediaType_Legacy {
    pub const R_IMF_IMTYPE_TARGA: Self = Self((0) as i8);
    pub const R_IMF_IMTYPE_IRIS: Self = Self((1) as i8);
    pub const R_IMF_IMTYPE_JPEG90: Self = Self((4) as i8);
    pub const R_IMF_IMTYPE_IRIZ: Self = Self((7) as i8);
    pub const R_IMF_IMTYPE_RAWTGA: Self = Self((14) as i8);
    pub const R_IMF_IMTYPE_PNG: Self = Self((17) as i8);
    pub const R_IMF_IMTYPE_BMP: Self = Self((20) as i8);
    pub const R_IMF_IMTYPE_RADHDR: Self = Self((21) as i8);
    pub const R_IMF_IMTYPE_TIFF: Self = Self((22) as i8);
    pub const R_IMF_IMTYPE_OPENEXR: Self = Self((23) as i8);
    pub const R_IMF_IMTYPE_FFMPEG: Self = Self((24) as i8);
    pub const R_IMF_IMTYPE_CINEON: Self = Self((26) as i8);
    pub const R_IMF_IMTYPE_DPX: Self = Self((27) as i8);
    pub const R_IMF_IMTYPE_MULTILAYER: Self = Self((28) as i8);
    pub const R_IMF_IMTYPE_DDS: Self = Self((29) as i8);
    pub const R_IMF_IMTYPE_JP2: Self = Self((30) as i8);
    pub const R_IMF_IMTYPE_PSD: Self = Self((34) as i8);
    pub const R_IMF_IMTYPE_WEBP: Self = Self((35) as i8);
    pub const R_IMF_IMTYPE_AVIF: Self = Self((37) as i8);
    pub const R_IMF_IMTYPE_INVALID: Self = Self(19 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_Flag(pub i8);

impl eMediaType_Flag {
    pub const R_IMF_FLAG_PREVIEW_JPG: Self = Self((1 << 1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImageFormatDepth(pub i8);

impl eImageFormatDepth {
    pub const R_IMF_CHAN_DEPTH_8: Self = Self(((1 << 1)) as i8);
    pub const R_IMF_CHAN_DEPTH_10: Self = Self(((1 << 2)) as i8);
    pub const R_IMF_CHAN_DEPTH_12: Self = Self(((1 << 3)) as i8);
    pub const R_IMF_CHAN_DEPTH_16: Self = Self(((1 << 4)) as i8);
    pub const R_IMF_CHAN_DEPTH_32: Self = Self(((1 << 6)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_ExrCodec(pub i8);

impl eMediaType_ExrCodec {
    pub const R_IMF_EXR_CODEC_NONE: Self = Self((0) as i8);
    pub const R_IMF_EXR_CODEC_PXR24: Self = Self((1) as i8);
    pub const R_IMF_EXR_CODEC_ZIP: Self = Self((2) as i8);
    pub const R_IMF_EXR_CODEC_PIZ: Self = Self((3) as i8);
    pub const R_IMF_EXR_CODEC_RLE: Self = Self((4) as i8);
    pub const R_IMF_EXR_CODEC_ZIPS: Self = Self((5) as i8);
    pub const R_IMF_EXR_CODEC_B44: Self = Self((6) as i8);
    pub const R_IMF_EXR_CODEC_B44A: Self = Self((7) as i8);
    pub const R_IMF_EXR_CODEC_DWAA: Self = Self((8) as i8);
    pub const R_IMF_EXR_CODEC_DWAB: Self = Self((9) as i8);
    pub const R_IMF_EXR_CODEC_HTJ2K: Self = Self((10) as i8);
    pub const R_IMF_EXR_CODEC_MAX: Self = Self((11) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_ExrFlag(pub i8);

impl eMediaType_ExrFlag {
    pub const R_IMF_EXR_FLAG_MULTIPART: Self = Self((1 << 0) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_Jp2Flag(pub i8);

impl eMediaType_Jp2Flag {
    pub const R_IMF_JP2_FLAG_YCC: Self = Self((1 << 0) as i8);
    pub const R_IMF_JP2_FLAG_CINE_PRESET: Self = Self((1 << 1) as i8);
    pub const R_IMF_JP2_FLAG_CINE_48: Self = Self((1 << 2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_Jp2Codec(pub i8);

impl eMediaType_Jp2Codec {
    pub const R_IMF_JP2_CODEC_JP2: Self = Self((0) as i8);
    pub const R_IMF_JP2_CODEC_J2K: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_CineonFlag(pub i8);

impl eMediaType_CineonFlag {
    pub const R_IMF_CINEON_FLAG_LOG: Self = Self((1 << 0) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_TiffCodec(pub i8);

impl eMediaType_TiffCodec {
    pub const R_IMF_TIFF_CODEC_DEFLATE: Self = Self((0) as i8);
    pub const R_IMF_TIFF_CODEC_LZW: Self = Self((1) as i8);
    pub const R_IMF_TIFF_CODEC_PACKBITS: Self = Self((2) as i8);
    pub const R_IMF_TIFF_CODEC_NONE: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMediaType_ColorManagement(pub i8);

impl eMediaType_ColorManagement {
    pub const R_IMF_COLOR_MANAGEMENT_FOLLOW_SCENE: Self = Self((0) as i8);
    pub const R_IMF_COLOR_MANAGEMENT_OVERRIDE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakeType(pub i32);

impl eBakeType {
    pub const R_BAKE_NORMALS: Self = Self((0) as i32);
    pub const R_BAKE_DISPLACEMENT: Self = Self((1) as i32);
    pub const R_BAKE_AO: Self = Self((2) as i32);
    pub const R_BAKE_VECTOR_DISPLACEMENT: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBake_Flag(pub i16);

impl eBake_Flag {
    pub const R_BAKE_CLEAR: Self = Self((1 << 0) as i16);
    pub const R_BAKE_TO_ACTIVE: Self = Self((1 << 2) as i16);
    pub const R_BAKE_MULTIRES: Self = Self((1 << 4) as i16);
    pub const R_BAKE_LORES_MESH: Self = Self((1 << 5) as i16);
    pub const R_BAKE_CAGE: Self = Self((1 << 8) as i16);
    pub const R_BAKE_SPLIT_MAT: Self = Self((1 << 9) as i16);
    pub const R_BAKE_AUTO_NAME: Self = Self((1 << 10) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakeMarginType(pub i8);

impl eBakeMarginType {
    pub const R_BAKE_ADJACENT_FACES: Self = Self((0) as i8);
    pub const R_BAKE_EXTEND: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakeNormalSwizzle(pub i8);

impl eBakeNormalSwizzle {
    pub const R_BAKE_POSX: Self = Self((0) as i8);
    pub const R_BAKE_POSY: Self = Self((1) as i8);
    pub const R_BAKE_POSZ: Self = Self((2) as i8);
    pub const R_BAKE_NEGX: Self = Self((3) as i8);
    pub const R_BAKE_NEGY: Self = Self((4) as i8);
    pub const R_BAKE_NEGZ: Self = Self((5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakeTarget(pub i8);

impl eBakeTarget {
    pub const R_BAKE_TARGET_IMAGE_TEXTURES: Self = Self((0) as i8);
    pub const R_BAKE_TARGET_VERTEX_COLORS: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakeSaveMode(pub i8);

impl eBakeSaveMode {
    pub const R_BAKE_SAVE_INTERNAL: Self = Self((0) as i8);
    pub const R_BAKE_SAVE_EXTERNAL: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakeViewFrom(pub i8);

impl eBakeViewFrom {
    pub const R_BAKE_VIEW_FROM_ABOVE_SURFACE: Self = Self((0) as i8);
    pub const R_BAKE_VIEW_FROM_ACTIVE_CAMERA: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakePassFilter(pub i32);

impl eBakePassFilter {
    pub const R_BAKE_PASS_FILTER_NONE: Self = Self((0) as i32);
    pub const R_BAKE_PASS_FILTER_UNUSED: Self = Self(((1 << 0)) as i32);
    pub const R_BAKE_PASS_FILTER_EMIT: Self = Self(((1 << 1)) as i32);
    pub const R_BAKE_PASS_FILTER_DIFFUSE: Self = Self(((1 << 2)) as i32);
    pub const R_BAKE_PASS_FILTER_GLOSSY: Self = Self(((1 << 3)) as i32);
    pub const R_BAKE_PASS_FILTER_TRANSM: Self = Self(((1 << 4)) as i32);
    pub const R_BAKE_PASS_FILTER_SUBSURFACE: Self = Self(((1 << 5)) as i32);
    pub const R_BAKE_PASS_FILTER_DIRECT: Self = Self(((1 << 6)) as i32);
    pub const R_BAKE_PASS_FILTER_INDIRECT: Self = Self(((1 << 7)) as i32);
    pub const R_BAKE_PASS_FILTER_COLOR: Self = Self(((1 << 8)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eBakeSpace(pub i8);

impl eBakeSpace {
    pub const R_BAKE_SPACE_CAMERA: Self = Self((0) as i8);
    pub const R_BAKE_SPACE_WORLD: Self = Self((1) as i8);
    pub const R_BAKE_SPACE_OBJECT: Self = Self((2) as i8);
    pub const R_BAKE_SPACE_TANGENT: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eQualityOption(pub i16);

impl eQualityOption {
    pub const SCE_PERF_HQ_NORMALS: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eHairType(pub i16);

impl eHairType {
    pub const SCE_HAIR_SHAPE_STRAND: Self = Self((0) as i16);
    pub const SCE_HAIR_SHAPE_STRIP: Self = Self((1) as i16);
    pub const SCE_HAIR_SHAPE_CYLINDER: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_MotionBlurPosition(pub i32);

impl eRender_MotionBlurPosition {
    pub const SCE_MB_CENTER: Self = Self((0) as i32);
    pub const SCE_MB_START: Self = Self((1) as i32);
    pub const SCE_MB_END: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCompositorDevice(pub i32);

impl eCompositorDevice {
    pub const SCE_COMPOSITOR_DEVICE_CPU: Self = Self((0) as i32);
    pub const SCE_COMPOSITOR_DEVICE_GPU: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCompositorCacheFlags(pub u8);

impl eCompositorCacheFlags {
    pub const SCE_COMPOSITOR_CACHE_NONE: Self = Self((0) as u8);
    pub const SCE_COMPOSITOR_CACHE_FRAMES: Self = Self(((1 << 0)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCompositorPrecision(pub i32);

impl eCompositorPrecision {
    pub const SCE_COMPOSITOR_PRECISION_AUTO: Self = Self((0) as i32);
    pub const SCE_COMPOSITOR_PRECISION_FULL: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCompositorDenoiseDevice(pub i32);

impl eCompositorDenoiseDevice {
    pub const SCE_COMPOSITOR_DENOISE_DEVICE_AUTO: Self = Self((0) as i32);
    pub const SCE_COMPOSITOR_DENOISE_DEVICE_CPU: Self = Self((1) as i32);
    pub const SCE_COMPOSITOR_DENOISE_DEVICE_GPU: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCompositorDenoiseQaulity(pub i32);

impl eCompositorDenoiseQaulity {
    pub const SCE_COMPOSITOR_DENOISE_HIGH: Self = Self((0) as i32);
    pub const SCE_COMPOSITOR_DENOISE_BALANCED: Self = Self((1) as i32);
    pub const SCE_COMPOSITOR_DENOISE_FAST: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRenderOutputMode(pub i32);

impl eRenderOutputMode {
    pub const R_SAVE_MODE_DEFAULT: Self = Self((0) as i32);
    pub const R_SAVE_MODE_DISABLED: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_TimeJumpUnit(pub i32);

impl eRender_TimeJumpUnit {
    pub const SCE_TIME_JUMP_FRAME: Self = Self((0) as i32);
    pub const SCE_TIME_JUMP_SECOND: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_Flag(pub i16);

impl eRender_Flag {
    pub const SCER_PRV_RANGE: Self = Self((1 << 0) as i16);
    pub const SCER_LOCK_FRAME_SELECTION: Self = Self((1 << 1) as i16);
    pub const SCER_ALLOW_PREROLL: Self = Self((1 << 2) as i16);
    pub const SCER_SHOW_SUBFRAME: Self = Self((1 << 3) as i16);
    pub const SCER_WRAP_TIMELINE_NAVIGATION: Self = Self((1 << 4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_Mode(pub i32);

impl eRender_Mode {
    pub const R_MODE_UNUSED_0: Self = Self((1 << 0) as i32);
    pub const R_SIMPLIFY_NORMALS: Self = Self((1 << 1) as i32);
    pub const R_MODE_UNUSED_2: Self = Self((1 << 2) as i32);
    pub const R_MODE_UNUSED_3: Self = Self((1 << 3) as i32);
    pub const R_MODE_UNUSED_4: Self = Self((1 << 4) as i32);
    pub const R_MODE_UNUSED_5: Self = Self((1 << 5) as i32);
    pub const R_MODE_UNUSED_6: Self = Self((1 << 6) as i32);
    pub const R_MODE_UNUSED_7: Self = Self((1 << 7) as i32);
    pub const R_MODE_UNUSED_8: Self = Self((1 << 8) as i32);
    pub const R_BORDER: Self = Self((1 << 9) as i32);
    pub const R_MODE_UNUSED_10: Self = Self((1 << 10) as i32);
    pub const R_CROP: Self = Self((1 << 11) as i32);
    pub const R_NO_CAMERA_SWITCH: Self = Self((1 << 12) as i32);
    pub const R_MODE_UNUSED_13: Self = Self((1 << 13) as i32);
    pub const R_MBLUR: Self = Self((1 << 14) as i32);
    pub const R_MODE_UNUSED_16: Self = Self((1 << 16) as i32);
    pub const R_MODE_UNUSED_17: Self = Self((1 << 17) as i32);
    pub const R_MODE_UNUSED_18: Self = Self((1 << 18) as i32);
    pub const R_MODE_UNUSED_19: Self = Self((1 << 19) as i32);
    pub const R_FIXED_THREADS: Self = Self((1 << 19) as i32);
    pub const R_MODE_UNUSED_20: Self = Self((1 << 20) as i32);
    pub const R_MODE_UNUSED_21: Self = Self((1 << 21) as i32);
    pub const R_NO_OVERWRITE: Self = Self((1 << 22) as i32);
    pub const R_TOUCH: Self = Self((1 << 23) as i32);
    pub const R_SIMPLIFY: Self = Self((1 << 24) as i32);
    pub const R_EDGE_FRS: Self = Self((1 << 25) as i32);
    pub const R_PERSISTENT_DATA: Self = Self((1 << 26) as i32);
    pub const R_MODE_UNUSED_27: Self = Self((1 << 27) as i32);
    pub const R_SAVE_OUTPUT: Self = Self((1 << 28) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_SeqFlag(pub i8);

impl eRender_SeqFlag {
    pub const R_SEQ_UNUSED_0: Self = Self(((1 << 0)) as i8);
    pub const R_SEQ_UNUSED_1: Self = Self(((1 << 1)) as i8);
    pub const R_SEQ_UNUSED_2: Self = Self(((1 << 2)) as i8);
    pub const R_SEQ_UNUSED_3: Self = Self(((1 << 3)) as i8);
    pub const R_SEQ_UNUSED_4: Self = Self(((1 << 4)) as i8);
    pub const R_SEQ_OVERRIDE_SCENE_SETTINGS: Self = Self(((1 << 5)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_SceMode(pub i32);

impl eRender_SceMode {
    pub const R_DOSEQ: Self = Self((1 << 0) as i32);
    pub const R_BG_RENDER: Self = Self((1 << 1) as i32);
    pub const R_PASSEPARTOUT: Self = Self((1 << 2) as i32);
    pub const R_BUTS_PREVIEW: Self = Self((1 << 3) as i32);
    pub const R_EXTENSION: Self = Self((1 << 4) as i32);
    pub const R_MATNODE_PREVIEW: Self = Self((1 << 5) as i32);
    pub const R_DOCOMP: Self = Self((1 << 6) as i32);
    pub const R_COMP_CROP: Self = Self((1 << 7) as i32);
    pub const R_SCEMODE_UNUSED_8: Self = Self((1 << 8) as i32);
    pub const R_SINGLE_LAYER: Self = Self((1 << 9) as i32);
    pub const R_SCEMODE_UNUSED_10: Self = Self((1 << 10) as i32);
    pub const R_SCEMODE_UNUSED_11: Self = Self((1 << 11) as i32);
    pub const R_NO_IMAGE_LOAD: Self = Self((1 << 12) as i32);
    pub const R_SCEMODE_UNUSED_13: Self = Self((1 << 13) as i32);
    pub const R_NO_FRAME_UPDATE: Self = Self((1 << 14) as i32);
    pub const R_SCEMODE_UNUSED_15: Self = Self((1 << 15) as i32);
    pub const R_SCEMODE_UNUSED_16: Self = Self((1 << 16) as i32);
    pub const R_SCEMODE_UNUSED_17: Self = Self((1 << 17) as i32);
    pub const R_TEXNODE_PREVIEW: Self = Self((1 << 18) as i32);
    pub const R_SCEMODE_UNUSED_19: Self = Self((1 << 19) as i32);
    pub const R_EXR_CACHE_FILE: Self = Self((1 << 20) as i32);
    pub const R_MULTIVIEW: Self = Self((1 << 21) as i32);
    pub const R_USE_TEXTURE_CACHE: Self = Self((1 << 22) as i32);
    pub const R_TEXTURE_CACHE_AUTO_GENERATE: Self = Self((1 << 23) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_Stamp(pub i32);

impl eRender_Stamp {
    pub const R_STAMP_TIME: Self = Self((1 << 0) as i32);
    pub const R_STAMP_FRAME: Self = Self((1 << 1) as i32);
    pub const R_STAMP_DATE: Self = Self((1 << 2) as i32);
    pub const R_STAMP_CAMERA: Self = Self((1 << 3) as i32);
    pub const R_STAMP_SCENE: Self = Self((1 << 4) as i32);
    pub const R_STAMP_NOTE: Self = Self((1 << 5) as i32);
    pub const R_STAMP_DRAW: Self = Self((1 << 6) as i32);
    pub const R_STAMP_MARKER: Self = Self((1 << 7) as i32);
    pub const R_STAMP_FILENAME: Self = Self((1 << 8) as i32);
    pub const R_STAMP_SEQSTRIP: Self = Self((1 << 9) as i32);
    pub const R_STAMP_RENDERTIME: Self = Self((1 << 10) as i32);
    pub const R_STAMP_CAMERALENS: Self = Self((1 << 11) as i32);
    pub const R_STAMP_STRIPMETA: Self = Self((1 << 12) as i32);
    pub const R_STAMP_MEMORY: Self = Self((1 << 13) as i32);
    pub const R_STAMP_HIDE_LABELS: Self = Self((1 << 14) as i32);
    pub const R_STAMP_FRAME_RANGE: Self = Self((1 << 15) as i32);
    pub const R_STAMP_HOSTNAME: Self = Self((1 << 16) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_AlphaMode(pub i8);

impl eRender_AlphaMode {
    pub const R_ADDSKY: Self = Self((0) as i8);
    pub const R_ALPHAPREMUL: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_ColorMgtFlag(pub i32);

impl eRender_ColorMgtFlag {
    pub const R_COLOR_MANAGEMENT: Self = Self(((1 << 0)) as i32);
    pub const R_COLOR_MANAGEMENT_UNUSED_1: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRender_LineThicknessMode(pub i32);

impl eRender_LineThicknessMode {
    pub const R_LINE_THICKNESS_ABSOLUTE: Self = Self((1) as i32);
    pub const R_LINE_THICKNESS_RELATIVE: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePaintFlags(pub i32);

impl ePaintFlags {
    pub const PAINT_SHOW_BRUSH: Self = Self(((1 << 0)) as i32);
    pub const PAINT_FAST_NAVIGATE: Self = Self(((1 << 1)) as i32);
    pub const PAINT_SHOW_BRUSH_ON_SURFACE: Self = Self(((1 << 2)) as i32);
    pub const PAINT_USE_CAVITY_MASK: Self = Self(((1 << 3)) as i32);
    pub const PAINT_SCULPT_DELAY_UPDATES: Self = Self(((1 << 4)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePaintDebugFlags(pub i32);

impl ePaintDebugFlags {
    pub const PAINT_DEBUG_SHOW_BVH_NODES: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePaintCanvasSource(pub i8);

impl ePaintCanvasSource {
    pub const PAINT_CANVAS_SOURCE_MATERIAL: Self = Self((0) as i8);
    pub const PAINT_CANVAS_SOURCE_IMAGE: Self = Self((1) as i8);
    pub const PAINT_CANVAS_SOURCE_COLOR_ATTRIBUTE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImagePaint_Interpolation(pub i32);

impl eImagePaint_Interpolation {
    pub const IMAGEPAINT_INTERP_LINEAR: Self = Self((0) as i32);
    pub const IMAGEPAINT_INTERP_CLOSEST: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImagePaint_Flag(pub i16);

impl eImagePaint_Flag {
    pub const IMAGEPAINT_DRAWING: Self = Self((1 << 0) as i16);
    pub const IMAGEPAINT_PROJECT_XRAY: Self = Self((1 << 4) as i16);
    pub const IMAGEPAINT_PROJECT_BACKFACE: Self = Self((1 << 5) as i16);
    pub const IMAGEPAINT_PROJECT_FLAT: Self = Self((1 << 6) as i16);
    pub const IMAGEPAINT_PROJECT_LAYER_CLONE: Self = Self((1 << 7) as i16);
    pub const IMAGEPAINT_PROJECT_LAYER_STENCIL: Self = Self((1 << 8) as i16);
    pub const IMAGEPAINT_PROJECT_LAYER_STENCIL_INV: Self = Self((1 << 9) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eImagePaint_MissingData(pub i16);

impl eImagePaint_MissingData {
    pub const IMAGEPAINT_MISSING_UVS: Self = Self((1 << 0) as i16);
    pub const IMAGEPAINT_MISSING_MATERIAL: Self = Self((1 << 1) as i16);
    pub const IMAGEPAINT_MISSING_TEX: Self = Self((1 << 2) as i16);
    pub const IMAGEPAINT_MISSING_STENCIL: Self = Self((1 << 3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleEdit_BrushType(pub i16);

impl eParticleEdit_BrushType {
    pub const PE_BRUSH_NONE: Self = Self((-1) as i16);
    pub const PE_BRUSH_COMB: Self = Self((0) as i16);
    pub const PE_BRUSH_CUT: Self = Self((1) as i16);
    pub const PE_BRUSH_LENGTH: Self = Self((2) as i16);
    pub const PE_BRUSH_PUFF: Self = Self((3) as i16);
    pub const PE_BRUSH_ADD: Self = Self((4) as i16);
    pub const PE_BRUSH_SMOOTH: Self = Self((5) as i16);
    pub const PE_BRUSH_WEIGHT: Self = Self((6) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleEdit_BrushFlag(pub i32);

impl eParticleEdit_BrushFlag {
    pub const PE_BRUSH_DATA_PUFF_VOLUME: Self = Self((1 << 0) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleEdit_EditType(pub i32);

impl eParticleEdit_EditType {
    pub const PE_TYPE_PARTICLES: Self = Self((0) as i32);
    pub const PE_TYPE_SOFTBODY: Self = Self((1) as i32);
    pub const PE_TYPE_CLOTH: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleEdit_Flag(pub i16);

impl eParticleEdit_Flag {
    pub const PE_KEEP_LENGTHS: Self = Self((1 << 0) as i16);
    pub const PE_LOCK_FIRST: Self = Self((1 << 1) as i16);
    pub const PE_DEFLECT_EMITTER: Self = Self((1 << 2) as i16);
    pub const PE_INTERPOLATE_ADDED: Self = Self((1 << 3) as i16);
    pub const PE_DRAW_PART: Self = Self((1 << 4) as i16);
    pub const PE_UNUSED_6: Self = Self((1 << 6) as i16);
    pub const PE_FADE_TIME: Self = Self((1 << 7) as i16);
    pub const PE_AUTO_VELOCITY: Self = Self((1 << 8) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eParticleEdit_SelectMode(pub i32);

impl eParticleEdit_SelectMode {
    pub const SCE_SELECT_PATH: Self = Self((1 << 0) as i32);
    pub const SCE_SELECT_POINT: Self = Self((1 << 1) as i32);
    pub const SCE_SELECT_END: Self = Self((1 << 2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSculptFlags(pub i32);

impl eSculptFlags {
    pub const SCULPT_FLAG_UNUSED_0: Self = Self(((1 << 0)) as i32);
    pub const SCULPT_FLAG_UNUSED_1: Self = Self(((1 << 1)) as i32);
    pub const SCULPT_FLAG_UNUSED_2: Self = Self(((1 << 2)) as i32);
    pub const SCULPT_LOCK_X: Self = Self(((1 << 3)) as i32);
    pub const SCULPT_LOCK_Y: Self = Self(((1 << 4)) as i32);
    pub const SCULPT_LOCK_Z: Self = Self(((1 << 5)) as i32);
    pub const SCULPT_FLAG_UNUSED_6: Self = Self(((1 << 6)) as i32);
    pub const SCULPT_FLAG_UNUSED_7: Self = Self(((1 << 7)) as i32);
    pub const SCULPT_ONLY_DEFORM: Self = Self(((1 << 8)) as i32);
    pub const SCULPT_FLAG_UNUSED_8: Self = Self(((1 << 10)) as i32);
    pub const SCULPT_DYNTOPO_SUBDIVIDE: Self = Self(((1 << 12)) as i32);
    pub const SCULPT_DYNTOPO_COLLAPSE: Self = Self(((1 << 11)) as i32);
    pub const SCULPT_DYNTOPO_DETAIL_CONSTANT: Self = Self(((1 << 13)) as i32);
    pub const SCULPT_DYNTOPO_DETAIL_BRUSH: Self = Self(((1 << 14)) as i32);
    pub const SCULPT_DYNTOPO_DETAIL_MANUAL: Self = Self(((1 << 16)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSculptTransformMode(pub i32);

impl eSculptTransformMode {
    pub const SCULPT_TRANSFORM_MODE_ALL_VERTICES: Self = Self((0) as i32);
    pub const SCULPT_TRANSFORM_MODE_RADIUS_ELASTIC: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGP_Lockaxis_Types(pub i32);

impl eGP_Lockaxis_Types {
    pub const GP_LOCKAXIS_VIEW: Self = Self((0) as i32);
    pub const GP_LOCKAXIS_X: Self = Self((1) as i32);
    pub const GP_LOCKAXIS_Y: Self = Self((2) as i32);
    pub const GP_LOCKAXIS_Z: Self = Self((3) as i32);
    pub const GP_LOCKAXIS_CURSOR: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGP_Sculpt_SettingsFlag(pub i32);

impl eGP_Sculpt_SettingsFlag {
    pub const GP_SCULPT_SETT_FLAG_FRAME_FALLOFF: Self = Self(((1 << 0)) as i32);
    pub const GP_SCULPT_SETT_FLAG_PRIMITIVE_CURVE: Self = Self(((1 << 1)) as i32);
    pub const GP_SCULPT_SETT_FLAG_SCALE_THICKNESS: Self = Self(((1 << 3)) as i32);
    pub const GP_SCULPT_SETT_FLAG_AUTOMASK_STROKE: Self = Self(((1 << 4)) as i32);
    pub const GP_SCULPT_SETT_FLAG_AUTOMASK_LAYER_STROKE: Self = Self(((1 << 5)) as i32);
    pub const GP_SCULPT_SETT_FLAG_AUTOMASK_MATERIAL_STROKE: Self = Self(((1 << 6)) as i32);
    pub const GP_SCULPT_SETT_FLAG_AUTOMASK_LAYER_ACTIVE: Self = Self(((1 << 7)) as i32);
    pub const GP_SCULPT_SETT_FLAG_AUTOMASK_MATERIAL_ACTIVE: Self = Self(((1 << 8)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGP_Sculpt_SelectMaskFlag(pub i8);

impl eGP_Sculpt_SelectMaskFlag {
    pub const GP_SCULPT_MASK_SELECTMODE_POINT: Self = Self(((1 << 0)) as i8);
    pub const GP_SCULPT_MASK_SELECTMODE_STROKE: Self = Self(((1 << 1)) as i8);
    pub const GP_SCULPT_MASK_SELECTMODE_SEGMENT: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGP_Vertex_SelectMaskFlag(pub i8);

impl eGP_Vertex_SelectMaskFlag {
    pub const GP_VERTEX_MASK_SELECTMODE_POINT: Self = Self(((1 << 0)) as i8);
    pub const GP_VERTEX_MASK_SELECTMODE_STROKE: Self = Self(((1 << 1)) as i8);
    pub const GP_VERTEX_MASK_SELECTMODE_SEGMENT: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGP_Interpolate_SettingsFlag(pub i16);

impl eGP_Interpolate_SettingsFlag {
    pub const GP_TOOLFLAG_INTERPOLATE_ALL_LAYERS: Self = Self(((1 << 0)) as i16);
    pub const GP_TOOLFLAG_INTERPOLATE_ONLY_SELECTED: Self = Self(((1 << 1)) as i16);
    pub const GP_TOOLFLAG_INTERPOLATE_EXCLUDE_BREAKDOWNS: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGP_Interpolate_Type(pub i16);

impl eGP_Interpolate_Type {
    pub const GP_IPO_LINEAR: Self = Self((0) as i16);
    pub const GP_IPO_CURVEMAP: Self = Self((1) as i16);
    pub const GP_IPO_BACK: Self = Self((3) as i16);
    pub const GP_IPO_BOUNCE: Self = Self((4) as i16);
    pub const GP_IPO_CIRC: Self = Self((5) as i16);
    pub const GP_IPO_CUBIC: Self = Self((6) as i16);
    pub const GP_IPO_ELASTIC: Self = Self((7) as i16);
    pub const GP_IPO_EXPO: Self = Self((8) as i16);
    pub const GP_IPO_QUAD: Self = Self((9) as i16);
    pub const GP_IPO_QUART: Self = Self((10) as i16);
    pub const GP_IPO_QUINT: Self = Self((11) as i16);
    pub const GP_IPO_SINE: Self = Self((12) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurvePaint_Flag(pub i8);

impl eCurvePaint_Flag {
    pub const CURVE_PAINT_FLAG_CORNERS_DETECT: Self = Self(((1 << 0)) as i8);
    pub const CURVE_PAINT_FLAG_PRESSURE_RADIUS: Self = Self(((1 << 1)) as i8);
    pub const CURVE_PAINT_FLAG_DEPTH_STROKE_ENDPOINTS: Self = Self(((1 << 2)) as i8);
    pub const CURVE_PAINT_FLAG_DEPTH_STROKE_OFFSET_ABS: Self = Self(((1 << 3)) as i8);
    pub const CURVE_PAINT_FLAG_DEPTH_ONLY_SELECTED: Self = Self(((1 << 4)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurvePaint_FitMethod(pub i8);

impl eCurvePaint_FitMethod {
    pub const CURVE_PAINT_FIT_METHOD_REFIT: Self = Self((0) as i8);
    pub const CURVE_PAINT_FIT_METHOD_SPLIT: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurvePaint_DepthMode(pub i8);

impl eCurvePaint_DepthMode {
    pub const CURVE_PAINT_PROJECT_CURSOR: Self = Self((0) as i8);
    pub const CURVE_PAINT_PROJECT_SURFACE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurvePaint_SurfacePlane(pub i8);

impl eCurvePaint_SurfacePlane {
    pub const CURVE_PAINT_SURFACE_PLANE_NORMAL_VIEW: Self = Self((0) as i8);
    pub const CURVE_PAINT_SURFACE_PLANE_NORMAL_SURFACE: Self = Self((1) as i8);
    pub const CURVE_PAINT_SURFACE_PLANE_VIEW: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eCurvePaint_AutoMerge(pub i8);

impl eCurvePaint_AutoMerge {
    pub const AUTO_MERGE: Self = Self((1 << 0) as i8);
    pub const AUTO_MERGE_AND_SPLIT: Self = Self((1 << 1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSeqOverlapMode(pub i32);

impl eSeqOverlapMode {
    pub const SEQ_OVERLAP_EXPAND: Self = Self(0 as i32);
    pub const SEQ_OVERLAP_OVERWRITE: Self = Self(1 as i32);
    pub const SEQ_OVERLAP_SHUFFLE: Self = Self(2 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSequencerSnapMode(pub i16);

impl eSequencerSnapMode {
    pub const SEQ_SNAP_TO_STRIPS: Self = Self((1 << 0) as i16);
    pub const SEQ_SNAP_TO_CURRENT_FRAME: Self = Self((1 << 1) as i16);
    pub const SEQ_SNAP_TO_STRIP_HOLD: Self = Self((1 << 2) as i16);
    pub const SEQ_SNAP_TO_MARKERS: Self = Self((1 << 3) as i16);
    pub const SEQ_SNAP_TO_PREVIEW_BORDERS: Self = Self((1 << 4) as i16);
    pub const SEQ_SNAP_TO_PREVIEW_CENTER: Self = Self((1 << 5) as i16);
    pub const SEQ_SNAP_TO_STRIPS_PREVIEW: Self = Self((1 << 6) as i16);
    pub const SEQ_SNAP_TO_RETIMING: Self = Self((1 << 7) as i16);
    pub const SEQ_SNAP_TO_INCREMENT: Self = Self((1 << 8) as i16);
    pub const SEQ_SNAP_TO_FRAME_RANGE: Self = Self((1 << 9) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSequencerSnapFlag(pub i16);

impl eSequencerSnapFlag {
    pub const SEQ_SNAP_IGNORE_MUTED: Self = Self((1 << 0) as i16);
    pub const SEQ_SNAP_IGNORE_SOUND: Self = Self((1 << 1) as i16);
    pub const SEQ_SNAP_CURRENT_FRAME_TO_STRIPS: Self = Self((1 << 2) as i16);
    pub const SEQ_SNAP_TO_ALL_CHANNEL_STRIPS: Self = Self((1 << 3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_TransformFlag(pub i32);

impl eTool_TransformFlag {
    pub const SCE_XFORM_AXIS_ALIGN: Self = Self(((1 << 0)) as i32);
    pub const SCE_XFORM_DATA_ORIGIN: Self = Self(((1 << 1)) as i32);
    pub const SCE_XFORM_SKIP_CHILDREN: Self = Self(((1 << 2)) as i32);
    pub const SCE_XFORM_SCULPT_PIVOT: Self = Self(((1 << 3)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_ObjectFlag(pub i32);

impl eTool_ObjectFlag {
    pub const SCE_OBJECT_MODE_LOCK: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_WorkspaceToolFlag(pub i32);

impl eTool_WorkspaceToolFlag {
    pub const SCE_WORKSPACE_TOOL_FALLBACK: Self = Self((0) as i32);
    pub const SCE_WORKSPACE_TOOL_DEFAULT: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSnapFlag(pub i16);

impl eSnapFlag {
    pub const SCE_SNAP: Self = Self(((1 << 0)) as i16);
    pub const SCE_SNAP_ROTATE: Self = Self(((1 << 1)) as i16);
    pub const SCE_SNAP_PEEL_OBJECT: Self = Self(((1 << 2)) as i16);
    pub const SCE_SNAP_NOT_TO_ACTIVE: Self = Self(((1 << 4)) as i16);
    pub const SCE_SNAP_ABS_GRID: Self = Self(((1 << 5)) as i16);
    pub const SCE_SNAP_ABS_TIME_STEP: Self = Self(((1 << 5)) as i16);
    pub const SCE_SNAP_BACKFACE_CULLING: Self = Self(((1 << 6)) as i16);
    pub const SCE_SNAP_KEEP_ON_SAME_OBJECT: Self = Self(((1 << 7)) as i16);
    pub const SCE_SNAP_TO_INCLUDE_EDITED: Self = Self(((1 << 8)) as i16);
    pub const SCE_SNAP_TO_INCLUDE_NONEDITED: Self = Self(((1 << 9)) as i16);
    pub const SCE_SNAP_TO_ONLY_SELECTABLE: Self = Self(((1 << 10)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSnapSourceOP(pub i8);

impl eSnapSourceOP {
    pub const SCE_SNAP_SOURCE_CLOSEST: Self = Self((0) as i8);
    pub const SCE_SNAP_SOURCE_CENTER: Self = Self((1) as i8);
    pub const SCE_SNAP_SOURCE_MEDIAN: Self = Self((2) as i8);
    pub const SCE_SNAP_SOURCE_ACTIVE: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSnapTargetOP(pub i16);

impl eSnapTargetOP {
    pub const SCE_SNAP_TARGET_ALL: Self = Self((0) as i16);
    pub const SCE_SNAP_TARGET_NOT_SELECTED: Self = Self(((1 << 0)) as i16);
    pub const SCE_SNAP_TARGET_NOT_ACTIVE: Self = Self(((1 << 1)) as i16);
    pub const SCE_SNAP_TARGET_NOT_EDITED: Self = Self(((1 << 2)) as i16);
    pub const SCE_SNAP_TARGET_ONLY_SELECTABLE: Self = Self(((1 << 3)) as i16);
    pub const SCE_SNAP_TARGET_NOT_NONEDITED: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSnapMode(pub i16);

impl eSnapMode {
    pub const SCE_SNAP_TO_NONE: Self = Self((0) as i16);
    pub const SCE_SNAP_TO_FRAME: Self = Self(((1 << 0)) as i16);
    pub const SCE_SNAP_TO_SECOND: Self = Self(((1 << 1)) as i16);
    pub const SCE_SNAP_TO_MARKERS: Self = Self(((1 << 2)) as i16);
    pub const SCE_SNAP_TO_KEYS: Self = Self(((1 << 3)) as i16);
    pub const SCE_SNAP_TO_STRIPS: Self = Self(((1 << 4)) as i16);
    pub const SCE_SNAP_TO_POINT: Self = Self(((1 << 0)) as i16);
    pub const SCE_SNAP_TO_EDGE_MIDPOINT: Self = Self(((1 << 1)) as i16);
    pub const SCE_SNAP_TO_EDGE_ENDPOINT: Self = Self(((1 << 2)) as i16);
    pub const SCE_SNAP_TO_EDGE_PERPENDICULAR: Self = Self(((1 << 3)) as i16);
    pub const SCE_SNAP_TO_EDGE: Self = Self(((1 << 4)) as i16);
    pub const SCE_SNAP_TO_FACE: Self = Self(((1 << 5)) as i16);
    pub const SCE_SNAP_TO_VOLUME: Self = Self(((1 << 6)) as i16);
    pub const SCE_SNAP_TO_GRID: Self = Self(((1 << 7)) as i16);
    pub const SCE_SNAP_TO_INCREMENT: Self = Self(((1 << 8)) as i16);
    pub const SCE_SNAP_INDIVIDUAL_NEAREST: Self = Self(((1 << 9)) as i16);
    pub const SCE_SNAP_INDIVIDUAL_PROJECT: Self = Self(((1 << 10)) as i16);
    pub const SCE_SNAP_TO_FACE_MIDPOINT: Self = Self(((1 << 11)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_UvSculptFlag(pub i32);

impl eTool_UvSculptFlag {
    pub const UV_SCULPT_LOCK_BORDERS: Self = Self((1) as i32);
    pub const UV_SCULPT_ALL_ISLANDS: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGpPaint_Flag(pub i32);

impl eGpPaint_Flag {
    pub const GPPAINT_FLAG_USE_MATERIAL: Self = Self((0) as i32);
    pub const GPPAINT_FLAG_USE_VERTEXCOLOR: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eVPaint_Flag(pub i8);

impl eVPaint_Flag {
    pub const VP_FLAG_VGROUP_RESTRICT: Self = Self(0 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSnapTransformMode(pub i8);

impl eSnapTransformMode {
    pub const SCE_SNAP_TRANSFORM_MODE_TRANSLATE: Self = Self(((1 << 0)) as i8);
    pub const SCE_SNAP_TRANSFORM_MODE_ROTATE: Self = Self(((1 << 1)) as i8);
    pub const SCE_SNAP_TRANSFORM_MODE_SCALE: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_SelectMode(pub i32);

impl eTool_SelectMode {
    pub const SCE_SELECT_VERTEX: Self = Self((1 << 0) as i32);
    pub const SCE_SELECT_EDGE: Self = Self((1 << 1) as i32);
    pub const SCE_SELECT_FACE: Self = Self((1 << 2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eMeshStatVis_Type(pub i8);

impl eMeshStatVis_Type {
    pub const SCE_STATVIS_OVERHANG: Self = Self((0) as i8);
    pub const SCE_STATVIS_THICKNESS: Self = Self((1) as i8);
    pub const SCE_STATVIS_INTERSECT: Self = Self((2) as i8);
    pub const SCE_STATVIS_DISTORT: Self = Self((3) as i8);
    pub const SCE_STATVIS_SHARP: Self = Self((4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_ProportionalFalloff(pub i8);

impl eTool_ProportionalFalloff {
    pub const PROP_SMOOTH: Self = Self((0) as i8);
    pub const PROP_SPHERE: Self = Self((1) as i8);
    pub const PROP_ROOT: Self = Self((2) as i8);
    pub const PROP_SHARP: Self = Self((3) as i8);
    pub const PROP_LIN: Self = Self((4) as i8);
    pub const PROP_CONST: Self = Self((5) as i8);
    pub const PROP_RANDOM: Self = Self((6) as i8);
    pub const PROP_INVSQUARE: Self = Self((7) as i8);
    pub const PROP_MODE_MAX: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_ProportionalEdit(pub i8);

impl eTool_ProportionalEdit {
    pub const PROP_EDIT_USE: Self = Self(((1 << 0)) as i8);
    pub const PROP_EDIT_CONNECTED: Self = Self(((1 << 1)) as i8);
    pub const PROP_EDIT_PROJECTED: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_WeightUser(pub i8);

impl eTool_WeightUser {
    pub const OB_DRAW_GROUPUSER_NONE: Self = Self((0) as i8);
    pub const OB_DRAW_GROUPUSER_ACTIVE: Self = Self((1) as i8);
    pub const OB_DRAW_GROUPUSER_ALL: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_Unwrapper(pub i8);

impl eTool_Unwrapper {
    pub const UVCALC_UNWRAP_METHOD_ANGLE: Self = Self((0) as i8);
    pub const UVCALC_UNWRAP_METHOD_CONFORMAL: Self = Self((1) as i8);
    pub const UVCALC_UNWRAP_METHOD_MINIMUM_STRETCH: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_UvCalcFlag(pub i8);

impl eTool_UvCalcFlag {
    pub const UVCALC_FILLHOLES: Self = Self((1 << 0) as i8);
    pub const UVCALC_NO_ASPECT_CORRECT: Self = Self((1 << 1) as i8);
    pub const UVCALC_TRANSFORM_CORRECT_SLIDE: Self = Self((1 << 2) as i8);
    pub const UVCALC_USESUBSURF: Self = Self((1 << 3) as i8);
    pub const UVCALC_TRANSFORM_CORRECT: Self = Self((1 << 4) as i8);
    pub const UVCALC_TRANSFORM_CORRECT_KEEP_CONNECTED: Self = Self((1 << 5) as i8);
    pub const UVCALC_UNWRAP_NO_FLIP: Self = Self((1 << 6) as i8);
    pub const UVCALC_UNWRAP_USE_WEIGHTS: Self = Self((1 << 7) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_UvFlag(pub i8);

impl eTool_UvFlag {
    pub const UV_FLAG_SELECT_SYNC: Self = Self((1 << 0) as i8);
    pub const UV_FLAG_SHOW_SAME_IMAGE: Self = Self((1 << 1) as i8);
    pub const UV_FLAG_SELECT_ISLAND: Self = Self((1 << 2) as i8);
    pub const UV_FLAG_CUSTOM_REGION: Self = Self((1 << 3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_UvSelectMode(pub i8);

impl eTool_UvSelectMode {
    pub const UV_SELECT_VERT: Self = Self((1 << 0) as i8);
    pub const UV_SELECT_EDGE: Self = Self((1 << 1) as i8);
    pub const UV_SELECT_FACE: Self = Self((1 << 2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eTool_UvSticky(pub i8);

impl eTool_UvSticky {
    pub const UV_STICKY_LOCATION: Self = Self((0) as i8);
    pub const UV_STICKY_DISABLE: Self = Self((1) as i8);
    pub const UV_STICKY_VERT: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGPencil_Flags(pub i8);

impl eGPencil_Flags {
    pub const GP_USE_MULTI_FRAME_EDITING: Self = Self(((1 << 0)) as i8);
    pub const GP_TOOL_FLAG_RETAIN_LAST: Self = Self(((1 << 1)) as i8);
    pub const GP_TOOL_FLAG_PAINT_ONBACK: Self = Self(((1 << 2)) as i8);
    pub const GP_TOOL_FLAG_THUMBNAIL_LIST: Self = Self(((1 << 3)) as i8);
    pub const GP_TOOL_FLAG_CREATE_WEIGHTS: Self = Self(((1 << 4)) as i8);
    pub const GP_TOOL_FLAG_AUTOMERGE_STROKE: Self = Self(((1 << 5)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGPencil_SimplifyFlags(pub i16);

impl eGPencil_SimplifyFlags {
    pub const SIMPLIFY_GPENCIL_ENABLE: Self = Self(((1 << 0)) as i16);
    pub const SIMPLIFY_GPENCIL_ON_PLAY: Self = Self(((1 << 1)) as i16);
    pub const SIMPLIFY_GPENCIL_FILL: Self = Self(((1 << 2)) as i16);
    pub const SIMPLIFY_GPENCIL_MODIFIER: Self = Self(((1 << 3)) as i16);
    pub const SIMPLIFY_GPENCIL_FX: Self = Self(((1 << 5)) as i16);
    pub const SIMPLIFY_GPENCIL_TINT: Self = Self(((1 << 7)) as i16);
    pub const SIMPLIFY_GPENCIL_AA: Self = Self(((1 << 8)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGPencil_Placement_Flags(pub i8);

impl eGPencil_Placement_Flags {
    pub const GP_PROJECT_VIEWSPACE: Self = Self(((1 << 0)) as i8);
    pub const GP_PROJECT_DEPTH_VIEW: Self = Self(((1 << 2)) as i8);
    pub const GP_PROJECT_DEPTH_STROKE: Self = Self(((1 << 3)) as i8);
    pub const GP_PROJECT_DEPTH_STROKE_ENDPOINTS: Self = Self(((1 << 4)) as i8);
    pub const GP_PROJECT_CURSOR: Self = Self(((1 << 5)) as i8);
    pub const GP_PROJECT_DEPTH_STROKE_FIRST: Self = Self(((1 << 6)) as i8);
    pub const GP_PROJECT_DEPTH_ONLY_SELECTED: Self = Self(6 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGPencil_Selectmode_types(pub i8);

impl eGPencil_Selectmode_types {
    pub const GP_SELECTMODE_POINT: Self = Self((0) as i8);
    pub const GP_SELECTMODE_STROKE: Self = Self((1) as i8);
    pub const GP_SELECTMODE_SEGMENT: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGPencil_GuideTypes(pub i8);

impl eGPencil_GuideTypes {
    pub const GP_GUIDE_CIRCULAR: Self = Self((0) as i8);
    pub const GP_GUIDE_RADIAL: Self = Self((1) as i8);
    pub const GP_GUIDE_PARALLEL: Self = Self((2) as i8);
    pub const GP_GUIDE_GRID: Self = Self((3) as i8);
    pub const GP_GUIDE_ISO: Self = Self((4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGPencil_Guide_Reference(pub i8);

impl eGPencil_Guide_Reference {
    pub const GP_GUIDE_REF_CURSOR: Self = Self((0) as i8);
    pub const GP_GUIDE_REF_CUSTOM: Self = Self((1) as i8);
    pub const GP_GUIDE_REF_OBJECT: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eUnit_System(pub i8);

impl eUnit_System {
    pub const USER_UNIT_NONE: Self = Self((0) as i8);
    pub const USER_UNIT_METRIC: Self = Self((1) as i8);
    pub const USER_UNIT_IMPERIAL: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eUnit_Flag(pub i8);

impl eUnit_Flag {
    pub const USER_UNIT_OPT_SPLIT: Self = Self((1) as i8);
    pub const USER_UNIT_ROT_RADIANS_DEPRECATED: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eUnit_RotationSystem(pub i8);

impl eUnit_RotationSystem {
    pub const USER_UNIT_ROT_DEGREES: Self = Self((0) as i8);
    pub const USER_UNIT_ROT_RADIANS: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ePhysics_Flag(pub i32);

impl ePhysics_Flag {
    pub const PHYS_GLOBAL_GRAVITY: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSceneDisplay_AA(pub i8);

impl eSceneDisplay_AA {
    pub const SCE_DISPLAY_AA_OFF: Self = Self((0) as i8);
    pub const SCE_DISPLAY_AA_FXAA: Self = Self((1) as i8);
    pub const SCE_DISPLAY_AA_SAMPLES_5: Self = Self((5) as i8);
    pub const SCE_DISPLAY_AA_SAMPLES_8: Self = Self((8) as i8);
    pub const SCE_DISPLAY_AA_SAMPLES_11: Self = Self((11) as i8);
    pub const SCE_DISPLAY_AA_SAMPLES_16: Self = Self((16) as i8);
    pub const SCE_DISPLAY_AA_SAMPLES_32: Self = Self((32) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RaytraceEEVEE_Flag(pub i32);

impl RaytraceEEVEE_Flag {
    pub const RAYTRACE_EEVEE_USE_DENOISE: Self = Self(((1 << 0)) as i32);
    pub const RAYTRACE_EEVEE_USE_BACKFACE: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RaytraceEEVEE_DenoiseStages(pub i32);

impl RaytraceEEVEE_DenoiseStages {
    pub const RAYTRACE_EEVEE_DENOISE_SPATIAL: Self = Self(((1 << 0)) as i32);
    pub const RAYTRACE_EEVEE_DENOISE_TEMPORAL: Self = Self(((1 << 1)) as i32);
    pub const RAYTRACE_EEVEE_DENOISE_BILATERAL: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RaytraceEEVEE_Method(pub i32);

impl RaytraceEEVEE_Method {
    pub const RAYTRACE_EEVEE_METHOD_PROBE: Self = Self((0) as i32);
    pub const RAYTRACE_EEVEE_METHOD_SCREEN: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSceneEEVEE_Flag(pub i32);

impl eSceneEEVEE_Flag {
    pub const SCE_EEVEE_VOLUMETRIC_SHADOWS: Self = Self(((1 << 2)) as i32);
    pub const SCE_EEVEE_GTAO_ENABLED: Self = Self(((1 << 4)) as i32);
    pub const SCE_EEVEE_MOTION_BLUR_ENABLED_DEPRECATED: Self = Self(((1 << 9)) as i32);
    pub const SCE_EEVEE_TAA_REPROJECTION: Self = Self(((1 << 11)) as i32);
    pub const SCE_EEVEE_SSR_ENABLED: Self = Self(((1 << 14)) as i32);
    pub const SCE_EEVEE_GI_AUTOBAKE: Self = Self(((1 << 19)) as i32);
    pub const SCE_EEVEE_OVERSCAN: Self = Self(((1 << 21)) as i32);
    pub const SCE_EEVEE_DOF_JITTER: Self = Self(((1 << 23)) as i32);
    pub const SCE_EEVEE_SHADOW_ENABLED: Self = Self(((1 << 24)) as i32);
    pub const SCE_EEVEE_RAYTRACE_OPTIONS_SPLIT: Self = Self(((1 << 25)) as i32);
    pub const SCE_EEVEE_SHADOW_JITTERED_VIEWPORT: Self = Self(((1 << 26)) as i32);
    pub const SCE_EEVEE_VOLUME_CUSTOM_RANGE: Self = Self(((1 << 27)) as i32);
    pub const SCE_EEVEE_FAST_GI_ENABLED: Self = Self(((1 << 28)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FastGI_Method(pub i8);

impl FastGI_Method {
    pub const FAST_GI_FULL: Self = Self((0) as i8);
    pub const FAST_GI_AO_ONLY: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSceneHydra_ExportMethod(pub i32);

impl eSceneHydra_ExportMethod {
    pub const SCE_HYDRA_EXPORT_HYDRA: Self = Self((0) as i32);
    pub const SCE_HYDRA_EXPORT_USD: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScene_OrientationSlot(pub i32);

impl eScene_OrientationSlot {
    pub const SCE_ORIENT_DEFAULT: Self = Self((0) as i32);
    pub const SCE_ORIENT_TRANSLATE: Self = Self((1) as i32);
    pub const SCE_ORIENT_ROTATE: Self = Self((2) as i32);
    pub const SCE_ORIENT_SCALE: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SceneCompositorEffectFlags(pub u8);

impl SceneCompositorEffectFlags {
    pub const None: Self = Self((0) as u8);
    pub const EnableForRender: Self = Self(((1 << 0)) as u8);
    pub const EnableForPreview: Self = Self(((1 << 1)) as u8);
    pub const IsActive: Self = Self(((1 << 2)) as u8);
    pub const ShowNodeGroupSelector: Self = Self(((1 << 3)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScene_Flag(pub i32);

impl eScene_Flag {
    pub const SCE_DS_SELECTED: Self = Self((1 << 0) as i32);
    pub const SCE_DS_COLLAPSED: Self = Self((1 << 1) as i32);
    pub const SCE_NLA_EDIT_ON: Self = Self((1 << 2) as i32);
    pub const SCE_FRAME_DROP: Self = Self((1 << 3) as i32);
    pub const SCE_KEYS_NO_SELONLY: Self = Self((1 << 4) as i32);
    pub const SCE_READFILE_LIBLINK_NEED_SETSCENE_CHECK: Self = Self((1 << 5) as i32);
    pub const SCE_CUSTOM_SIMULATION_RANGE: Self = Self((1 << 6) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScenePlaybackLoopMode(pub u8);

impl eScenePlaybackLoopMode {
    pub const SCE_LOOP_MODE_INFINITE: Self = Self((0) as u8);
    pub const SCE_LOOP_MODE_STOP_END_FRAME: Self = Self((1) as u8);
    pub const SCE_LOOP_MODE_STOP_START_FRAME: Self = Self((2) as u8);
    pub const SCE_LOOP_MODE_RESTORE: Self = Self((3) as u8);
    pub const SCE_LOOP_MODE_BOUNCE: Self = Self((4) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eScene_IterFlag(pub i32);

impl eScene_IterFlag {
    pub const F_START: Self = Self((0) as i32);
    pub const F_SCENE: Self = Self((1) as i32);
    pub const F_DUPLI: Self = Self((3) as i32);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AudioData {
    pub mixrate: i32,
    pub main: f32,
    pub speed_of_sound: f32,
    pub doppler_factor: f32,
    pub distance_model: i32,
    pub flag: i16,
    pub _pad: [u8; 2],
}

impl Default for AudioData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SceneRenderLayer {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
}

impl Default for SceneRenderLayer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SceneRenderView {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub suffix: [u8; 64],
    pub viewflag: eSceneView_Flag,
}

impl Default for SceneRenderView {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Stereo3dFormat {
    pub flag: eStereo3dFlag,
}

impl Default for Stereo3dFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImageFormatData {
    pub media_type: MediaType,
    pub imtype: i8,
    pub depth: eImageFormatDepth,
    pub color_mode: ImColorMode,
    pub flag: i8,
    pub quality: i8,
    pub compress: i8,
    pub exr_codec: i8,
    pub exr_flag: i8,
    pub jp2_flag: i8,
    pub jp2_codec: i8,
    pub tiff_codec: i8,
    pub cineon_flag: i8,
    pub _pad: [u8; 3],
}

impl Default for ImageFormatData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BakeData {
    pub im_format: ImageFormatData,
    pub filepath: [u8; 1024],
    pub width: i16,
    pub height: i16,
    pub margin: i16,
    pub flag: i16,
    pub cage_extrusion: f32,
    pub max_ray_distance: f32,
    pub pass_filter: eBakePassFilter,
    pub normal_swizzle: [u8; 3],
    pub R_BAKE_POSY: i8,
    pub R_BAKE_POSZ: i8,
}

impl Default for BakeData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RenderData {
    pub ffcodecdata: FFMpegCodecData,
    pub cfra: i32,
    pub sfra: i32,
    pub efra: i32,
    pub subframe: f32,
    pub psfra: i32,
    pub pefra: i32,
    pub images: i32,
    pub framapto: i32,
    pub flag: i16,
    pub threads: i16,
    pub framelen: f32,
    pub frame_step: i32,
    pub dimensionspreset: i16,
    pub size: i16,
    pub xsch: i32,
    pub ysch: i32,
    pub use_lock_interface: i8,
    pub _pad7: [u8; 3],
}

impl Default for RenderData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TimeMarker {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub frame: i32,
    pub name: [u8; 64],
    pub flag: u32,
    pub camera: *mut core::ffi::c_void,
    pub prop: *mut core::ffi::c_void,
}

impl Default for TimeMarker {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UnifiedPaintSettings {
    pub unprojected_size: f32,
    pub alpha: f32,
    pub weight: f32,
    pub color: [f32; 3],
    pub _0: f32,
    pub _0_1: f32,
}

impl Default for UnifiedPaintSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NamedBrushAssetReference {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: *mut core::ffi::c_void,
    pub brush_asset_reference: *mut core::ffi::c_void,
}

impl Default for NamedBrushAssetReference {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ToolSystemBrushBindings {
    pub main_brush_asset_reference: *mut core::ffi::c_void,
    pub active_brush_per_brush_type: ListBaseT<NamedBrushAssetReference>,
    pub nullptr: ListBaseT<NamedBrushAssetReference>,
}

impl Default for ToolSystemBrushBindings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MeshAutomaskingSettings {
    pub boundary_edges_propagation_steps: i32,
    pub cavity_blur_steps: i32,
    pub cavity_factor: f32,
    pub start_normal_limit: f32,
    pub start_normal_falloff: f32,
    pub view_normal_limit: f32,
    pub view_normal_falloff: f32,
    pub cavity_curve: *mut core::ffi::c_void,
    pub cavity_curve_op: *mut core::ffi::c_void,
}

impl Default for MeshAutomaskingSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Paint {
    pub brush_asset_reference: *mut core::ffi::c_void,
    pub tool_brush_bindings: ToolSystemBrushBindings,
    pub palette: *mut core::ffi::c_void,
    pub cavity_curve: *mut core::ffi::c_void,
    pub flags: ePaintFlags,
    pub debug_flags: ePaintDebugFlags,
}

impl Default for Paint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ImagePaintSettings {
    pub paint: Paint,
    pub flag: eImagePaint_Flag,
}

impl Default for ImagePaintSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PaintModeSettings {
    pub canvas_source: ePaintCanvasSource,
    pub _pad: [u8; 7],
}

impl Default for PaintModeSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleBrushData {
    pub size: i16,
    pub step: i16,
    pub invert: i16,
    pub count: i16,
    pub flag: i32,
    pub strength: f32,
}

impl Default for ParticleBrushData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleEditSettings {
    pub flag: i16,
    pub totrekey: i16,
    pub totaddkey: i16,
    pub brushtype: i16,
    pub brush: [ParticleBrushData; 7],
    pub paintcursor: *mut core::ffi::c_void,
    pub emitterdist: f32,
    pub _pad0: [u8; 4],
}

impl Default for ParticleEditSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Sculpt {
    pub flags: eSculptFlags,
    pub transform_mode: eSculptTransformMode,
    pub radial_symm_legacy: [i32; 3],
}

impl Default for Sculpt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurvesSculpt {
    pub paint: Paint,
}

impl Default for CurvesSculpt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UvSculpt {
    pub curve_distance_falloff: *mut core::ffi::c_void,
    pub size: i32,
    pub strength: f32,
    pub curve_distance_falloff_preset: i8,
    pub _pad: [u8; 7],
}

impl Default for UvSculpt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GpPaint {
    pub paint: Paint,
    pub flag: i32,
    pub mode: i32,
}

impl Default for GpPaint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GpVertexPaint {
    pub paint: Paint,
    pub flag: i32,
    pub _pad: [u8; 4],
}

impl Default for GpVertexPaint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GpSculptPaint {
    pub paint: Paint,
    pub flag: i32,
    pub _pad: [u8; 4],
}

impl Default for GpSculptPaint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GpWeightPaint {
    pub paint: Paint,
    pub flag: i32,
    pub _pad: [u8; 4],
}

impl Default for GpWeightPaint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct VPaint {
    pub paint: Paint,
    pub flag: i8,
    pub _pad: [u8; 7],
}

impl Default for VPaint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GP_Sculpt_Guide {
    pub use_guide: i8,
    pub use_snapping: i8,
    pub reference_point: i8,
    pub r#type: i8,
    pub _pad2: [u8; 4],
}

impl Default for GP_Sculpt_Guide {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GP_Sculpt_Settings {
    pub paintcursor: *mut core::ffi::c_void,
    pub flag: eGP_Sculpt_SettingsFlag,
}

impl Default for GP_Sculpt_Settings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GP_Interpolate_Settings {
    pub custom_ipo: *mut core::ffi::c_void,
}

impl Default for GP_Interpolate_Settings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurvePaintSettings {
    pub curve_type: i8,
    pub flag: i8,
    pub depth_mode: i8,
    pub surface_plane: i8,
    pub fit_method: i8,
    pub _pad: i8,
}

impl Default for CurvePaintSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MeshStatVis {
    pub r#type: i8,
    pub _pad1: [u8; 2],
}

impl Default for MeshStatVis {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SequencerToolSettings {
    pub fit_method: eSeqImageFitMethod,
    pub snap_mode: eSequencerSnapMode,
}

impl Default for SequencerToolSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ToolSettings {
    pub wpaint: *mut core::ffi::c_void,
    pub sculpt: *mut core::ffi::c_void,
    pub uvsculpt: UvSculpt,
    pub gp_paint: *mut core::ffi::c_void,
    pub gp_vertexpaint: *mut core::ffi::c_void,
    pub gp_sculptpaint: *mut core::ffi::c_void,
    pub gp_weightpaint: *mut core::ffi::c_void,
    pub curves_sculpt: *mut core::ffi::c_void,
    pub vgroup_weight: f32,
    pub doublimit: f32,
    pub automerge: i8,
    pub object_flag: i8,
    pub selectmode: i8,
    pub unwrapper: eTool_Unwrapper,
    pub uvcalc_flag: eTool_UvCalcFlag,
    pub uv_flag: eTool_UvFlag,
    pub uv_selectmode: eTool_UvSelectMode,
    pub uv_sticky: eTool_UvSticky,
    pub uv_custom_region: rctf,
}

impl Default for ToolSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UnitSettings {
    pub scale_length: f32,
    pub system: i8,
    pub system_rotation: i8,
    pub flag: i16,
    pub length_unit: i8,
    pub mass_unit: i8,
    pub time_unit: i8,
    pub temperature_unit: i8,
    pub _pad: [u8; 4],
}

impl Default for UnitSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PhysicsSettings {
    pub gravity: [f32; 3],
    pub _0: f32,
    pub _9: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DisplaySafeAreas {
    pub title: [f32; 2],
    pub _5: f32,
}

impl Default for DisplaySafeAreas {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SceneDisplay {
    pub light_direction: [f32; 3],
    pub M_SQRT1_3: f32,
    pub M_SQRT1_3_1: f32,
}

impl Default for SceneDisplay {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RaytraceEEVEE {
    pub screen_trace_quality: f32,
    pub screen_trace_thickness: f32,
    pub trace_max_roughness: f32,
    pub resolution_scale: i32,
    pub flag: RaytraceEEVEE_Flag,
    pub denoise_stages: RaytraceEEVEE_DenoiseStages,
    pub backface_radiance_scale: f32,
    pub _pad: [u8; 4],
}

impl Default for RaytraceEEVEE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SceneEEVEE {
    pub gi_diffuse_bounces: i32,
    pub gi_cubemap_resolution: i32,
    pub gi_visibility_resolution: i32,
    pub gi_glossy_clamp: f32,
    pub gi_irradiance_pool_size: i32,
    pub _pad0: [u8; 4],
}

impl Default for SceneEEVEE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SceneGpencil {
    pub smaa_threshold: f32,
    pub smaa_threshold_render: f32,
    pub aa_samples: i32,
    pub motion_blur_steps: i32,
}

impl Default for SceneGpencil {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SceneHydra {
    pub export_method: eSceneHydra_ExportMethod,
    pub _pad0: i32,
}

impl Default for SceneHydra {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TransformOrientationSlot {
    pub r#type: i32,
    pub index_custom: i32,
    pub flag: i8,
    pub _pad0: [u8; 7],
}

impl Default for TransformOrientationSlot {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SceneCompositorEffect {
    pub next: *mut core::ffi::c_void,
    pub previous: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub node_group: *mut core::ffi::c_void,
    pub system_properties: *mut core::ffi::c_void,
    pub flags: SceneCompositorEffectFlags,
    pub _pad0: [u8; 1],
}

impl Default for SceneCompositorEffect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Scene {
    pub adt: *mut core::ffi::c_void,
    pub camera: *mut core::ffi::c_void,
    pub world: *mut core::ffi::c_void,
    pub set: *mut core::ffi::c_void,
    pub base: ListBaseT<Base>,
    pub nullptr: ListBaseT<Base>,
}

impl Default for Scene {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

