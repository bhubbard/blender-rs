//! Auto-transpiled C/C++ header module: NOD_nested_node_id

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FoundNestedNodeID {
    pub id: i32,
    pub is_in_simulation: bool,
    pub is_in_loop: bool,
    pub is_in_closure: bool,
}
