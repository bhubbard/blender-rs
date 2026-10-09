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
pub struct ContainerDefRNA {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub cont: *mut core::ffi::c_void,
    pub properties: ListBaseT<PropertyDefRNA>,
}

impl Default for ContainerDefRNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FunctionDefRNA {
    pub cont: ContainerDefRNA,
    pub func: *mut core::ffi::c_void,
    pub srna: *mut core::ffi::c_void,
    pub call: *mut core::ffi::c_void,
    pub gencall: *mut core::ffi::c_void,
}

impl Default for FunctionDefRNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct PropertyDefRNA {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub cont: *mut core::ffi::c_void,
    pub prop: *mut core::ffi::c_void,
    pub dnastructname: String,
    pub dnastructfromname: String,
    pub dnastructfromprop: String,
    pub dnaname: String,
    pub dnatype: String,
    pub dnaarraylength: i32,
    pub dnapointerlevel: i32,
    pub dnadefaultdata: *mut core::ffi::c_void,
    pub dnalengthstructname: String,
    pub dnalengthname: String,
    pub dnalengthfixed: i32,
    pub booleanbit: i64,
    pub booleannegative: bool,
    pub enumbitflags: i32,
}

impl Default for PropertyDefRNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct StructDefRNA {
    pub cont: ContainerDefRNA,
    pub srna: *mut core::ffi::c_void,
    pub filename: String,
    pub dnaname: String,
    pub dnafromname: String,
    pub dnafromprop: String,
    pub functions: ListBaseT<FunctionDefRNA>,
}

impl Default for StructDefRNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct AllocDefRNA {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub mem: *mut core::ffi::c_void,
}

impl Default for AllocDefRNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BlenderDefRNA {
    pub laststruct: *mut core::ffi::c_void,
    pub error: bool,
    pub silent: bool,
    pub verify: bool,
    pub animate: bool,
    pub make_overridable: bool,
    pub structs: ListBaseT<StructDefRNA>,
}

impl Default for BlenderDefRNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

