//! Scene graph and primitive generators for blender-app.

use blender_bmesh::{BMesh, VertHandle};
use blender_math::Vec3;

pub struct SceneContext {
    #[allow(dead_code)]
    pub name: String,
    pub bmesh: BMesh,
}

impl SceneContext {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            bmesh: BMesh::new(),
        }
    }

    /// Creates a standard 2x2x2 Blender default cube.
    pub fn create_default_cube(&mut self) -> anyhow::Result<()> {
        self.bmesh = BMesh::new();
        let s = 1.0f32;
        let coords = [
            Vec3::new(-s, -s, -s), // 0
            Vec3::new( s, -s, -s), // 1
            Vec3::new( s,  s, -s), // 2
            Vec3::new(-s,  s, -s), // 3
            Vec3::new(-s, -s,  s), // 4
            Vec3::new( s, -s,  s), // 5
            Vec3::new( s,  s,  s), // 6
            Vec3::new(-s,  s,  s), // 7
        ];

        let mut v = Vec::with_capacity(8);
        for c in coords {
            v.push(self.bmesh.vert_create(Some(c), None));
        }

        // 6 quad faces with standard CCW winding
        let faces = [
            [v[0], v[3], v[2], v[1]], // -Z bottom
            [v[4], v[5], v[6], v[7]], // +Z top
            [v[0], v[1], v[5], v[4]], // -Y front
            [v[2], v[3], v[7], v[6]], // +Y back
            [v[0], v[4], v[7], v[3]], // -X left
            [v[1], v[2], v[6], v[5]], // +X right
        ];

        for f in faces {
            self.bmesh.face_create(&f, None)?;
        }

        self.bmesh.validate_topology().map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(())
    }

    /// Creates a UV sphere with given segments and rings.
    pub fn create_uv_sphere(&mut self, radius: f32, segments: usize, rings: usize) -> anyhow::Result<()> {
        self.bmesh = BMesh::new();
        let mut vert_grid: Vec<Vec<VertHandle>> = Vec::new();

        // Top pole
        let top_vert = self.bmesh.vert_create(Some(Vec3::new(0.0, 0.0, radius)), None);

        // Latitude rings
        for r in 1..rings {
            let theta = std::f32::consts::PI * (r as f32) / (rings as f32);
            let z = radius * theta.cos();
            let r_sin = radius * theta.sin();

            let mut ring_verts = Vec::new();
            for s in 0..segments {
                let phi = 2.0 * std::f32::consts::PI * (s as f32) / (segments as f32);
                let x = r_sin * phi.cos();
                let y = r_sin * phi.sin();
                ring_verts.push(self.bmesh.vert_create(Some(Vec3::new(x, y, z)), None));
            }
            vert_grid.push(ring_verts);
        }

        // Bottom pole
        let bot_vert = self.bmesh.vert_create(Some(Vec3::new(0.0, 0.0, -radius)), None);

        // Top cap triangles
        if !vert_grid.is_empty() {
            let first_ring = &vert_grid[0];
            for s in 0..segments {
                let next_s = (s + 1) % segments;
                self.bmesh.face_create(&[top_vert, first_ring[s], first_ring[next_s]], None)?;
            }
        }

        // Body quad strips
        for r in 0..vert_grid.len().saturating_sub(1) {
            let ring_a = &vert_grid[r];
            let ring_b = &vert_grid[r + 1];
            for s in 0..segments {
                let next_s = (s + 1) % segments;
                self.bmesh.face_create(&[ring_a[s], ring_b[s], ring_b[next_s], ring_a[next_s]], None)?;
            }
        }

        // Bottom cap triangles
        if let Some(last_ring) = vert_grid.last() {
            for s in 0..segments {
                let next_s = (s + 1) % segments;
                self.bmesh.face_create(&[last_ring[next_s], last_ring[s], bot_vert], None)?;
            }
        }

        self.bmesh.validate_topology().map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(())
    }

    /// Performs edge subdivision on the BMesh.
    pub fn subdivide_edges(&mut self, factor: f32) -> anyhow::Result<usize> {
        let edges: Vec<_> = self.bmesh.epool.iter().map(|(h, _)| h).collect();
        let mut count = 0;
        for edge_h in edges {
            if self.bmesh.edge_split(edge_h, factor).is_ok() {
                count += 1;
            }
        }
        self.bmesh.validate_topology().map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(count)
    }

    /// Extracts all edge vertex coordinate pairs for rendering.
    pub fn extract_edges_lines(&self) -> Vec<(Vec3, Vec3)> {
        let mut lines = Vec::new();
        for (_, edge) in self.bmesh.epool.iter() {
            if let (Some(v1), Some(v2)) = (self.bmesh.vpool.get(edge.v1), self.bmesh.vpool.get(edge.v2)) {
                lines.push((v1.co, v2.co));
            }
        }
        lines
    }
}
