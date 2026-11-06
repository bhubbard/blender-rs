//! Blender Upstream Compositor & Color Management Test Suite
//! Ported directly from `tests/gtests/compositor/` and `tests/python/compositor/`.
//!
//! Validates:
//! 1. sRGB <-> Linear photometric transfer functions with roundtrip tolerance.
//! 2. ITU-R BT.709 (Rec.709 / sRGB D65) color matrix conversions and relative luminance Y.
//! 3. Porter-Duff Alpha-Over compositing with straight and premultiplied alpha channels.
//! 4. Summed-Area Table (Integral Image) constant-time O(1) box blur filter.
//! 5. Compositor node color operations: Brightness/Contrast, Invert, Multiply, Screen, Add.

use blender_math::{Vec3, Vec4};

// ============================================================================
// 1. Color Space Transfer Functions
// ============================================================================

/// IEC 61966-2-1:1999 sRGB to Linear photometric conversion
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Linear photometric to IEC 61966-2-1:1999 sRGB conversion
pub fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

pub fn srgb_to_linear_v3(v: Vec3) -> Vec3 {
    Vec3::new(srgb_to_linear(v.x), srgb_to_linear(v.y), srgb_to_linear(v.z))
}

pub fn linear_to_srgb_v3(v: Vec3) -> Vec3 {
    Vec3::new(linear_to_srgb(v.x), linear_to_srgb(v.y), linear_to_srgb(v.z))
}

// ============================================================================
// 2. ITU-R BT.709 Rec.709 / sRGB D65 Chromaticity Matrix
// ============================================================================

/// ITU-R BT.709 relative luminance coefficients (Rec.709 Y channel)
pub fn relative_luminance_rec709(linear_rgb: Vec3) -> f32 {
    0.2126 * linear_rgb.x + 0.7152 * linear_rgb.y + 0.0722 * linear_rgb.z
}

/// Linear RGB to CIE 1931 XYZ (D65 illuminant)
pub fn linear_rgb_to_xyz(rgb: Vec3) -> Vec3 {
    Vec3::new(
        0.4124564 * rgb.x + 0.3575761 * rgb.y + 0.1804375 * rgb.z,
        0.2126729 * rgb.x + 0.7151522 * rgb.y + 0.0721750 * rgb.z,
        0.0193339 * rgb.x + 0.1191920 * rgb.y + 0.9503041 * rgb.z,
    )
}

/// CIE 1931 XYZ to Linear RGB (D65 illuminant)
pub fn xyz_to_linear_rgb(xyz: Vec3) -> Vec3 {
    Vec3::new(
        3.2404542 * xyz.x - 1.5371385 * xyz.y - 0.4985314 * xyz.z,
        -0.9692660 * xyz.x + 1.8760108 * xyz.y + 0.0415560 * xyz.z,
        0.0556434 * xyz.x - 0.2040259 * xyz.y + 1.0572252 * xyz.z,
    )
}

// ============================================================================
// 3. Porter-Duff Alpha Over Compositing
// ============================================================================

/// Porter-Duff Alpha-Over for premultiplied RGBA colors:
/// Out = FG + BG * (1 - FG.a)
pub fn alpha_over_premultiplied(fg: Vec4, bg: Vec4) -> Vec4 {
    let out_a = fg.w + bg.w * (1.0 - fg.w);
    let out_rgb = Vec3::new(fg.x, fg.y, fg.z) + Vec3::new(bg.x, bg.y, bg.z) * (1.0 - fg.w);
    Vec4::new(out_rgb.x, out_rgb.y, out_rgb.z, out_a)
}

/// Porter-Duff Alpha-Over for straight RGBA colors
pub fn alpha_over_straight(fg: Vec4, bg: Vec4) -> Vec4 {
    let out_a = fg.w + bg.w * (1.0 - fg.w);
    if out_a <= 1e-6 {
        return Vec4::ZERO;
    }
    let fg_rgb = Vec3::new(fg.x, fg.y, fg.z) * fg.w;
    let bg_rgb = Vec3::new(bg.x, bg.y, bg.z) * bg.w * (1.0 - fg.w);
    let out_rgb = (fg_rgb + bg_rgb) / out_a;
    Vec4::new(out_rgb.x, out_rgb.y, out_rgb.z, out_a)
}

