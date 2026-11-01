/* SPDX-FileCopyrightText: 2001-2002 NaN Holding BV. All rights reserved.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use blender_dna::{bConstraint, bPose, bPoseChannel, Object, Scene};
use crate::DEG_depsgraph::Depsgraph;

#[repr(C)]
pub struct IKPlugin {
    pub initialize_tree_func: Option<
        unsafe extern "C" fn(
            depsgraph: *mut Depsgraph,
            scene: *mut Scene,
            ob: *mut Object,
            ctime: f32,
        ),
    >,
    pub execute_tree_func: Option<
        unsafe extern "C" fn(
            depsgraph: *mut Depsgraph,
            scene: *mut Scene,
            ob: *mut Object,
            pchan: *mut bPoseChannel,
            ctime: f32,
        ),
    >,
    pub release_tree_func:
        Option<unsafe extern "C" fn(scene: *mut Scene, ob: *mut Object, ctime: f32)>,
    pub remove_armature_func: Option<unsafe extern "C" fn(pose: *mut bPose)>,
    pub clear_cache: Option<unsafe extern "C" fn(pose: *mut bPose)>,
    pub update_param: Option<unsafe extern "C" fn(pose: *mut bPose)>,
    pub test_constraint: Option<unsafe extern "C" fn(ob: *mut Object, cons: *mut bConstraint)>,
}
