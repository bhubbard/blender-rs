//! Auto-transpiled C/C++ header module: BKE_cloth

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClothHairData {
    pub loc: [f32; 3],
    pub rest_target: [f32; 3],
    pub radius: f32,
    pub bending_stiffness: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClothSolverResult {
    pub status: i32,
    pub avg_iterations: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cloth {
    pub numsprings: u32,
    pub mvert_num: u32,
    pub primitive_num: u32,
    pub old_solver_type: u8,
    pub pad2: u8,
    pub pad3: i16,
    pub last_frame: i32,
    pub initial_mesh_volume: f32,
    pub average_acceleration: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClothVertex {
    pub flags: i32,
    pub v: [f32; 3],
    pub xconst: [f32; 3],
    pub x: [f32; 3],
    pub xold: [f32; 3],
    pub tx: [f32; 3],
    pub txold: [f32; 3],
    pub tv: [f32; 3],
    pub mass: f32,
    pub goal: f32,
    pub impulse: [f32; 3],
    pub xrest: [f32; 3],
    pub dcvel: [f32; 3],
    pub impulse_count: u32,
    pub avg_spring_len: f32,
    pub struct_stiff: f32,
    pub bend_stiff: f32,
    pub shear_stiff: f32,
    pub spring_count: i32,
    pub shrink_factor: f32,
    pub internal_stiff: f32,
    pub pressure_factor: f32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClothSpring {
    pub ij: i32,
    pub kl: i32,
    pub mn: i32,
    pub la: i32,
    pub lb: i32,
    pub restlen: f32,
    pub restang: f32,
    pub r#type: i32,
    pub flags: i32,
    pub lin_stiffness: f32,
    pub ang_stiffness: f32,
    pub editrestlen: f32,
    pub target: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColliderContacts {
    pub totcollisions: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum eClothVertexFlag {
    CLOTH_VERT_FLAG_PINNED = (1 << 0),
    CLOTH_VERT_FLAG_NOSELFCOLL = (1 << 1),
    CLOTH_VERT_FLAG_NOOBJCOLL = (1 << 2),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLOTH_SPRING_TYPES {
    CLOTH_SPRING_TYPE_STRUCTURAL = (1 << 1),
    CLOTH_SPRING_TYPE_SHEAR = (1 << 2),
    CLOTH_SPRING_TYPE_BENDING = (1 << 3),
    CLOTH_SPRING_TYPE_GOAL = (1 << 4),
    CLOTH_SPRING_TYPE_SEWING = (1 << 5),
    CLOTH_SPRING_TYPE_BENDING_HAIR = (1 << 6),
    CLOTH_SPRING_TYPE_INTERNAL = (1 << 7),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CLOTH_SPRINGS_FLAGS {
    CLOTH_SPRING_FLAG_DEACTIVATE = (1 << 1),
    CLOTH_SPRING_FLAG_NEEDED = (1 << 2),
}
