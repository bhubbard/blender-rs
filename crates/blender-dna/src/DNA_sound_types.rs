//! Auto-transpiled C/C++ header module: DNA_sound_types

use core::ffi::c_void;
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct bSound {
    pub id: ID,
    pub filepath: [i8; 1024],
    pub packedfile: *mut PackedFile,
    pub newpackedfile: *mut PackedFile,
    pub _pad0: *mut core::ffi::c_void,
    pub offset_time: f64,
    pub volume: f32,
    pub attenuation: f32,
    pub pitch: f32,
    pub min_gain: f32,
    pub max_gain: f32,
    pub distance: f32,
    pub audio_channels: i32,
    pub samplerate: i32,
    pub flags: eSound_Flag,
    pub stream_index: i16,
    pub _pad1: [i8; 4],
    pub _pad2: *mut core::ffi::c_void,
}

impl Default for bSound {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SoundRuntime {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PackedFile {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSound_Flag {
    SOUND_FLAGS_3D = (1 << 3),
    SOUND_FLAGS_CACHING = (1 << 4),
    SOUND_FLAGS_MONO = (1 << 5),
}

impl Default for eSound_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SOUND_FLAGS_3D: i32 = eSound_Flag::SOUND_FLAGS_3D as i32;
pub const SOUND_FLAGS_CACHING: i32 = eSound_Flag::SOUND_FLAGS_CACHING as i32;
pub const SOUND_FLAGS_MONO: i32 = eSound_Flag::SOUND_FLAGS_MONO as i32;

