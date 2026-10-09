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
#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type UString = String;
#[allow(non_camel_case_types)]
type PropertyFlag = u32;
#[allow(non_camel_case_types)]
type PropertyOverrideFlag = u32;
#[allow(non_camel_case_types)]
type ParameterFlag = u32;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CineonFileHeader {
    pub magic_num: u32,
    pub offset: u32,
    pub gen_hdr_size: u32,
    pub ind_hdr_size: u32,
    pub user_data_size: u32,
    pub file_size: u32,
    pub version: [u8; 8],
    pub file_name: [u8; 100],
    pub creation_date: [u8; 12],
    pub creation_time: [u8; 12],
    pub reserved: [u8; 36],
}

impl Default for CineonFileHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CineonElementHeader {
    pub descriptor1: u8,
    pub descriptor2: u8,
    pub bits_per_sample: u8,
    pub filler: u8,
    pub pixels_per_line: u32,
    pub lines_per_image: u32,
    pub ref_low_data: u32,
    pub ref_low_quantity: f32,
    pub ref_high_data: u32,
    pub ref_high_quantity: f32,
}

impl Default for CineonElementHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CineonImageHeader {
    pub orientation: u8,
    pub elements_per_image: u8,
    pub filler: u16,
    pub element: [CineonElementHeader; 8],
    pub white_point_x: f32,
    pub white_point_y: f32,
    pub red_primary_x: f32,
    pub red_primary_y: f32,
    pub green_primary_x: f32,
    pub green_primary_y: f32,
    pub blue_primary_x: f32,
    pub blue_primary_y: f32,
    pub label: [u8; 200],
    pub reserved: [u8; 28],
    pub interleave: u8,
    pub packing: u8,
    pub data_sign: u8,
    pub sense: u8,
    pub line_padding: u32,
    pub element_padding: u32,
    pub reserved2: [u8; 20],
}

impl Default for CineonImageHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CineonOriginationHeader {
    pub x_offset: i32,
    pub y_offset: i32,
    pub file_name: [u8; 100],
    pub creation_date: [u8; 12],
    pub creation_time: [u8; 12],
    pub input_device: [u8; 64],
    pub model_number: [u8; 32],
    pub input_serial_number: [u8; 32],
    pub x_input_samples_per_mm: f32,
    pub y_input_samples_per_mm: f32,
    pub input_device_gamma: f32,
    pub reserved: [u8; 40],
}

impl Default for CineonOriginationHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CineonFilmHeader {
    pub film_code: u8,
    pub film_type: u8,
    pub edge_code_perforation_offset: u8,
    pub filler: u8,
    pub prefix: u32,
    pub count: u32,
    pub format: [u8; 32],
    pub frame_position: u32,
    pub frame_rate: f32,
    pub attribute: [u8; 32],
    pub slate: [u8; 200],
    pub reserved: [u8; 740],
}

impl Default for CineonFilmHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CineonMainHeader {
    pub fileHeader: CineonFileHeader,
    pub imageHeader: CineonImageHeader,
    pub originationHeader: CineonOriginationHeader,
    pub filmHeader: CineonFilmHeader,
}

impl Default for CineonMainHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

