//! STL import/export. Reads binary and ASCII files (auto-detected), writes either.
//!
//! Detection follows the usual robust rule: a file is binary when its length is exactly
//! `84 + 50 * triangle_count`, because binary headers may legitimately begin with `solid`.

use glam::Vec3;
use std::collections::HashMap;
use std::fmt::{self, Write as _};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub normal: Vec3,
    pub v: [Vec3; 3],
}

impl Triangle {
    /// Triangle with its normal computed from the winding (zero for a degenerate one).
    pub fn from_vertices(v: [Vec3; 3]) -> Self {
        let normal = (v[1] - v[0]).cross(v[2] - v[0]).normalize_or_zero();
        Self { normal, v }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StlMesh {
    pub triangles: Vec<Triangle>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StlError {
    /// Fewer than 84 bytes, or the declared triangle count does not fit the data.
    Truncated,
    /// Not recognisable as binary or ASCII STL.
    UnknownFormat,
    /// ASCII parse error with a 1-based line number.
    Ascii { line: usize, message: String },
}

impl fmt::Display for StlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StlError::Truncated => write!(f, "truncated binary STL"),
            StlError::UnknownFormat => write!(f, "not a binary or ASCII STL file"),
            StlError::Ascii { line, message } => write!(f, "line {line}: {message}"),
        }
    }
}

impl std::error::Error for StlError {}

/// Parses STL data, detecting binary versus ASCII.
pub fn parse_stl(data: &[u8]) -> Result<StlMesh, StlError> {
    if data.len() >= 84 {
        let count = u32::from_le_bytes([data[80], data[81], data[82], data[83]]) as usize;
        if data.len() == 84 + 50 * count {
            return parse_binary(data, count);
        }
    }
    let looks_ascii = data.len() >= 5 && data[..5].eq_ignore_ascii_case(b"solid");
    if looks_ascii {
        if let Ok(text) = std::str::from_utf8(data) {
            if text.contains("facet") || text.contains("endsolid") {
                return parse_ascii(text);
            }
        }
    }
    if data.len() >= 84 {
        // Has a binary-sized header but the length disagrees with the count.
        return Err(StlError::Truncated);
    }
    Err(StlError::UnknownFormat)
}

fn f32_at(d: &[u8], o: usize) -> f32 {
    f32::from_le_bytes([d[o], d[o + 1], d[o + 2], d[o + 3]])
}

fn vec_at(d: &[u8], o: usize) -> Vec3 {
    Vec3::new(f32_at(d, o), f32_at(d, o + 4), f32_at(d, o + 8))
}

fn parse_binary(d: &[u8], count: usize) -> Result<StlMesh, StlError> {
    let mut triangles = Vec::with_capacity(count);
    for i in 0..count {
        let o = 84 + 50 * i;
        triangles.push(Triangle {
            normal: vec_at(d, o),
            v: [vec_at(d, o + 12), vec_at(d, o + 24), vec_at(d, o + 36)],
        });
    }
    Ok(StlMesh { triangles })
}

fn parse_ascii(text: &str) -> Result<StlMesh, StlError> {
    let err = |line: usize, message: &str| StlError::Ascii { line, message: message.to_string() };
    let mut triangles = Vec::new();
    let mut normal = Vec3::ZERO;
    let mut verts: Vec<Vec3> = Vec::new();
    let mut in_facet = false;
    for (n, raw) in text.lines().enumerate() {
        let line = n + 1;
        let mut t = raw.split_whitespace();
        let Some(kw) = t.next() else { continue };
        let nums = |it: std::str::SplitWhitespace<'_>| -> Result<Vec3, StlError> {
            let v: Result<Vec<f32>, _> = it.map(str::parse::<f32>).collect();
            match v {
                Ok(v) if v.len() == 3 => Ok(Vec3::new(v[0], v[1], v[2])),
                _ => Err(err(line, "expected three numbers")),
            }
        };
        match kw.to_ascii_lowercase().as_str() {
            "facet" => {
                if in_facet {
                    return Err(err(line, "nested facet"));
                }
                in_facet = true;
                verts.clear();
                let mut rest = t;
                if rest.next().map(str::to_ascii_lowercase).as_deref() != Some("normal") {
                    return Err(err(line, "expected `facet normal`"));
                }
                normal = nums(rest)?;
            }
            "vertex" => {
                if !in_facet {
                    return Err(err(line, "vertex outside facet"));
                }
                verts.push(nums(t)?);
            }
            "endfacet" => {
                if !in_facet || verts.len() != 3 {
                    return Err(err(line, "facet must have exactly 3 vertices"));
                }
                triangles.push(Triangle { normal, v: [verts[0], verts[1], verts[2]] });
                in_facet = false;
            }
            _ => {}
        }
    }
    if in_facet {
        return Err(err(text.lines().count(), "unterminated facet"));
    }
    Ok(StlMesh { triangles })
}

/// Writes binary STL with an 80-byte header carrying `header` (truncated / zero padded).
pub fn write_binary(mesh: &StlMesh, header: &str) -> Vec<u8> {
    let mut out = vec![0u8; 80];
    let h = header.as_bytes();
    let n = h.len().min(80);
    out[..n].copy_from_slice(&h[..n]);
    out.extend_from_slice(&(mesh.triangles.len() as u32).to_le_bytes());
    for t in &mesh.triangles {
        for v in [t.normal, t.v[0], t.v[1], t.v[2]] {
            for c in [v.x, v.y, v.z] {
                out.extend_from_slice(&c.to_le_bytes());
            }
        }
        out.extend_from_slice(&[0, 0]);
    }
    out
}

