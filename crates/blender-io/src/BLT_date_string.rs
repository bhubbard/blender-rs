/* SPDX-FileCopyrightText: 2026 Blender Authors
 *
 * SPDX-License-Identifier: GPL-2.0-or-later */

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormat {
    Default = 0,
    LeSlash = 1,
    LeDot = 2,
    LeDash = 3,
    MeSlash = 4,
    BeSlash = 5,
    BeDot = 6,
    BeDash = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    H24 = 0,
    H12 = 1,
}

pub fn format_date_simple(year: i32, month: u32, day: u32, format: DateFormat) -> String {
    match format {
        DateFormat::Default | DateFormat::BeDash => format!("{:04}-{:02}-{:02}", year, month, day),
        DateFormat::LeSlash => format!("{:02}/{:02}/{:04}", day, month, year),
        DateFormat::LeDot => format!("{:02}.{:02}.{:04}", day, month, year),
        DateFormat::LeDash => format!("{:02}-{:02}-{:04}", day, month, year),
        DateFormat::MeSlash => format!("{:02}/{:02}/{:04}", month, day, year),
        DateFormat::BeSlash => format!("{:04}/{:02}/{:02}", year, month, day),
        DateFormat::BeDot => format!("{:04}.{:02}.{:02}", year, month, day),
    }
}

pub fn format_time_simple(hour: u32, minute: u32, format: TimeFormat) -> String {
    match format {
        TimeFormat::H24 => format!("{:02}:{:02}", hour, minute),
        TimeFormat::H12 => {
            let (h, ampm) = if hour == 0 {
                (12, "AM")
            } else if hour < 12 {
                (hour, "AM")
            } else if hour == 12 {
                (12, "PM")
            } else {
                (hour - 12, "PM")
            };
            format!("{:02}:{:02} {}", h, minute, ampm)
        }
    }
}
