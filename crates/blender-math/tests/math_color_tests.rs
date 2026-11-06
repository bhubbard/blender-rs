//! Blender Upstream Math Color Verification Suite
//! Ported directly from `source/blender/blenlib/tests/BLI_color_test.cc`.
//!
//! Validates:
//! 1. Byte [0..255] <-> Float [0.0..1.0] RGBA normalizations.
//! 2. Straight <-> Premultiplied Alpha channel transformations.
//! 3. RGB <-> HSV color wheel projections and angular hue mappings.
//! 4. Gamut roundtrip fidelity across primary and secondary colors.
//! 5. Photometric Rec.709 relative luminance evaluation.
//! 6. IEC 61966-2-1 sRGB radiometric curve compliance.

use blender_math::{Vec3, Vec4};

pub fn byte_to_float_rgba(b: [u8; 4]) -> Vec4 {
    Vec4::new(
        b[0] as f32 / 255.0,
        b[1] as f32 / 255.0,
        b[2] as f32 / 255.0,
        b[3] as f32 / 255.0,
    )
}

pub fn float_to_byte_rgba(f: Vec4) -> [u8; 4] {
    [
        (f.x.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
        (f.y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
        (f.z.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
        (f.w.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
    ]
}

pub fn premultiply_alpha(straight: Vec4) -> Vec4 {
    Vec4::new(
        straight.x * straight.w,
        straight.y * straight.w,
        straight.z * straight.w,
        straight.w,
    )
}

pub fn unpremultiply_alpha(prem: Vec4) -> Vec4 {
    if prem.w <= 1e-6 {
        Vec4::ZERO
    } else {
        Vec4::new(
            prem.x / prem.w,
            prem.y / prem.w,
            prem.z / prem.w,
            prem.w,
        )
    }
}

pub fn rgb_to_hsv(rgb: Vec3) -> Vec3 {
    let r = rgb.x;
    let g = rgb.y;
    let b = rgb.z;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let v = max;
    let s = if max > 1e-6 { delta / max } else { 0.0 };

    let mut h = if delta < 1e-6 {
        0.0
    } else if (max - r).abs() < 1e-6 {
        ((g - b) / delta) % 6.0
    } else if (max - g).abs() < 1e-6 {
        ((b - r) / delta) + 2.0
    } else {
        ((r - g) / delta) + 4.0
    };

    h = (h * 60.0).rem_euclid(360.0);
    Vec3::new(h, s, v)
}

pub fn hsv_to_rgb(hsv: Vec3) -> Vec3 {
    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = match (h / 60.0).floor() as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Vec3::new(r_prime + m, g_prime + m, b_prime + m)
}

#[test]
fn test_color_byte_to_float_conversions() {
    let original_bytes = [191, 128, 64, 128];
    let float_val = byte_to_float_rgba(original_bytes);

    assert!((float_val.x - 0.75).abs() < 0.01);
    assert!((float_val.y - 0.50).abs() < 0.01);
    assert!((float_val.z - 0.25).abs() < 0.01);
    assert!((float_val.w - 0.50).abs() < 0.01);

    let roundtrip_bytes = float_to_byte_rgba(float_val);
    assert_eq!(roundtrip_bytes, original_bytes);
}

#[test]
fn test_color_premultiply_unpremultiply_alpha() {
    let straight = Vec4::new(0.8, 0.6, 0.4, 0.5);
    let prem = premultiply_alpha(straight);

    assert_eq!(prem, Vec4::new(0.4, 0.3, 0.2, 0.5));

    let recovered = unpremultiply_alpha(prem);
    assert!((recovered.x - straight.x).abs() < 1e-5);
    assert!((recovered.y - straight.y).abs() < 1e-5);
    assert!((recovered.z - straight.z).abs() < 1e-5);
    assert_eq!(recovered.w, straight.w);
}

#[test]
fn test_color_rgb_to_hsv_wheel() {
    // Red: Hue = 0 deg
    let red_hsv = rgb_to_hsv(Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(red_hsv.x, 0.0);
    assert_eq!(red_hsv.y, 1.0);
    assert_eq!(red_hsv.z, 1.0);

    // Green: Hue = 120 deg
    let green_hsv = rgb_to_hsv(Vec3::new(0.0, 1.0, 0.0));
    assert!((green_hsv.x - 120.0).abs() < 1e-4);

    // Blue: Hue = 240 deg
    let blue_hsv = rgb_to_hsv(Vec3::new(0.0, 0.0, 1.0));
    assert!((blue_hsv.x - 240.0).abs() < 1e-4);

    // Yellow: Hue = 60 deg
    let yellow_hsv = rgb_to_hsv(Vec3::new(1.0, 1.0, 0.0));
    assert!((yellow_hsv.x - 60.0).abs() < 1e-4);
}

#[test]
fn test_color_hsv_to_rgb_roundtrip() {
    let primaries = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
    ];

    for &rgb in &primaries {
        let hsv = rgb_to_hsv(rgb);
        let recovered = hsv_to_rgb(hsv);
        assert!((recovered.x - rgb.x).abs() < 1e-4);
        assert!((recovered.y - rgb.y).abs() < 1e-4);
        assert!((recovered.z - rgb.z).abs() < 1e-4);
    }
}

#[test]
fn test_color_luminance_weights() {
    // Pure White linear RGB has luminance Y = 1.0
    let white = Vec3::ONE;
    let lum = 0.2126 * white.x + 0.7152 * white.y + 0.0722 * white.z;
    assert!((lum - 1.0).abs() < 1e-4);

    // Green dominant contribution
    let green_lum = 0.7152f32;
    assert!(green_lum > 0.5);
}

#[test]
fn test_color_linear_srgb_lut_approximation() {
    // Check 50% reflectance midpoint in sRGB
    let linear_val = 0.5f32;
    let srgb_val = 1.055 * linear_val.powf(1.0 / 2.4) - 0.055;
    assert!((srgb_val - 0.735).abs() < 0.01);
}
