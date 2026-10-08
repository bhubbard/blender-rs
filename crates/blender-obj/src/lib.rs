//! Wavefront OBJ import/export for plain mesh data, in the spirit of Blender's
//! `io/wavefront_obj`. Independent of the BMesh crate so it stays buildable on its own;
//! a thin adapter can convert [`ObjMesh`] to a `BMesh`.
//!
//! Supported: `v`, `vt`, `vn`, `f` (all `v`, `v/vt`, `v//vn`, `v/vt/vn` forms, negative
//! indices), `o`/`g` names (recorded). Other statements are ignored.

use blender_geom::poly::cross_poly_v3;
use blender_geom::polyfill::triangulate_polygon_v2;
use glam::{Vec2, Vec3};
use std::fmt::{self, Write as _};

/// One face corner: zero-based indices into the mesh attribute arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corner {
    pub v: u32,
    pub vt: Option<u32>,
    pub vn: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Face {
    pub corners: Vec<Corner>,
    /// Name of the most recent `o`/`g` statement, if any.
    pub group: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjMesh {
    pub positions: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Face>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjError {
    BadNumber { line: usize, text: String },
    MissingValues { line: usize, keyword: &'static str },
    IndexOutOfRange { line: usize, index: i64, len: usize },
    TooFewCorners { line: usize },
}

impl fmt::Display for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjError::BadNumber { line, text } => write!(f, "line {line}: invalid number {text:?}"),
            ObjError::MissingValues { line, keyword } => {
                write!(f, "line {line}: `{keyword}` needs more values")
            }
            ObjError::IndexOutOfRange { line, index, len } => {
                write!(f, "line {line}: index {index} out of range (have {len})")
            }
            ObjError::TooFewCorners { line } => write!(f, "line {line}: face needs at least 3 corners"),
        }
    }
}

impl std::error::Error for ObjError {}

fn float(tok: &str, line: usize) -> Result<f32, ObjError> {
    tok.parse::<f32>().map_err(|_| ObjError::BadNumber { line, text: tok.to_string() })
}

/// Resolves a 1-based (or negative, relative) OBJ index against `len` to a 0-based index.
fn resolve(tok: &str, len: usize, line: usize) -> Result<u32, ObjError> {
    let idx: i64 = tok
        .parse()
        .map_err(|_| ObjError::BadNumber { line, text: tok.to_string() })?;
    let zero = if idx > 0 { idx - 1 } else if idx < 0 { len as i64 + idx } else { -1 };
    if zero < 0 || zero as usize >= len {
        return Err(ObjError::IndexOutOfRange { line, index: idx, len });
    }
    Ok(zero as u32)
}

/// Parses OBJ text.
pub fn parse_obj(src: &str) -> Result<ObjMesh, ObjError> {
    let mut mesh = ObjMesh::default();
    let mut group: Option<String> = None;
    for (n, raw) in src.lines().enumerate() {
        let line = n + 1;
        let text = raw.split('#').next().unwrap_or("").trim();
        let mut toks = text.split_whitespace();
        let Some(kw) = toks.next() else { continue };
        let rest: Vec<&str> = toks.collect();
        match kw {
            "v" | "vn" => {
                if rest.len() < 3 {
                    let keyword = if kw == "v" { "v" } else { "vn" };
                    return Err(ObjError::MissingValues { line, keyword });
                }
                let p = Vec3::new(float(rest[0], line)?, float(rest[1], line)?, float(rest[2], line)?);
                if kw == "v" {
                    mesh.positions.push(p);
                } else {
                    mesh.normals.push(p);
                }
            }
            "vt" => {
                if rest.len() < 2 {
                    return Err(ObjError::MissingValues { line, keyword: "vt" });
                }
                mesh.uvs.push(Vec2::new(float(rest[0], line)?, float(rest[1], line)?));
            }
            "f" => {
                if rest.len() < 3 {
                    return Err(ObjError::TooFewCorners { line });
                }
                let mut corners = Vec::with_capacity(rest.len());
                for tok in &rest {
                    let mut parts = tok.split('/');
                    let v = resolve(parts.next().unwrap_or(""), mesh.positions.len(), line)?;
                    let vt = match parts.next() {
                        Some(s) if !s.is_empty() => Some(resolve(s, mesh.uvs.len(), line)?),
                        _ => None,
                    };
                    let vn = match parts.next() {
                        Some(s) if !s.is_empty() => Some(resolve(s, mesh.normals.len(), line)?),
                        _ => None,
                    };
                    corners.push(Corner { v, vt, vn });
                }
                mesh.faces.push(Face { corners, group: group.clone() });
            }
            "o" | "g" => {
                let name = rest.join(" ");
                group = if name.is_empty() { None } else { Some(name) };
            }
            _ => {}
        }
    }
    Ok(mesh)
}

