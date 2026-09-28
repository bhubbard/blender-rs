//! Auto-transpiled C/C++ header module: DNA_genfile

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSDNA_Type {
    SDNA_TYPE_CHAR = 0,
    SDNA_TYPE_UCHAR = 1,
    SDNA_TYPE_SHORT = 2,
    SDNA_TYPE_USHORT = 3,
    SDNA_TYPE_INT = 4,
    SDNA_TYPE_FLOAT = 7,
    SDNA_TYPE_DOUBLE = 8,
    SDNA_TYPE_INT64 = 9,
    SDNA_TYPE_UINT64 = 10,
    SDNA_TYPE_VOID = 11,
    SDNA_TYPE_INT8 = 12,
    SDNA_TYPE_RAW_DATA = 13,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eSDNA_StructCompare {
    SDNA_CMP_REMOVED = 0,
    SDNA_CMP_EQUAL = 1,
    SDNA_CMP_NOT_EQUAL = 2,
    SDNA_CMP_UNKNOWN = 3,
}
