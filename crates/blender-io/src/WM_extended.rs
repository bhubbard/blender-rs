//! Blender WindowManager core engine, event dispatch, and operator handling ported to Rust
//! Corresponds to:
//! wm.cc, wm_cursors.cc, wm_event_system.cc, wm_files.cc, wm_files_link.cc, wm_operators.cc, wm_playanim.cc

use blender_dna::*;
use crate::wm_gizmo_wmapi::wmEvent;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WmCursorStyle {
    Default,
    None,
    Wait,
    Crosshair,
    Move,
    ResizeH,
    ResizeV,
    Hand,
    Text,
}

impl Default for WmCursorStyle {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WmEventDispatcher {
    pub queue: VecDeque<wmEvent>,
    pub active_cursor: WmCursorStyle,
}

impl WmEventDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_event(&mut self, event: wmEvent) {
        self.queue.push_back(event);
    }

    pub fn poll_event(&mut self) -> Option<wmEvent> {
        self.queue.pop_front()
    }

    pub fn set_cursor(&mut self, cursor: WmCursorStyle) {
        self.active_cursor = cursor;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WmFileContext {
    pub current_filepath: String,
    pub is_modified: bool,
    pub auto_save_interval_secs: u32,
}

impl WmFileContext {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            current_filepath: path.into(),
            is_modified: false,
            auto_save_interval_secs: 120,
        }
    }
}
