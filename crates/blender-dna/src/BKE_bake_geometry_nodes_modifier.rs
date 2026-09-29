//! Auto-transpiled C/C++ header module: BKE_bake_geometry_nodes_modifier

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FrameCache {
    pub frame: SubFrame,
    pub values: BakeValues,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrevCache {
    pub values: BakeValues,
    pub frame: SubFrame,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeBakeCache {
    pub failed_finding_bake: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SimulationNodeCache {
    pub bake: NodeBakeCache,
    pub cache_status: CacheStatus,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeNodeCache {
    pub bake: NodeBakeCache,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ModifierCache {
    pub mutex: mutable Mutex,
    pub requested_bakes: Set<int>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheStatus {
    Valid,
    Invalid,
    Baked,
}
