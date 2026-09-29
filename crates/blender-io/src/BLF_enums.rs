//! Auto-transpiled C/C++ header module: BLF_enums

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontShadowType {
    None = 0,
    Blur3x3 = 3,
    Blur5x5 = 5,
    Outline = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BLFWrapMode {
    Minimal = 0,
    Typographical = 1 << 0,
    Path = 1 << 1,
    HardLimit = 1 << 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFlags {
    BLF_NONE = 0,
    BLF_ROTATION = 1 << 0,
    BLF_CLIPPING = 1 << 1,
    BLF_SHADOW = 1 << 2,
    BLF_ASPECT = 1 << 5,
    BLF_WORD_WRAP = 1 << 6,
    BLF_MONOCHROME = 1 << 7,
    BLF_HINTING_NONE = 1 << 8,
    BLF_HINTING_SLIGHT = 1 << 9,
    BLF_HINTING_FULL = 1 << 10,
    BLF_BOLD = 1 << 11,
    BLF_ITALIC = 1 << 12,
    BLF_MONOSPACED = 1 << 13,
    BLF_DEFAULT = 1 << 14,
    BLF_LAST_RESORT = 1 << 15,
    BLF_BAD_FONT = 1 << 16,
    BLF_RENDER_SUBPIXELAA = 1 << 18,
    BLF_NO_FALLBACK = 1 << 19,
}
