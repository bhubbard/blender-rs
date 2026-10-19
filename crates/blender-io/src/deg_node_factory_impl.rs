//! Auto-transpiled C/C++ header module: deg_node_factory_impl
//! Upstream: source/blender/depsgraph/intern/node/deg_node_factory_impl.hh

use crate::*;
use std::ffi::c_char;
use std::marker::PhantomData;

pub struct DepsNodeFactoryImpl<T> {
    pub type_info: TypeInfo,
    _phantom: PhantomData<T>,
}

impl<T> DepsNodeFactoryImpl<T> {
    pub fn new(type_info: TypeInfo) -> Self {
        Self {
            type_info,
            _phantom: PhantomData,
        }
    }
}

impl<T: Default> DepsNodeFactory for DepsNodeFactoryImpl<T> {
    fn type_(&self) -> NodeType {
        self.type_info.node_type
    }

    fn type_name(&self) -> *const c_char {
        self.type_info.type_name
    }

    fn id_recalc_tag(&self) -> i32 {
        self.type_info.id_recalc_tag
    }

    fn create_node(&self, _id: *const ID, _subdata: *const c_char, name: &str) -> Box<DepsgraphNode> {
        Box::new(DepsgraphNode {
            name: name.to_string(),
            type_: self.type_info.node_type,
            ..Default::default()
        })
    }
}
