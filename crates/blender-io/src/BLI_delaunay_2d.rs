//! Auto-transpiled C/C++ header module: BLI_delaunay_2d

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CDT_output_type {
    CDT_FULL,
    CDT_INSIDE,
    CDT_INSIDE_WITH_HOLES,
    CDT_INSIDE_WITH_HOLES_NONZERO,
    CDT_CONSTRAINTS,
    CDT_CONSTRAINTS_VALID_BMESH,
    CDT_CONSTRAINTS_VALID_BMESH_WITH_HOLES,
    CDT_CONSTRAINTS_VALID_BMESH_WITH_HOLES_NONZERO,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CDT_ids_needed_type {
    CDT_NO_ORIG_IDS = 0,
    CDT_ORIG_VERTS = (1 << 0),
    CDT_INTERSECTED_EDGES = (1 << 1),
    CDT_ORIG_EDGES = (1 << 2),
    CDT_ORIG_FACES = (1 << 3),
    CDT_CW_ORIG_FACES = (1 << 4),
    CDT_ONLY_ONE_ORIG = (1 << 5),
}
