//! Real-time 3D wireframe rasterizer for terminal viewport.

use blender_math::{Mat4, Vec3, Vec4};

pub struct ViewportRasterizer {
    width: usize,
    height: usize,
    buffer: Vec<char>,
}

impl ViewportRasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![' '; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(' ');
    }

    /// Draws a line using Bresenham algorithm on character grid.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, ch: char) {
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                self.buffer[(y as usize) * self.width + (x as usize)] = ch;
            }
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Renders a set of 3D lines given model rotation angles (pitch, yaw).
    pub fn render_frame(&mut self, lines: &[(Vec3, Vec3)], pitch: f32, yaw: f32) -> String {
        self.clear();

        // Model matrix (rotation)
        let rot = Mat4::from_rotation_x(pitch) * Mat4::from_rotation_y(yaw);

        // View matrix (camera at z = 4.0 looking at origin)
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 4.0), Vec3::ZERO, Vec3::Y);

        // Perspective projection
        let (w, h) = (self.width, self.height);
        let aspect = (w as f32) / (h as f32 * 2.0); // Adjust for terminal font aspect ratio
        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_3, aspect, 0.1, 100.0);

        let mvp = proj * view * rot;

        let to_screen = |v: Vec3| -> Option<(i32, i32)> {
            let clip = mvp * Vec4::new(v.x, v.y, v.z, 1.0);
            if clip.w <= 0.01 {
                return None;
            }
            let ndc = Vec3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w);
            if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 {
                return None;
            }

            let sx = ((ndc.x + 1.0) * 0.5 * (w as f32 - 1.0)).round() as i32;
            let sy = ((-ndc.y + 1.0) * 0.5 * (h as f32 - 1.0)).round() as i32;
            Some((sx, sy))
        };

        for (v0, v1) in lines {
            if let (Some(p0), Some(p1)) = (to_screen(*v0), to_screen(*v1)) {
                self.draw_line(p0.0, p0.1, p1.0, p1.1, '█');
            }
        }

        // Build string with border
        let mut out = String::with_capacity((self.width + 3) * (self.height + 2));
        out.push('┌');
        out.push_str(&"─".repeat(self.width));
        out.push_str("┐\n");

        for y in 0..self.height {
            out.push('│');
            let row_start = y * self.width;
            let row_end = row_start + self.width;
            for &c in &self.buffer[row_start..row_end] {
                out.push(c);
            }
            out.push_str("│\n");
        }

        out.push('└');
        out.push_str(&"─".repeat(self.width));
        out.push_str("┘\n");
        out
    }
}
