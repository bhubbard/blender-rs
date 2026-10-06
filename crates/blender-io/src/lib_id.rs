use crate::*;

pub enum ID {
    ID_INVALID,
    ID_VALID,
}

pub const ID_SORT_STEP_SIZE: usize = 512;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IDCopyLibManagementData {
    pub id_src: *mut ID,
    pub id_dst: *mut ID,
    pub flag: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SomeTypeWithIDMember {
    pub id: i32,
}
