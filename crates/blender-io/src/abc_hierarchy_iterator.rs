//! Auto-transpiled C/C++ header module: abc_hierarchy_iterator

use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ABCWriterConstructorArgs {
    pub depsgraph: *mut Depsgraph,
    pub abc_archive: *mut ABCArchive,
    pub hierarchy_iterator: *mut ABCHierarchyIterator,
    pub export_params: *mut AlembicExportParams,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Depsgraph {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Main {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ABCAbstractWriter {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ABCHierarchyIterator {
    pub _opaque: [u8; 0],
}
