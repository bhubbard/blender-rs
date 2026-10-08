//! 2D polygon triangulation and convex hull, porting the roles of `BLI_polyfill_2d`
//! (ear clipping) and `BLI_convexhull_2d` (monotone chain).

use glam::Vec2;

fn cross(o: Vec2, a: Vec2, b: Vec2) -> f32 {
    (a - o).perp_dot(b - o)
}

/// Signed area of a simple polygon (positive for counter-clockwise winding).
pub fn signed_area_poly_v2(pts: &[Vec2]) -> f32 {
    let n = pts.len();
    (0..n).map(|i| pts[i].perp_dot(pts[(i + 1) % n])).sum::<f32>() * 0.5
}

/// Triangulates a simple polygon (either winding, concave allowed) by ear clipping.
/// Returns `n - 2` triangles as vertex indices, wound the same way as the input.
/// Degenerate (zero-area) ears are dropped, so fewer triangles may be returned for
/// polygons with collinear vertices. Fewer than 3 points yields nothing.
pub fn triangulate_polygon_v2(pts: &[Vec2]) -> Vec<[u32; 3]> {
    let n = pts.len();
    let mut out = Vec::with_capacity(n.saturating_sub(2));
    if n < 3 {
        return out;
    }
    let sign = if signed_area_poly_v2(pts) >= 0.0 { 1.0 } else { -1.0 };
    let mut idx: Vec<usize> = (0..n).collect();

    let is_ear = |idx: &[usize], i: usize| -> bool {
        let m = idx.len();
        let (a, b, c) = (idx[(i + m - 1) % m], idx[i], idx[(i + 1) % m]);
        let (pa, pb, pc) = (pts[a], pts[b], pts[c]);
        if cross(pa, pb, pc) * sign <= 0.0 {
            return false; // reflex or collinear
        }
        idx.iter().all(|&k| {
            if k == a || k == b || k == c {
                return true;
            }
            let p = pts[k];
            if p == pa || p == pb || p == pc {
                return true;
            }
            let d1 = cross(pa, pb, p) * sign;
            let d2 = cross(pb, pc, p) * sign;
            let d3 = cross(pc, pa, p) * sign;
            !(d1 >= 0.0 && d2 >= 0.0 && d3 >= 0.0)
        })
    };

    while idx.len() > 3 {
        let m = idx.len();
        let ear = (0..m).find(|&i| is_ear(&idx, i));
        match ear {
            Some(i) => {
                out.push([idx[(i + m - 1) % m] as u32, idx[i] as u32, idx[(i + 1) % m] as u32]);
                idx.remove(i);
            }
            None => {
                // No strict ear (collinear run or numeric trouble): drop a degenerate vertex
                // if one exists, otherwise clip the most convex corner to guarantee progress.
                let degenerate = (0..m).find(|&i| {
                    cross(pts[idx[(i + m - 1) % m]], pts[idx[i]], pts[idx[(i + 1) % m]]) == 0.0
                });
                let i = degenerate.unwrap_or_else(|| {
                    (0..m)
                        .max_by(|&x, &y| {
                            let f = |i: usize| {
                                cross(pts[idx[(i + m - 1) % m]], pts[idx[i]], pts[idx[(i + 1) % m]]) * sign
                            };
                            f(x).total_cmp(&f(y))
                        })
                        .unwrap()
                });
                if degenerate.is_none() {
                    out.push([idx[(i + m - 1) % m] as u32, idx[i] as u32, idx[(i + 1) % m] as u32]);
                }
                idx.remove(i);
            }
        }
    }
    if cross(pts[idx[0]], pts[idx[1]], pts[idx[2]]) != 0.0 {
        out.push([idx[0] as u32, idx[1] as u32, idx[2] as u32]);
    }
    out
}

