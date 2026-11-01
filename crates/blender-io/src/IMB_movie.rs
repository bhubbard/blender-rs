/* SPDX-FileCopyrightText: 2024-2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

//! Movie file reading and writing structures (FFmpeg abstraction).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovieReaderState {
    Uninitialized,
    Failed,
    Valid,
}

#[derive(Debug, Clone)]
pub struct MovieReader {
    pub state: MovieReaderState,
    pub cur_position: i32,
    pub duration_in_frames: i32,
    pub frs_sec: i32,
    pub frs_sec_base: f64,
    pub start_offset: f64,
    pub x: i32,
    pub y: i32,
    pub video_rotation: i32,
    pub filepath: String,
}

impl Default for MovieReader {
    fn default() -> Self {
        Self {
            state: MovieReaderState::Uninitialized,
            cur_position: 0,
            duration_in_frames: 0,
            frs_sec: 24,
            frs_sec_base: 1.0,
            start_offset: 0.0,
            x: 0,
            y: 0,
            video_rotation: 0,
            filepath: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovieWriter {
    pub ffmpeg_type: i32,
    pub codec_id: i32,
    pub width: i32,
    pub height: i32,
    pub audio_bitrate: i32,
    pub video_bitrate: i32,
}

impl Default for MovieWriter {
    fn default() -> Self {
        Self {
            ffmpeg_type: 0,
            codec_id: 0,
            width: 1920,
            height: 1080,
            audio_bitrate: 192000,
            video_bitrate: 5000000,
        }
    }
}
