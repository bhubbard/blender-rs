//! Auto-transpiled C/C++ header module: ANIM_keyframing

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleKeyingResult {
    SUCCESS = 0,
    UNKNOWN_FAILURE,
    CANNOT_CREATE_FCURVE,
    FCURVE_NOT_KEYFRAMEABLE,
    NO_KEY_NEEDED,
    UNABLE_TO_INSERT_TO_NLA_STACK,
    ID_NOT_EDITABLE,
    ID_NOT_ANIMATABLE,
    NO_VALID_LAYER,
    NO_VALID_STRIP,
    NO_VALID_SLOT,
    CANNOT_RESOLVE_PATH,
    _KEYING_RESULT_MAX,
}
