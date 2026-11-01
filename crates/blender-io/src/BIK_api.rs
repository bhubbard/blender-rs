/* SPDX-FileCopyrightText: 2001-2002 NaN Holding BV. All rights reserved.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use blender_dna::{bConstraint, bPose, bPoseChannel, Object, Scene};
use crate::DEG_depsgraph::Depsgraph;

extern "C" {
    pub fn BIK_init_tree(
        depsgraph: *mut Depsgraph,
        scene: *mut Scene,
        ob: *mut Object,
        ctime: f32,
    );
    pub fn BIK_execute_tree(
        depsgraph: *mut Depsgraph,
        scene: *mut Scene,
        ob: *mut Object,
        pchan: *mut bPoseChannel,
        ctime: f32,
    );
    pub fn BIK_release_tree(scene: *mut Scene, ob: *mut Object, ctime: f32);
    pub fn BIK_clear_data(pose: *mut bPose);
    pub fn BIK_clear_cache(pose: *mut bPose);
    pub fn BIK_update_param(pose: *mut bPose);
    pub fn BIK_test_constraint(ob: *mut Object, cons: *mut bConstraint);
}
