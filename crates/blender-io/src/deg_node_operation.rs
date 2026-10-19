//! Auto-transpiled C/C++ header module: deg_node_operation
//! Upstream: source/blender/depsgraph/intern/node/deg_node_operation.hh

use crate::*;
use crate::DEG_depsgraph::Depsgraph;
use std::ffi::c_void;

pub type DepsEvalOperationCb = Option<unsafe extern "C" fn(*mut Depsgraph)>;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationCode {
    OPERATION = 0,
    ID_PROPERTY,
    PARAMETERS_ENTRY,
    PARAMETERS_EVAL,
    PARAMETERS_EXIT,
    VISIBILITY,
    HIERARCHY,
    ANIMATION_ENTRY,
    ANIMATION_EVAL,
    ANIMATION_EXIT,
    DRIVER,
    DRIVER_UNSHARE,
    SCENE_EVAL,
    AUDIO_ENTRY,
    AUDIO_VOLUME,
    OBJECT_FROM_LAYER_ENTRY,
    OBJECT_BASE_FLAGS,
    OBJECT_FROM_LAYER_EXIT,
    DIMENSIONS,
    TRANSFORM_INIT,
    TRANSFORM_LOCAL,
    TRANSFORM_PARENT,
    TRANSFORM_CONSTRAINTS,
    TRANSFORM_EVAL,
    TRANSFORM_SIMULATION_INIT,
    TRANSFORM_FINAL,
    RIGIDBODY_REBUILD,
    RIGIDBODY_SIM,
    RIGIDBODY_TRANSFORM_COPY,
    GEOMETRY_EVAL_INIT,
    MODIFIER,
    GEOMETRY_EVAL,
    GEOMETRY_EVAL_DONE,
    GEOMETRY_SHAPEKEY,
    LIGHT_PROBE_EVAL,
    SPEAKER_EVAL,
    SOUND_EVAL,
    ARMATURE_EVAL,
    POSE_INIT,
    POSE_INIT_IK,
    POSE_CLEANUP,
    POSE_DONE,
    POSE_IK_SOLVER,
    POSE_SPLINE_IK_SOLVER,
    BONE_LOCAL,
    BONE_POSE_PARENT,
    BONE_CONSTRAINTS,
    BONE_READY,
    BONE_DONE,
    BONE_SEGMENTS,
    BONE_VISIBILITY,
    PARTICLE_SYSTEM_INIT,
    PARTICLE_SYSTEM_EVAL,
    PARTICLE_SYSTEM_DONE,
    PARTICLE_SETTINGS_INIT,
    PARTICLE_SETTINGS_EVAL,
    PARTICLE_SETTINGS_RESET,
    POINT_CACHE_RESET,
    FILE_CACHE_UPDATE,
    VIEW_LAYER_EVAL,
    COPY_ON_EVAL,
    SHADING,
    SHADING_DONE,
    MATERIAL_UPDATE,
    LIGHT_UPDATE,
    WORLD_UPDATE,
    LIGHT_LINKING_UPDATE,
    NTREE_OUTPUT,
    NTREE_GEOMETRY_PREPROCESS,
    GEOMETRY_SELECT_UPDATE,
    MASK_ANIMATION,
    MASK_EVAL,
    MOVIECLIP_EVAL,
    IMAGE_ANIMATION,
    SYNCHRONIZE_TO_ORIGINAL,
    GENERIC_DATABLOCK_UPDATE,
    SEQUENCES_EVAL,
    COMPOSITOR_EVAL,
    INSTANCER,
    INSTANCE,
    INSTANCE_GEOMETRY,
    NUM_OPERATIONS,
}

impl Default for OperationCode {
    fn default() -> Self {
        OperationCode::OPERATION
    }
}

pub const DEG_OP_FLAG_USER_MODIFIED: u32 = 1 << 0;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct OperationNode {
    pub base: DepsgraphNode,
    pub opcode: OperationCode,
    pub name_tag: i32,
    pub flag: u32,
    pub operation: DepsEvalOperationCb,
    pub owner: *mut c_void,
}

impl Default for OperationNode {
    fn default() -> Self {
        Self {
            base: DepsgraphNode {
                type_: NodeType::OPERATION,
                ..Default::default()
            },
            opcode: OperationCode::OPERATION,
            name_tag: -1,
            flag: 0,
            operation: None,
            owner: core::ptr::null_mut(),
        }
    }
}

impl OperationNode {
    pub fn new(opcode: OperationCode, name: &str, name_tag: i32) -> Self {
        Self {
            base: DepsgraphNode {
                name: name.to_string(),
                type_: NodeType::OPERATION,
                ..Default::default()
            },
            opcode,
            name_tag,
            flag: 0,
            operation: None,
            owner: core::ptr::null_mut(),
        }
    }
}
