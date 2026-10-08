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

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FModifier {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub curve: *mut core::ffi::c_void,
    pub data: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub r#type: eFModifier_Types,
}

impl Default for FModifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_Generator {
    pub coefficients: *mut core::ffi::c_void,
    pub arraysize: u32,
    pub poly_order: i32,
    pub mode: eFMod_Generator_Modes,
}

impl Default for FMod_Generator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_FunctionGenerator {
    pub amplitude: f32,
    pub phase_multiplier: f32,
    pub phase_offset: f32,
    pub value_offset: f32,
    pub r#type: eFMod_Generator_Functions,
}

impl Default for FMod_FunctionGenerator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FCM_EnvelopeData {
    pub min: f32,
    pub max: f32,
    pub time: f32,
    pub f1: i16,
    pub f2: i16,
}

impl Default for FCM_EnvelopeData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_Envelope {
    pub data: *mut core::ffi::c_void,
    pub totvert: i32,
    pub midval: f32,
    pub min: f32,
    pub max: f32,
}

impl Default for FMod_Envelope {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_Cycles {
    pub before_mode: eFMod_Cycling_Modes,
}

impl Default for FMod_Cycles {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_Limits {
    pub rect: rctf,
}

impl Default for FMod_Limits {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_Noise {
    pub size: f32,
    pub strength: f32,
    pub phase: f32,
    pub offset: f32,
    pub roughness: f32,
    pub lacunarity: f32,
    pub depth: i16,
    pub modification: eFMod_Noise_Modifications,
}

impl Default for FMod_Noise {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_Stepped {
    pub step_size: f32,
    pub offset: f32,
    pub start_frame: f32,
    pub end_frame: f32,
    pub flag: eFMod_Stepped_Flags,
}

impl Default for FMod_Stepped {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FMod_Smooth {
    pub sigma: f32,
    pub filter_width: i32,
}

impl Default for FMod_Smooth {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DriverTarget {
    pub id: *mut core::ffi::c_void,
    pub rna_path: *mut core::ffi::c_void,
    pub pchan_name: [u8; 64],
    pub transChan: eDriverTarget_TransformChannels,
}

impl Default for DriverTarget {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DriverVar {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub targets: [DriverTarget; 8],
    pub num_targets: i8,
    pub r#type: eDriverVar_Types,
}

impl Default for DriverVar {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ChannelDriver {
    pub variables: ListBaseT<DriverVar>,
    pub nullptr: ListBaseT<DriverVar>,
}

impl Default for ChannelDriver {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FPoint {
    pub vec: [f32; 2],
    pub flag: i32,
    pub _pad: [u8; 4],
}

impl Default for FPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FCurve {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub grp: *mut core::ffi::c_void,
    pub driver: *mut core::ffi::c_void,
    pub modifiers: ListBaseT<FModifier>,
    pub nullptr: ListBaseT<FModifier>,
}

impl Default for FCurve {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NlaStrip {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub strips: ListBaseT<NlaStrip>,
    pub nullptr: ListBaseT<NlaStrip>,
}

impl Default for NlaStrip {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NlaTrack {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub strips: ListBaseT<NlaStrip>,
    pub nullptr: ListBaseT<NlaStrip>,
}

impl Default for NlaTrack {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct KS_Path {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub id: *mut core::ffi::c_void,
    pub group: [u8; 64],
    pub idtype: i32,
    pub groupmode: eKSP_Grouping,
}

impl Default for KS_Path {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct KeyingSet {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub paths: ListBaseT<KS_Path>,
    pub nullptr: ListBaseT<KS_Path>,
}

impl Default for KeyingSet {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AnimOverride {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub rna_path: *mut core::ffi::c_void,
    pub array_index: i32,
    pub value: f32,
}

impl Default for AnimOverride {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AnimData {
    pub action: *mut core::ffi::c_void,
    pub slot_handle: i32,
    pub last_slot_identifier: [u8; 258],
    pub _pad0: [u8; 2],
}

impl Default for AnimData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct IdAdtTemplate {
    pub id: ID,
    pub adt: *mut core::ffi::c_void,
}

impl Default for IdAdtTemplate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

