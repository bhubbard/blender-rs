//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeTreeInterfaceItemType(pub i8);

impl NodeTreeInterfaceItemType {
    pub const Panel: Self = Self((0) as i8);
    pub const Socket: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeTreeInterfaceSocketFlag(pub i32);

impl NodeTreeInterfaceSocketFlag {
    pub const NODE_INTERFACE_SOCKET_INPUT: Self = Self((1 << 0) as i32);
    pub const NODE_INTERFACE_SOCKET_OUTPUT: Self = Self((1 << 1) as i32);
    pub const NODE_INTERFACE_SOCKET_HIDE_VALUE: Self = Self((1 << 2) as i32);
    pub const NODE_INTERFACE_SOCKET_HIDE_IN_MODIFIER: Self = Self((1 << 3) as i32);
    pub const NODE_INTERFACE_SOCKET_COMPACT: Self = Self((1 << 4) as i32);
    pub const NODE_INTERFACE_SOCKET_SINGLE_VALUE_ONLY_LEGACY: Self = Self((1 << 5) as i32);
    pub const NODE_INTERFACE_SOCKET_LAYER_SELECTION: Self = Self((1 << 6) as i32);
    pub const NODE_INTERFACE_SOCKET_INSPECT: Self = Self((1 << 7) as i32);
    pub const NODE_INTERFACE_SOCKET_PANEL_TOGGLE: Self = Self((1 << 8) as i32);
    pub const NODE_INTERFACE_SOCKET_MENU_EXPANDED: Self = Self((1 << 9) as i32);
    pub const NODE_INTERFACE_SOCKET_OPTIONAL_LABEL: Self = Self((1 << 10) as i32);
    pub const NODE_INTERFACE_SOCKET_SELECT: Self = Self((1 << 11) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeSocketInterfaceStructureType(pub i8);

impl NodeSocketInterfaceStructureType {
    pub const Auto: Self = Self((0) as i8);
    pub const Single: Self = Self((1) as i8);
    pub const Dynamic: Self = Self((2) as i8);
    pub const Field: Self = Self((3) as i8);
    pub const Grid: Self = Self((4) as i8);
    pub const List: Self = Self((5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StructureType(pub i8);

impl StructureType {
    pub const Single: Self = Self(0 as i8);
    pub const Dynamic: Self = Self(1 as i8);
    pub const Field: Self = Self(2 as i8);
    pub const Grid: Self = Self(3 as i8);
    pub const List: Self = Self(4 as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeTreeInterfacePanelFlag(pub i32);

impl NodeTreeInterfacePanelFlag {
    pub const NODE_INTERFACE_PANEL_DEFAULT_CLOSED: Self = Self((1 << 0) as i32);
    pub const NODE_INTERFACE_PANEL_ALLOW_CHILD_PANELS_LEGACY: Self = Self((1 << 1) as i32);
    pub const NODE_INTERFACE_PANEL_ALLOW_SOCKETS_AFTER_PANELS: Self = Self((1 << 2) as i32);
    pub const NODE_INTERFACE_PANEL_IS_COLLAPSED: Self = Self((1 << 3) as i32);
    pub const NODE_INTERFACE_PANEL_SELECT: Self = Self((1 << 4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodeDefaultInputType(pub i16);

impl NodeDefaultInputType {
    pub const NODE_DEFAULT_INPUT_VALUE: Self = Self((0) as i16);
    pub const NODE_DEFAULT_INPUT_INDEX_FIELD: Self = Self((1) as i16);
    pub const NODE_DEFAULT_INPUT_ID_INDEX_FIELD: Self = Self((2) as i16);
    pub const NODE_DEFAULT_INPUT_NORMAL_FIELD: Self = Self((3) as i16);
    pub const NODE_DEFAULT_INPUT_POSITION_FIELD: Self = Self((4) as i16);
    pub const NODE_DEFAULT_INPUT_INSTANCE_TRANSFORM_FIELD: Self = Self((5) as i16);
    pub const NODE_DEFAULT_INPUT_HANDLE_LEFT_FIELD: Self = Self((6) as i16);
    pub const NODE_DEFAULT_INPUT_HANDLE_RIGHT_FIELD: Self = Self((7) as i16);
    pub const NODE_DEFAULT_INPUT_SCENE_FRAME: Self = Self((8) as i16);
    pub const NODE_DEFAULT_INPUT_UNIFORM_IMAGE_COORDINATES: Self = Self((9) as i16);
    pub const NODE_DEFAULT_INPUT_SELF_OBJECT: Self = Self((10) as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeTreeInterfaceItem {
    pub item_type: NodeTreeInterfaceItemType,
    pub _pad: [u8; 7],
}

impl Default for bNodeTreeInterfaceItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeTreeInterfaceSocket {
    pub item: bNodeTreeInterfaceItem,
    pub name_: *mut core::ffi::c_void,
    pub description_: *mut core::ffi::c_void,
    pub socket_type: *mut core::ffi::c_void,
    pub flag: NodeTreeInterfaceSocketFlag,
}

impl Default for bNodeTreeInterfaceSocket {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct bNodeTreeInterfacePanel {
    pub item: bNodeTreeInterfaceItem,
    pub name_: *mut core::ffi::c_void,
    pub description_: *mut core::ffi::c_void,
    pub flag: NodeTreeInterfacePanelFlag,
}

impl Default for bNodeTreeInterfacePanel {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