/// Convex hull (Andrew's monotone chain). Returns indices into `pts` in counter-clockwise
/// order, without collinear points; duplicates are collapsed.
pub fn convex_hull_v2(pts: &[Vec2]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..pts.len()).collect();
    order.sort_by(|&a, &b| pts[a].x.total_cmp(&pts[b].x).then(pts[a].y.total_cmp(&pts[b].y)));
    order.dedup_by(|a, b| pts[*a] == pts[*b]);
    if order.len() < 3 {
        return order;
    }
    let mut hull: Vec<usize> = Vec::with_capacity(order.len() * 2);
    for pass in 0..2 {
        let start = hull.len();
        let iter: Box<dyn Iterator<Item = &usize>> =
            if pass == 0 { Box::new(order.iter()) } else { Box::new(order.iter().rev()) };
        for &i in iter {
            while hull.len() >= start + 2
                && cross(pts[hull[hull.len() - 2]], pts[hull[hull.len() - 1]], pts[i]) <= 0.0
            {
                hull.pop();
            }
            hull.push(i);
        }
        hull.pop();
    }
    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u64);
    impl Lcg {
        fn f(&mut self) -> f32 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (self.0 >> 40) as f32 / (1u64 << 24) as f32
        }
    }

    fn tri_area_sum(pts: &[Vec2], tris: &[[u32; 3]]) -> f32 {
        tris.iter()
            .map(|t| cross(pts[t[0] as usize], pts[t[1] as usize], pts[t[2] as usize]).abs() * 0.5)
            .sum()
    }

    #[test]
    fn square_and_triangle() {
        let sq = [Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0), Vec2::new(2.0, 2.0), Vec2::new(0.0, 2.0)];
        let t = triangulate_polygon_v2(&sq);
        assert_eq!(t.len(), 2);
        assert_eq!(tri_area_sum(&sq, &t), 4.0);
        assert_eq!(triangulate_polygon_v2(&sq[..3]).len(), 1);
        assert!(triangulate_polygon_v2(&sq[..2]).is_empty());
    }

    #[test]
    fn concave_l_shape_both_windings() {
        let l = [
            Vec2::new(0.0, 0.0),
            Vec2::new(4.0, 0.0),
            Vec2::new(4.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 4.0),
            Vec2::new(0.0, 4.0),
        ];
        let t = triangulate_polygon_v2(&l);
        assert_eq!(t.len(), 4);
        assert!((tri_area_sum(&l, &t) - 7.0).abs() < 1e-5);
        let mut rev = l;
        rev.reverse();
        let t = triangulate_polygon_v2(&rev);
        assert_eq!(t.len(), 4);
        assert!((tri_area_sum(&rev, &t) - 7.0).abs() < 1e-5);
        // Output winding follows the input winding.
        for tri in &t {
            assert!(cross(rev[tri[0] as usize], rev[tri[1] as usize], rev[tri[2] as usize]) < 0.0);
        }
    }

    #[test]
    fn random_star_polygons_preserve_area() {
        let mut rng = Lcg(99);
        for n in [5usize, 8, 20, 60, 200] {
            let pts: Vec<Vec2> = (0..n)
                .map(|i| {
                    let a = i as f32 / n as f32 * std::f32::consts::TAU;
                    let r = 1.0 + rng.f() * 4.0;
                    Vec2::new(a.cos() * r, a.sin() * r)
                })
                .collect();
            let t = triangulate_polygon_v2(&pts);
            assert_eq!(t.len(), n - 2, "n={n}");
            let want = signed_area_poly_v2(&pts).abs();
            assert!((tri_area_sum(&pts, &t) - want).abs() < want * 1e-3, "n={n}");
        }
    }

    #[test]
    fn collinear_vertices_are_tolerated() {
        let p = [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.0, 2.0),
        ];
        let t = triangulate_polygon_v2(&p);
        assert!((tri_area_sum(&p, &t) - 4.0).abs() < 1e-5);
    }

    #[test]
    fn hull_basics() {
        let pts = [
            Vec2::new(0.0, 0.0),
            Vec2::new(4.0, 0.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(0.0, 4.0),
            Vec2::new(2.0, 2.0), // interior
            Vec2::new(2.0, 0.0), // collinear on an edge
            Vec2::new(0.0, 0.0), // duplicate
        ];
        let h = convex_hull_v2(&pts);
        assert_eq!(h.len(), 4);
        let poly: Vec<Vec2> = h.iter().map(|&i| pts[i]).collect();
        assert_eq!(signed_area_poly_v2(&poly), 16.0); // CCW
        assert!(convex_hull_v2(&[]).is_empty());
        assert_eq!(convex_hull_v2(&pts[..1]).len(), 1);
    }

    #[test]
    fn hull_contains_all_points() {
        let mut rng = Lcg(5);
        let pts: Vec<Vec2> = (0..300).map(|_| Vec2::new(rng.f() * 10.0, rng.f() * 10.0)).collect();
        let h = convex_hull_v2(&pts);
        for i in 0..h.len() {
            let (a, b) = (pts[h[i]], pts[h[(i + 1) % h.len()]]);
            assert!(pts.iter().all(|&p| cross(a, b, p) >= -1e-4));
        }
    }
}