/// Writes ASCII STL. Floats use Rust's shortest round-trip formatting.
pub fn write_ascii(mesh: &StlMesh, name: &str) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "solid {name}");
    for t in &mesh.triangles {
        let _ = writeln!(s, "  facet normal {} {} {}", t.normal.x, t.normal.y, t.normal.z);
        let _ = writeln!(s, "    outer loop");
        for v in &t.v {
            let _ = writeln!(s, "      vertex {} {} {}", v.x, v.y, v.z);
        }
        let _ = writeln!(s, "    endloop");
        let _ = writeln!(s, "  endfacet");
    }
    let _ = writeln!(s, "endsolid {name}");
    s
}

impl StlMesh {
    /// Welds bit-identical vertices, returning positions and index triples.
    /// STL stores every triangle's vertices separately, so this recovers shared topology.
    pub fn to_indexed(&self) -> (Vec<Vec3>, Vec<[u32; 3]>) {
        let mut map: HashMap<[u32; 3], u32> = HashMap::new();
        let mut positions = Vec::new();
        let mut tris = Vec::with_capacity(self.triangles.len());
        for t in &self.triangles {
            let mut idx = [0u32; 3];
            for (k, v) in t.v.iter().enumerate() {
                // Normalise -0.0 so that 0.0 and -0.0 weld.
                let key = [(v.x + 0.0).to_bits(), (v.y + 0.0).to_bits(), (v.z + 0.0).to_bits()];
                idx[k] = *map.entry(key).or_insert_with(|| {
                    positions.push(*v);
                    positions.len() as u32 - 1
                });
            }
            tris.push(idx);
        }
        (positions, tris)
    }

    /// Builds a mesh from indexed triangles, computing normals from the winding.
    pub fn from_indexed(positions: &[Vec3], tris: &[[u32; 3]]) -> Self {
        Self {
            triangles: tris
                .iter()
                .map(|t| {
                    Triangle::from_vertices([
                        positions[t[0] as usize],
                        positions[t[1] as usize],
                        positions[t[2] as usize],
                    ])
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tetra() -> StlMesh {
        let p = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ];
        StlMesh::from_indexed(&p, &[[0, 2, 1], [0, 1, 3], [1, 2, 3], [0, 3, 2]])
    }

    #[test]
    fn binary_roundtrip() {
        let m = tetra();
        let bytes = write_binary(&m, "test");
        assert_eq!(bytes.len(), 84 + 50 * 4);
        assert_eq!(parse_stl(&bytes).unwrap(), m);
    }

    #[test]
    fn ascii_roundtrip() {
        let m = tetra();
        let text = write_ascii(&m, "tetra");
        assert!(text.starts_with("solid tetra"));
        assert_eq!(parse_stl(text.as_bytes()).unwrap(), m);
    }

    #[test]
    fn binary_header_starting_with_solid_is_still_binary() {
        let bytes = write_binary(&tetra(), "solid exported by some CAD tool");
        assert_eq!(parse_stl(&bytes).unwrap(), tetra());
    }

    #[test]
    fn errors() {
        assert_eq!(parse_stl(b"hello"), Err(StlError::UnknownFormat));
        let mut bytes = write_binary(&tetra(), "");
        bytes.truncate(bytes.len() - 10);
        assert_eq!(parse_stl(&bytes), Err(StlError::Truncated));
        let bad = "solid x\nfacet normal 0 0 1\nvertex 0 0 0\nendfacet\nendsolid x\n";
        assert!(matches!(parse_stl(bad.as_bytes()), Err(StlError::Ascii { line: 4, .. })));
        let bad = "solid x\nfacet normal 0 0\nendsolid x\n";
        assert!(matches!(parse_stl(bad.as_bytes()), Err(StlError::Ascii { line: 2, .. })));
        let bad = "solid x\nfacet normal 0 0 1\nvertex 0 0 0\n";
        assert!(matches!(parse_stl(bad.as_bytes()), Err(StlError::Ascii { .. })));
    }

    #[test]
    fn empty_meshes() {
        let m = StlMesh::default();
        assert_eq!(parse_stl(&write_binary(&m, "")).unwrap(), m);
        assert_eq!(parse_stl(write_ascii(&m, "e").as_bytes()).unwrap(), m);
    }

    #[test]
    fn welding_recovers_shared_vertices() {
        let (pos, tris) = tetra().to_indexed();
        assert_eq!(pos.len(), 4);
        assert_eq!(tris.len(), 4);
        // -0.0 and 0.0 weld together.
        let m = StlMesh {
            triangles: vec![Triangle::from_vertices([
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-0.0, 1.0, 0.0),
                Vec3::new(1.0, 0.0, -0.0),
            ])],
        };
        assert_eq!(m.to_indexed().0.len(), 3);
    }

    #[test]
    fn computed_normals_follow_winding() {
        let t = Triangle::from_vertices([Vec3::ZERO, Vec3::X, Vec3::Y]);
        assert_eq!(t.normal, Vec3::Z);
        assert_eq!(Triangle::from_vertices([Vec3::ZERO, Vec3::X, Vec3::X * 2.0]).normal, Vec3::ZERO);
        for t in tetra().triangles {
            // Outward normals: pointing away from the tetrahedron's centroid.
            let c = (t.v[0] + t.v[1] + t.v[2]) / 3.0 - Vec3::splat(0.25);
            assert!(t.normal.dot(c) > 0.0);
        }
    }
}