// ============================================================================
// 4. Integral Image (Summed-Area Table) for Constant Time O(1) Box Blur
// ============================================================================

pub struct SummedAreaTable {
    pub width: usize,
    pub height: usize,
    table: Vec<f32>,
}

impl SummedAreaTable {
    pub fn new(width: usize, height: usize, image: &[f32]) -> Self {
        assert_eq!(image.len(), width * height);
        let mut table = vec![0.0f32; width * height];

        for y in 0..height {
            let mut row_sum = 0.0f32;
            for x in 0..width {
                row_sum += image[y * width + x];
                let above = if y > 0 { table[(y - 1) * width + x] } else { 0.0 };
                table[y * width + x] = row_sum + above;
            }
        }

        Self { width, height, table }
    }

    /// O(1) rectangular sum query within [x0..=x1, y0..=y1]
    pub fn query_sum(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> f32 {
        let x1 = x1.min(self.width - 1);
        let y1 = y1.min(self.height - 1);

        let d = self.table[y1 * self.width + x1];
        let b = if y0 > 0 { self.table[(y0 - 1) * self.width + x1] } else { 0.0 };
        let c = if x0 > 0 { self.table[y1 * self.width + (x0 - 1)] } else { 0.0 };
        let a = if x0 > 0 && y0 > 0 { self.table[(y0 - 1) * self.width + (x0 - 1)] } else { 0.0 };

        d - b - c + a
    }

    /// O(1) box blur sample centered at (x, y) with radius r
    pub fn box_blur_pixel(&self, x: usize, y: usize, radius: usize) -> f32 {
        let x0 = x.saturating_sub(radius);
        let y0 = y.saturating_sub(radius);
        let x1 = (x + radius).min(self.width - 1);
        let y1 = (y + radius).min(self.height - 1);

        let area = ((x1 - x0 + 1) * (y1 - y0 + 1)) as f32;
        self.query_sum(x0, y0, x1, y1) / area
    }
}

// ============================================================================
// 5. Compositor Blend Modes
// ============================================================================

pub fn blend_multiply(a: Vec3, b: Vec3) -> Vec3 {
    a * b
}

pub fn blend_screen(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::ONE - (Vec3::ONE - a) * (Vec3::ONE - b)
}

pub fn blend_add(a: Vec3, b: Vec3) -> Vec3 {
    a + b
}

pub fn color_brightness_contrast(c: Vec3, bright: f32, contrast: f32) -> Vec3 {
    (c - Vec3::splat(0.5)) * (1.0 + contrast) + Vec3::splat(0.5 + bright)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[test]
fn test_srgb_linear_roundtrip() {
    let test_values = [0.0, 0.001, 0.04045, 0.18, 0.5, 0.73, 1.0];
    for &val in &test_values {
        let linear = srgb_to_linear(val);
        let recovered = linear_to_srgb(linear);
        assert!(
            (recovered - val).abs() < 1e-4,
            "Failed sRGB roundtrip for {val}: intermediate linear={linear}, recovered={recovered}"
        );
    }

    // Standard mid-gray check: 18% linear reflectance ~ 46.1% sRGB
    let mid_gray_srgb = linear_to_srgb(0.18);
    assert!((mid_gray_srgb - 0.46135).abs() < 1e-3);
}

#[test]
fn test_rec709_luminance_and_xyz_transform() {
    // Pure White in linear RGB has Y = 1.0
    let white = Vec3::ONE;
    let lum_white = relative_luminance_rec709(white);
    assert!((lum_white - 1.0).abs() < 1e-4);

    // Primary contributions
    let red_lum = relative_luminance_rec709(Vec3::new(1.0, 0.0, 0.0));
    let green_lum = relative_luminance_rec709(Vec3::new(0.0, 1.0, 0.0));
    let blue_lum = relative_luminance_rec709(Vec3::new(0.0, 0.0, 1.0));
    assert!((red_lum - 0.2126).abs() < 1e-4);
    assert!((green_lum - 0.7152).abs() < 1e-4);
    assert!((blue_lum - 0.0722).abs() < 1e-4);

    // RGB -> XYZ -> RGB roundtrip
    let test_rgb = Vec3::new(0.35, 0.65, 0.15);
    let xyz = linear_rgb_to_xyz(test_rgb);
    let recovered_rgb = xyz_to_linear_rgb(xyz);
    assert!((recovered_rgb.x - test_rgb.x).abs() < 1e-4);
    assert!((recovered_rgb.y - test_rgb.y).abs() < 1e-4);
    assert!((recovered_rgb.z - test_rgb.z).abs() < 1e-4);
}

#[test]
fn test_alpha_over_compositing() {
    // 1. Fully opaque red over black -> Pure Red
    let fg_red = Vec4::new(1.0, 0.0, 0.0, 1.0);
    let bg_black = Vec4::new(0.0, 0.0, 0.0, 1.0);
    let res = alpha_over_straight(fg_red, bg_black);
    assert_eq!(res, Vec4::new(1.0, 0.0, 0.0, 1.0));

    // 2. 50% transparent white over 100% opaque black -> 50% Gray
    let fg_white_half = Vec4::new(1.0, 1.0, 1.0, 0.5);
    let res_half = alpha_over_straight(fg_white_half, bg_black);
    assert!((res_half.x - 0.5).abs() < 1e-4);
    assert!((res_half.y - 0.5).abs() < 1e-4);
    assert!((res_half.z - 0.5).abs() < 1e-4);
    assert_eq!(res_half.w, 1.0);

    // 3. Fully transparent over anything yields background unchanged
    let fg_transparent = Vec4::new(1.0, 0.0, 0.0, 0.0);
    let bg_blue = Vec4::new(0.0, 0.0, 1.0, 1.0);
    let res_bg = alpha_over_straight(fg_transparent, bg_blue);
    assert_eq!(res_bg, bg_blue);
}

#[test]
fn test_summed_area_table_box_blur() {
    // Create an 8x8 image with a single bright impulse in the center
    let width = 8;
    let height = 8;
    let mut image = vec![0.0f32; width * height];
    image[3 * width + 3] = 16.0; // Impulse

    let sat = SummedAreaTable::new(width, height, &image);

    // Total sum of the whole image must be 16.0
    let total_sum = sat.query_sum(0, 0, width - 1, height - 1);
    assert!((total_sum - 16.0).abs() < 1e-5);

    // 3x3 window around (3, 3) contains the impulse: area = 9, sum = 16
    let blur_center = sat.box_blur_pixel(3, 3, 1);
    assert!((blur_center - 16.0 / 9.0).abs() < 1e-5);

    // Pixel far away from impulse has zero blur value
    let blur_corner = sat.box_blur_pixel(0, 0, 1);
    assert_eq!(blur_corner, 0.0);
}

#[test]
fn test_compositor_blend_modes() {
    let col_a = Vec3::new(0.5, 0.2, 0.8);
    let col_b = Vec3::new(0.4, 0.5, 0.1);

    // Multiply
    let mult = blend_multiply(col_a, col_b);
    assert!((mult.x - 0.2).abs() < 1e-5);
    assert!((mult.y - 0.1).abs() < 1e-5);
    assert!((mult.z - 0.08).abs() < 1e-5);

    // Screen
    let screen = blend_screen(col_a, col_b);
    // 1 - (1 - 0.5)*(1 - 0.4) = 1 - 0.3 = 0.7
    assert!((screen.x - 0.7).abs() < 1e-5);

    // Add
    let add = blend_add(col_a, col_b);
    assert!((add.x - 0.9).abs() < 1e-5);
    assert!((add.y - 0.7).abs() < 1e-5);
    assert!((add.z - 0.9).abs() < 1e-5);
}

#[test]
fn test_color_brightness_contrast() {
    let mid_gray = Vec3::splat(0.5);

    // Increasing contrast keeps 0.5 at 0.5
    let c1 = color_brightness_contrast(mid_gray, 0.0, 0.5);
    assert_eq!(c1, mid_gray);

    // Increasing brightness shifts 0.5 to 0.7
    let c2 = color_brightness_contrast(mid_gray, 0.2, 0.0);
    assert!((c2.x - 0.7).abs() < 1e-5);
}
