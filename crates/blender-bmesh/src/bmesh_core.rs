//! Auto-transpiled C/C++ header module: bmesh_core

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BMeshElemErrorFlag {
    IS_NULL = (1 << 0),
    IS_WRONG_TYPE = (1 << 1),
    IS_VERT_WRONG_EDGE_TYPE = (1 << 2),
    IS_EDGE_NULL_DISK_LINK = (1 << 3),
    IS_EDGE_WRONG_LOOP_TYPE = (1 << 4),
    IS_EDGE_WRONG_FACE_TYPE = (1 << 5),
    IS_EDGE_NULL_RADIAL_LINK = (1 << 6),
    IS_EDGE_ZERO_FACE_LENGTH = (1 << 7),
    IS_LOOP_WRONG_FACE_TYPE = (1 << 8),
    IS_LOOP_WRONG_EDGE_TYPE = (1 << 9),
    IS_LOOP_WRONG_VERT_TYPE = (1 << 10),
    IS_LOOP_VERT_NOT_IN_EDGE = (1 << 11),
    IS_LOOP_NULL_CYCLE_LINK = (1 << 12),
    IS_LOOP_ZERO_FACE_LENGTH = (1 << 13),
    IS_LOOP_WRONG_FACE_LENGTH = (1 << 14),
    IS_LOOP_WRONG_RADIAL_LENGTH = (1 << 15),
    IS_FACE_NULL_LOOP = (1 << 16),
    IS_FACE_WRONG_LOOP_FACE = (1 << 17),
    IS_FACE_NULL_EDGE = (1 << 18),
    IS_FACE_NULL_VERT = (1 << 19),
    IS_FACE_LOOP_VERT_NOT_IN_EDGE = (1 << 20),
    IS_FACE_LOOP_WRONG_RADIAL_LENGTH = (1 << 21),
    IS_FACE_LOOP_WRONG_DISK_LENGTH = (1 << 22),
    IS_FACE_LOOP_DUPE_LOOP = (1 << 23),
    IS_FACE_LOOP_DUPE_VERT = (1 << 24),
    IS_FACE_LOOP_DUPE_EDGE = (1 << 25),
    IS_FACE_WRONG_LENGTH = (1 << 26),
}
