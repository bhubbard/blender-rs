//! Auto-transpiled C/C++ header module: DNA_nla_types

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bActionModifier {
    pub r#type: i16,
    pub channel: [i8; 32],
    pub noisesize: f32,
    pub channels: i16,
    pub no_rot_axis: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct bActionStrip {

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eActStrip_Mode {
    ACTSTRIPMODE_BLEND = 0,
    ACTSTRIPMODE_ADD = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eActStrip_Flag {
    ACTSTRIP_SELECT = (1 << 0),
    ACTSTRIP_USESTRIDE = (1 << 1),
    ACTSTRIP_HOLDLASTFRAME = (1 << 3),
    ACTSTRIP_ACTIVE = (1 << 4),
    ACTSTRIP_LOCK_ACTION = (1 << 5),
    ACTSTRIP_MUTE = (1 << 6),
    ACTSTRIP_REVERSE = (1 << 7),
    ACTSTRIP_AUTO_BLENDS = (1 << 11),
}