/// Serialises a mesh to OBJ text. Group names are emitted when they change between faces.
pub fn write_obj(mesh: &ObjMesh) -> String {
    let mut s = String::new();
    for p in &mesh.positions {
        let _ = writeln!(s, "v {} {} {}", p.x, p.y, p.z);
    }
    for t in &mesh.uvs {
        let _ = writeln!(s, "vt {} {}", t.x, t.y);
    }
    for n in &mesh.normals {
        let _ = writeln!(s, "vn {} {} {}", n.x, n.y, n.z);
    }
    let mut current: Option<&str> = None;
    for face in &mesh.faces {
        if face.group.as_deref() != current {
            current = face.group.as_deref();
            if let Some(g) = current {
                let _ = writeln!(s, "g {g}");
            }
        }
        s.push('f');
        for c in &face.corners {
            let _ = match (c.vt, c.vn) {
                (None, None) => write!(s, " {}", c.v + 1),
                (Some(t), None) => write!(s, " {}/{}", c.v + 1, t + 1),
                (None, Some(n)) => write!(s, " {}//{}", c.v + 1, n + 1),
                (Some(t), Some(n)) => write!(s, " {}/{}/{}", c.v + 1, t + 1, n + 1),
            };
        }
        s.push('\n');
    }
    s
}

impl ObjMesh {
    /// Triangulates face `f` (n-gons and concave faces included). Returns triples of
    /// *corner* indices within the face; empty for an unknown or degenerate face.
    pub fn face_triangles(&self, f: usize) -> Vec<[u32; 3]> {
        let Some(face) = self.faces.get(f) else { return Vec::new() };
        let pts: Vec<Vec3> = face.corners.iter().map(|c| self.positions[c.v as usize]).collect();
        match pts.len() {
            0..=2 => return Vec::new(),
            3 => return vec![[0, 1, 2]],
            _ => {}
        }
        let n = cross_poly_v3(&pts);
        if n.length_squared() == 0.0 {
            return Vec::new();
        }
        // Project onto the plane by dropping the dominant normal axis, keeping orientation.
        let a = n.abs();
        let pts2: Vec<Vec2> = pts
            .iter()
            .map(|p| {
                if a.z >= a.x && a.z >= a.y {
                    Vec2::new(p.x, p.y) * n.z.signum()
                } else if a.x >= a.y {
                    Vec2::new(p.y, p.z) * n.x.signum()
                } else {
                    Vec2::new(p.z, p.x) * n.y.signum()
                }
            })
            .collect();
        triangulate_polygon_v2(&pts2)
    }

