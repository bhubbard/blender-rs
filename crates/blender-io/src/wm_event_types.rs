/* SPDX-FileCopyrightText: 2023-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Window manager event codes, modifier masks, and tablet input definitions.

pub const EVT_DATA_TIMER: i32 = 2;
pub const EVT_DATA_DRAGDROP: i32 = 3;
pub const EVT_DATA_NDOF_MOTION: i32 = 4;
pub const EVT_DATA_XR: i32 = 5;

pub const EVT_TABLET_NONE: i32 = 0;
pub const EVT_TABLET_STYLUS: i32 = 1;
pub const EVT_TABLET_ERASER: i32 = 2;

#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WmEventType {
    EventNone = 0x0000,

    /* Mouse buttons */
    LeftMouse = 0x0001,
    MiddleMouse = 0x0002,
    RightMouse = 0x0003,
    MouseMove = 0x0004,
    Button4Mouse = 0x0007,
    Button5Mouse = 0x0008,
    Button6Mouse = 0x0009,
    Button7Mouse = 0x000a,

    /* Mouse wheel */
    WheelUpMouse = 0x0011,
    WheelDownMouse = 0x0012,
    WheelInMouse = 0x0013,
    WheelOutMouse = 0x0014,

    /* Standard Keyboard keys */
    A = 0x0061,
    B = 0x0062,
    C = 0x0063,
    D = 0x0064,
    E = 0x0065,
    F = 0x0066,
    G = 0x0067,
    H = 0x0068,
    I = 0x0069,
    J = 0x006a,
    K = 0x006b,
    L = 0x006c,
    M = 0x006d,
    N = 0x006e,
    O = 0x006f,
    P = 0x0070,
    Q = 0x0071,
    R = 0x0072,
    S = 0x0073,
    T = 0x0074,
    U = 0x0075,
    V = 0x0076,
    W = 0x0077,
    X = 0x0078,
    Y = 0x0079,
    Z = 0x007a,

    EscKey = 0x00da,
    TabKey = 0x00db,
    ReturnKey = 0x00dc,
    SpaceKey = 0x00dd,
    BackSpaceKey = 0x00df,
    DelKey = 0x00e0,

    /* Modifiers */
    LeftShiftKey = 0x0130,
    RightShiftKey = 0x0131,
    LeftCtrlKey = 0x0132,
    RightCtrlKey = 0x0133,
    LeftAltKey = 0x0134,
    RightAltKey = 0x0135,
    OsKey = 0x0136,

    /* System events */
    Timer = 0x0111,
    WindowDeactivate = 0x0113,
}

pub const KM_SHIFT: i32 = 1 << 0;
pub const KM_CTRL: i32 = 1 << 1;
pub const KM_ALT: i32 = 1 << 2;
pub const KM_OSKEY: i32 = 1 << 3;

pub const KM_ANY: i32 = -1;
pub const KM_NOTHING: i32 = 0;

pub const KM_PRESS: i16 = 0;
pub const KM_RELEASE: i16 = 1;
pub const KM_CLICK: i16 = 2;
pub const KM_DOUBLE_CLICK: i16 = 3;
