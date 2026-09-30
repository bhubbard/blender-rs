//! Auto-transpiled C/C++ header module: DNA_ID_enums

use crate::*;

pub const IDP_NUMTYPES: i32 = 11;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eIconSizes {
    ICON_SIZE_ICON = 0,
    ICON_SIZE_PREVIEW = 1,
    NUM_ICON_SIZES,
}

impl Default for eIconSizes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ICON_SIZE_ICON: i32 = eIconSizes::ICON_SIZE_ICON as i32;
pub const ICON_SIZE_PREVIEW: i32 = eIconSizes::ICON_SIZE_PREVIEW as i32;
pub const NUM_ICON_SIZES: i32 = eIconSizes::NUM_ICON_SIZES as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eIDPropertyType {
    IDP_STRING = 0,
    IDP_INT = 1,
    IDP_FLOAT = 2,
    IDP_ARRAY = 5,
    IDP_GROUP = 6,
    IDP_ID = 7,
    IDP_DOUBLE = 8,
    IDP_IDPARRAY = 9,
    IDP_BOOLEAN = 10,
}

impl Default for eIDPropertyType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IDP_STRING: i32 = eIDPropertyType::IDP_STRING as i32;
pub const IDP_INT: i32 = eIDPropertyType::IDP_INT as i32;
pub const IDP_FLOAT: i32 = eIDPropertyType::IDP_FLOAT as i32;
pub const IDP_ARRAY: i32 = eIDPropertyType::IDP_ARRAY as i32;
pub const IDP_GROUP: i32 = eIDPropertyType::IDP_GROUP as i32;
pub const IDP_ID: i32 = eIDPropertyType::IDP_ID as i32;
pub const IDP_DOUBLE: i32 = eIDPropertyType::IDP_DOUBLE as i32;
pub const IDP_IDPARRAY: i32 = eIDPropertyType::IDP_IDPARRAY as i32;
pub const IDP_BOOLEAN: i32 = eIDPropertyType::IDP_BOOLEAN as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eIDPropertyTypeFilter {
    IDP_TYPE_FILTER_STRING = 1 << IDP_STRING,
    IDP_TYPE_FILTER_INT = 1 << IDP_INT,
    IDP_TYPE_FILTER_FLOAT = 1 << IDP_FLOAT,
    IDP_TYPE_FILTER_ARRAY = 1 << IDP_ARRAY,
    IDP_TYPE_FILTER_GROUP = 1 << IDP_GROUP,
    IDP_TYPE_FILTER_ID = 1 << IDP_ID,
    IDP_TYPE_FILTER_DOUBLE = 1 << IDP_DOUBLE,
    IDP_TYPE_FILTER_IDPARRAY = 1 << IDP_IDPARRAY,
    IDP_TYPE_FILTER_BOOLEAN = 1 << IDP_BOOLEAN,
}

impl Default for eIDPropertyTypeFilter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IDP_TYPE_FILTER_STRING: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_STRING as i32;
pub const IDP_TYPE_FILTER_INT: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_INT as i32;
pub const IDP_TYPE_FILTER_FLOAT: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_FLOAT as i32;
pub const IDP_TYPE_FILTER_ARRAY: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_ARRAY as i32;
pub const IDP_TYPE_FILTER_GROUP: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_GROUP as i32;
pub const IDP_TYPE_FILTER_ID: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_ID as i32;
pub const IDP_TYPE_FILTER_DOUBLE: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_DOUBLE as i32;
pub const IDP_TYPE_FILTER_IDPARRAY: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_IDPARRAY as i32;
pub const IDP_TYPE_FILTER_BOOLEAN: i32 = eIDPropertyTypeFilter::IDP_TYPE_FILTER_BOOLEAN as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eIDPropertySubType {
    IDP_STRING_SUB_UTF8 = 0,
    IDP_STRING_SUB_BYTE = 1,
}

impl Default for eIDPropertySubType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IDP_STRING_SUB_UTF8: i32 = eIDPropertySubType::IDP_STRING_SUB_UTF8 as i32;
pub const IDP_STRING_SUB_BYTE: i32 = eIDPropertySubType::IDP_STRING_SUB_BYTE as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eIDPropertyFlag {
    IDP_FLAG_OVERRIDABLE_LIBRARY = 1 << 0,
    IDP_FLAG_OVERRIDELIBRARY_LOCAL = 1 << 1,
    IDP_FLAG_STATIC_TYPE = 1 << 4,
    IDP_FLAG_GHOST = 1 << 7,
}

impl Default for eIDPropertyFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const IDP_FLAG_OVERRIDABLE_LIBRARY: i32 = eIDPropertyFlag::IDP_FLAG_OVERRIDABLE_LIBRARY as i32;
pub const IDP_FLAG_OVERRIDELIBRARY_LOCAL: i32 = eIDPropertyFlag::IDP_FLAG_OVERRIDELIBRARY_LOCAL as i32;
pub const IDP_FLAG_STATIC_TYPE: i32 = eIDPropertyFlag::IDP_FLAG_STATIC_TYPE as i32;
pub const IDP_FLAG_GHOST: i32 = eIDPropertyFlag::IDP_FLAG_GHOST as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryFlag {
    LIBRARY_FLAG_IS_ARCHIVE = 1 << 0,
    LIBRARY_FLAG_IS_EXTERNAL = 1 << 1,
}

