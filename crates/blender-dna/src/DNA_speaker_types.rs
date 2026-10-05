//! Auto-transpiled C/C++ header module: DNA_speaker_types

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Speaker {
    pub id: ID,
    pub adt: *mut AnimData,
    pub sound: *mut bSound,
    pub volume_max: f32,
    pub volume_min: f32,
    pub distance_max: f32,
    pub distance_reference: f32,
    pub attenuation: f32,
    pub cone_angle_outer: f32,
    pub cone_angle_inner: f32,
    pub cone_volume_outer: f32,
    pub volume: f32,
    pub pitch: f32,
    pub flag: eSpeaker_Flag,
    pub _pad1: [i8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimData {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bSound {
    pub _opaque: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSpeaker_Flag {
    SPK_DS_EXPAND = 1 << 0,
    SPK_MUTED = 1 << 1,
}

impl Default for eSpeaker_Flag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const SPK_DS_EXPAND: i32 = eSpeaker_Flag::SPK_DS_EXPAND as i32;
pub const SPK_MUTED: i32 = eSpeaker_Flag::SPK_MUTED as i32;

