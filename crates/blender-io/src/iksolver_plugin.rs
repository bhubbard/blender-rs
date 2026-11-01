/* SPDX-FileCopyrightText: 2001-2002 NaN Holding BV. All rights reserved.
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

use blender_dna::{bPose, bPoseChannel, Object, Scene};
use crate::DEG_depsgraph::Depsgraph;

extern "C" {
    pub fn iksolver_initialize_tree(
        depsgraph: *mut Depsgraph,
        scene: *mut Scene,
        ob: *mut Object,
        ctime: f32,
    );
    pub fn iksolver_execute_tree(
        depsgraph: *mut Depsgraph,
        scene: *mut Scene,
        ob: *mut Object,
        pchan_root: *mut bPoseChannel,
        ctime: f32,
    );
    pub fn iksolver_release_tree(scene: *mut Scene, ob: *mut Object, ctime: f32);
    pub fn iksolver_clear_data(pose: *mut bPose);
}