impl Default for LibraryFlag {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const LIBRARY_FLAG_IS_ARCHIVE: i32 = LibraryFlag::LIBRARY_FLAG_IS_ARCHIVE as i32;
pub const LIBRARY_FLAG_IS_EXTERNAL: i32 = LibraryFlag::LIBRARY_FLAG_IS_EXTERNAL as i32;

#[inline]
pub const fn MAKE_ID2(c: char, d: char) -> i32 {
    ((d as i32) << 8) | (c as i32)
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ID_Type {
    ID_SCE = MAKE_ID2('S', 'C'),
    ID_LI = MAKE_ID2('L', 'I'),
    ID_OB = MAKE_ID2('O', 'B'),
    ID_ME = MAKE_ID2('M', 'E'),
    ID_CU_LEGACY = MAKE_ID2('C', 'U'),
    ID_MB = MAKE_ID2('M', 'B'),
    ID_MA = MAKE_ID2('M', 'A'),
    ID_TE = MAKE_ID2('T', 'E'),
    ID_IM = MAKE_ID2('I', 'M'),
    ID_LT = MAKE_ID2('L', 'T'),
    ID_LA = MAKE_ID2('L', 'A'),
    ID_CA = MAKE_ID2('C', 'A'),
    ID_KE = MAKE_ID2('K', 'E'),
    ID_WO = MAKE_ID2('W', 'O'),
    ID_SCR = MAKE_ID2('S', 'R'),
    ID_VF = MAKE_ID2('V', 'F'),
    ID_TXT = MAKE_ID2('T', 'X'),
    ID_SPK = MAKE_ID2('S', 'K'),
    ID_SO = MAKE_ID2('S', 'O'),
    ID_GR = MAKE_ID2('G', 'R'),
    ID_AR = MAKE_ID2('A', 'R'),
    ID_AC = MAKE_ID2('A', 'C'),
    ID_NT = MAKE_ID2('N', 'T'),
    ID_BR = MAKE_ID2('B', 'R'),
    ID_PA = MAKE_ID2('P', 'A'),
    ID_GD_LEGACY = MAKE_ID2('G', 'D'),
    ID_WM = MAKE_ID2('W', 'M'),
    ID_MC = MAKE_ID2('M', 'C'),
    ID_MSK = MAKE_ID2('M', 'S'),
    ID_LS = MAKE_ID2('L', 'S'),
    ID_PAL = MAKE_ID2('P', 'L'),
    ID_PC = MAKE_ID2('P', 'C'),
    ID_CF = MAKE_ID2('C', 'F'),
    ID_WS = MAKE_ID2('W', 'S'),
    ID_LP = MAKE_ID2('L', 'P'),
    ID_CV = MAKE_ID2('C', 'V'),
    ID_PT = MAKE_ID2('P', 'T'),
    ID_VO = MAKE_ID2('V', 'O'),
    ID_GP = MAKE_ID2('G', 'P'),
}

impl Default for ID_Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub const ID_SCE: i32 = ID_Type::ID_SCE as i32;
pub const ID_LI: i32 = ID_Type::ID_LI as i32;
pub const ID_OB: i32 = ID_Type::ID_OB as i32;
pub const ID_ME: i32 = ID_Type::ID_ME as i32;
pub const ID_CU_LEGACY: i32 = ID_Type::ID_CU_LEGACY as i32;
pub const ID_MB: i32 = ID_Type::ID_MB as i32;
pub const ID_MA: i32 = ID_Type::ID_MA as i32;
pub const ID_TE: i32 = ID_Type::ID_TE as i32;
pub const ID_IM: i32 = ID_Type::ID_IM as i32;
pub const ID_LT: i32 = ID_Type::ID_LT as i32;
pub const ID_LA: i32 = ID_Type::ID_LA as i32;
pub const ID_CA: i32 = ID_Type::ID_CA as i32;
pub const ID_KE: i32 = ID_Type::ID_KE as i32;
pub const ID_WO: i32 = ID_Type::ID_WO as i32;
pub const ID_SCR: i32 = ID_Type::ID_SCR as i32;
pub const ID_VF: i32 = ID_Type::ID_VF as i32;
pub const ID_TXT: i32 = ID_Type::ID_TXT as i32;
pub const ID_SPK: i32 = ID_Type::ID_SPK as i32;
pub const ID_SO: i32 = ID_Type::ID_SO as i32;
pub const ID_GR: i32 = ID_Type::ID_GR as i32;
pub const ID_AR: i32 = ID_Type::ID_AR as i32;
pub const ID_AC: i32 = ID_Type::ID_AC as i32;
pub const ID_NT: i32 = ID_Type::ID_NT as i32;
pub const ID_BR: i32 = ID_Type::ID_BR as i32;
pub const ID_PA: i32 = ID_Type::ID_PA as i32;
pub const ID_GD_LEGACY: i32 = ID_Type::ID_GD_LEGACY as i32;
pub const ID_WM: i32 = ID_Type::ID_WM as i32;
pub const ID_MC: i32 = ID_Type::ID_MC as i32;
pub const ID_MSK: i32 = ID_Type::ID_MSK as i32;
pub const ID_LS: i32 = ID_Type::ID_LS as i32;
pub const ID_PAL: i32 = ID_Type::ID_PAL as i32;
pub const ID_PC: i32 = ID_Type::ID_PC as i32;
pub const ID_CF: i32 = ID_Type::ID_CF as i32;
pub const ID_WS: i32 = ID_Type::ID_WS as i32;
pub const ID_LP: i32 = ID_Type::ID_LP as i32;
pub const ID_CV: i32 = ID_Type::ID_CV as i32;
pub const ID_PT: i32 = ID_Type::ID_PT as i32;
pub const ID_VO: i32 = ID_Type::ID_VO as i32;
pub const ID_GP: i32 = ID_Type::ID_GP as i32;
