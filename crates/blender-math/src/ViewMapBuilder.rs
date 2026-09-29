//! Auto-transpiled C/C++ header module: ViewMapBuilder

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum intersection_algo {
    sweep_line,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum visibility_algo {
    ray_casting,
    ray_casting_fast,
    ray_casting_very_fast,
    ray_casting_culled_adaptive_traditional,
    ray_casting_adaptive_traditional,
    ray_casting_culled_adaptive_cumulative,
    ray_casting_adaptive_cumulative,
}
