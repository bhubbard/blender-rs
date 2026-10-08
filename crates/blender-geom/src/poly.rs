//! Polygon area and normal (`cross_poly_v3`, `area_poly_v3`, `normal_poly_v3`) using
//! Newell's method, which is exact for planar polygons and robust for slightly non-planar ones.

use glam::Vec3;

/// Un-normalised polygon normal; its length is twice the polygon area.
pub fn cross_poly_v3(verts: &[Vec3]) -> Vec3 {
    let mut n = Vec3::ZERO;
    if verts.len() < 3 {
        return n;
    }
    let mut prev = verts[verts.len() - 1];
    for &cur in verts {
        n += prev.cross(cur);
        prev = cur;
    }
    n
}

/// Area of a (near-)planar polygon.
pub fn area_poly_v3(verts: &[Vec3]) -> f32 {
    0.5 * cross_poly_v3(verts).length()
}

/// Unit normal of a polygon, or zero for a degenerate one. Returns the area as well.
pub fn normal_poly_v3(verts: &[Vec3]) -> (Vec3, f32) {
    let n = cross_poly_v3(verts);
    (n.normalize_or_zero(), 0.5 * n.length())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(z: f32) -> [Vec3; 4] {
        [
            Vec3::new(0.0, 0.0, z),
            Vec3::new(2.0, 0.0, z),
            Vec3::new(2.0, 2.0, z),
            Vec3::new(0.0, 2.0, z),
        ]
    }

    #[test]
    fn square_area_and_normal() {
        let (n, a) = normal_poly_v3(&square(5.0));
        assert_eq!(a, 4.0);
        assert_eq!(n, Vec3::Z);
        assert_eq!(area_poly_v3(&square(0.0)), 4.0);
    }

    #[test]
    fn winding_flips_normal() {
        let mut s = square(0.0);
        s.reverse();
        assert_eq!(normal_poly_v3(&s).0, -Vec3::Z);
    }

    #[test]
    fn degenerate_polygons() {
        assert_eq!(normal_poly_v3(&[]), (Vec3::ZERO, 0.0));
        assert_eq!(normal_poly_v3(&[Vec3::ONE, Vec3::ONE, Vec3::ONE]), (Vec3::ZERO, 0.0));
        let collinear = [Vec3::ZERO, Vec3::X, Vec3::new(2.0, 0.0, 0.0)];
        assert_eq!(area_poly_v3(&collinear), 0.0);
    }

    #[test]
    fn matches_triangle_formula() {
        let t = [Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)];
        assert_eq!(area_poly_v3(&t), 6.0);
    }
}