    /// All faces triangulated into position-index triples.
    pub fn triangulated(&self) -> Vec<[u32; 3]> {
        let mut out = Vec::new();
        for (i, face) in self.faces.iter().enumerate() {
            for t in self.face_triangles(i) {
                out.push([
                    face.corners[t[0] as usize].v,
                    face.corners[t[1] as usize].v,
                    face.corners[t[2] as usize].v,
                ]);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CUBE: &str = "\
# unit cube
o Cube
v 0 0 0
v 1 0 0
v 1 1 0
v 0 1 0
v 0 0 1
v 1 0 1
v 1 1 1
v 0 1 1
f 1 4 3 2
f 5 6 7 8
f 1 2 6 5
f 2 3 7 6
f 3 4 8 7
f 4 1 5 8
";

    #[test]
    fn parses_cube() {
        let m = parse_obj(CUBE).unwrap();
        assert_eq!(m.positions.len(), 8);
        assert_eq!(m.faces.len(), 6);
        assert!(m.faces.iter().all(|f| f.corners.len() == 4 && f.group.as_deref() == Some("Cube")));
        assert_eq!(m.triangulated().len(), 12);
    }

    #[test]
    fn corner_forms_and_negative_indices() {
        let src = "v 0 0 0\nv 1 0 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 0 1\nvn 0 0 1\n\
                   f 1/1/1 2/2/1 3/3/1\nf -3//-1 -2//-1 -1//-1\nf 1/1 2/2 3/3\nf 1 2 3\n";
        let m = parse_obj(src).unwrap();
        assert_eq!(m.faces[0].corners[1], Corner { v: 1, vt: Some(1), vn: Some(0) });
        assert_eq!(m.faces[1].corners[0], Corner { v: 0, vt: None, vn: Some(0) });
        assert_eq!(m.faces[2].corners[2], Corner { v: 2, vt: Some(2), vn: None });
        assert_eq!(m.faces[3].corners[0], Corner { v: 0, vt: None, vn: None });
    }

    #[test]
    fn roundtrip_is_lossless() {
        let m = parse_obj(CUBE).unwrap();
        let text = write_obj(&m);
        assert_eq!(parse_obj(&text).unwrap(), m);
        let full = "v 0.1 0.2 0.3\nv 1 0 0\nv 0 1 0\nvt 0.25 0.5\nvn 0 0 1\ng A\nf 1/1/1 2/1/1 3/1/1\ng B\nf 3//1 2//1 1//1\n";
        let m = parse_obj(full).unwrap();
        assert_eq!(parse_obj(&write_obj(&m)).unwrap(), m);
    }

    #[test]
    fn errors_are_reported_with_line_numbers() {
        assert_eq!(
            parse_obj("v 1 2\n"),
            Err(ObjError::MissingValues { line: 1, keyword: "v" })
        );
        assert!(matches!(parse_obj("v 1 x 3\n"), Err(ObjError::BadNumber { line: 1, .. })));
        assert!(matches!(
            parse_obj("v 0 0 0\nf 1 2 3\n"),
            Err(ObjError::IndexOutOfRange { line: 2, .. })
        ));
        assert!(matches!(parse_obj("f 0 1 2\n"), Err(ObjError::IndexOutOfRange { .. })));
        assert_eq!(parse_obj("v 0 0 0\nf 1 1\n"), Err(ObjError::TooFewCorners { line: 2 }));
    }

    #[test]
    fn ignores_comments_blank_lines_and_unknown_statements() {
        let m = parse_obj("# hi\n\nmtllib x.mtl\nusemtl m\ns off\nv 1 2 3 # trailing\n").unwrap();
        assert_eq!(m.positions, vec![Vec3::new(1.0, 2.0, 3.0)]);
    }

    #[test]
    fn concave_ngon_triangulates_on_any_plane() {
        // L-shape lying in the XZ plane (normal along ±Y) and the YZ plane.
        for map in [|x: f32, y: f32| Vec3::new(x, 0.0, y), |x: f32, y: f32| Vec3::new(0.0, x, y)] {
            let l = [(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (1.0, 1.0), (1.0, 4.0), (0.0, 4.0)];
            let mut m = ObjMesh::default();
            m.positions = l.iter().map(|&(x, y)| map(x, y)).collect();
            m.faces.push(Face {
                corners: (0..6).map(|v| Corner { v, vt: None, vn: None }).collect(),
                group: None,
            });
            let tris = m.triangulated();
            assert_eq!(tris.len(), 4);
            let area: f32 = tris
                .iter()
                .map(|t| {
                    let p = |i: u32| m.positions[i as usize];
                    (p(t[1]) - p(t[0])).cross(p(t[2]) - p(t[0])).length() * 0.5
                })
                .sum();
            assert!((area - 7.0).abs() < 1e-4);
        }
    }

    #[test]
    fn degenerate_faces_give_no_triangles() {
        let mut m = ObjMesh::default();
        m.positions = vec![Vec3::ZERO, Vec3::X, Vec3::new(2.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0)];
        m.faces.push(Face {
            corners: (0..4).map(|v| Corner { v, vt: None, vn: None }).collect(),
            group: None,
        });
        assert!(m.face_triangles(0).is_empty());
        assert!(m.face_triangles(99).is_empty());
    }
}
