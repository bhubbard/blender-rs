//! BMesh: Blender's boundary-representation half-edge mesh modeling kernel in Safe Rust.
//!
//! Mirrors `source/blender/bmesh` with generational memory pools to guarantee
//! 100% memory safety without self-referential pointer hazards.
use blender_dna::{CustomDataType, MeshElemFlags};
use blender_math::{normal_tri_v3, Vec3};
use blender_mem::{Handle, MemPool};
use hashbrown::HashMap;
use thiserror::Error;
#[derive(Error, Debug, PartialEq)]
pub enum BMeshError {
#[error("Invalid vertex handle: {0:?}")]
InvalidVert(VertHandle),
#[error("Invalid edge handle: {0:?}")]
InvalidEdge(EdgeHandle),
#[error("Invalid face handle: {0:?}")]
InvalidFace(FaceHandle),
#[error("Degenerate edge: v1 and v2 cannot be the same vertex")]
DegenerateEdge,
#[error("Duplicate edge between vertex {0:?} and {1:?}")]
DuplicateEdge(VertHandle, VertHandle),
#[error("Invalid split: vertices must be distinct, in the face, and non-adjacent")]
InvalidSplit,
#[error("Invalid join: edge must be shared by exactly two distinct faces")]
InvalidJoin,
#[error("Invalid dissolve: vertex must have exactly two distinct neighbours")]
InvalidDissolve,
#[error("Face must have at least 3 vertices")]
DegenerateFace,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ElemType {
Vert = 1,
Edge = 2,
Loop = 4,
Face = 8,
}
pub type VertHandle = Handle<BMVert>;
pub type EdgeHandle = Handle<BMEdge>;
pub type LoopHandle = Handle<BMLoop>;
pub type FaceHandle = Handle<BMFace>;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BMHeader {
pub htype: ElemType,
pub flags: MeshElemFlags,
pub api_flag: u8,
pub index: i32,
}
impl BMHeader {
pub const fn new(htype: ElemType) -> Self {
Self {
htype,
flags: MeshElemFlags::empty(),
api_flag: 0,
index: -1,
}
}
}
/// A BMesh vertex.
#[derive(Debug, Clone, PartialEq)]
pub struct BMVert {
pub head: BMHeader,
pub co: Vec3,
pub no: Vec3,
/// Reference to one edge using this vertex (entry point for disk cycle).
pub edge: Option<EdgeHandle>,
}
/// A BMesh edge connecting two vertices.
#[derive(Debug, Clone, PartialEq)]
pub struct BMEdge {
pub head: BMHeader,
pub v1: VertHandle,
pub v2: VertHandle,
/// Reference to one loop in the radial cycle around this edge.
pub loop_: Option<LoopHandle>,
/// Circular linked disk cycle around v1
pub v1_disk_prev: Option<EdgeHandle>,
pub v1_disk_next: Option<EdgeHandle>,
/// Circular linked disk cycle around v2
pub v2_disk_prev: Option<EdgeHandle>,
pub v2_disk_next: Option<EdgeHandle>,
}
/// A BMesh loop (face corner).
#[derive(Debug, Clone, PartialEq)]
pub struct BMLoop {
pub head: BMHeader,
pub v: VertHandle,
pub e: EdgeHandle,
pub f: FaceHandle,
/// Next loop in polygon perimeter
pub next: LoopHandle,
/// Previous loop in polygon perimeter
pub prev: LoopHandle,
/// Radial cycle around edge
pub radial_next: LoopHandle,
pub radial_prev: LoopHandle,
}
/// A BMesh polygon face.
#[derive(Debug, Clone, PartialEq)]
pub struct BMFace {
pub head: BMHeader,
pub l_first: LoopHandle,
pub len: u32,
pub no: Vec3,
}
/// Custom data storage for vertices, edges, loops, and faces.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CustomData {
pub float_layers: HashMap<(u32, CustomDataType), f32>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListBaseT<T> {
pub _marker: core::marker::PhantomData<T>,
}
impl<T> Default for ListBaseT<T> {
fn default() -> Self {
Self {
_marker: core::marker::PhantomData,
}
}
}
pub use blender_mem::BLI_mempool::BLI_mempool;
pub use blender_mem::BLI_mempool::BLI_mempool_chunk;
pub use blender_mem::BLI_mempool::BLI_mempool_iter;
pub use blender_mem::{GHash, GHashIterator, GHashIterState, GSet, Entry, GHashPair};
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MLoopNorSpaceArray {
pub _opaque: [u8; 0],
}
/// The BMesh instance owning memory pools and mesh elements.
pub struct BMesh {
pub vpool: MemPool<BMVert>,
pub epool: MemPool<BMEdge>,
pub lpool: MemPool<BMLoop>,
pub fpool: MemPool<BMFace>,
pub vdata: CustomData,
pub edata: CustomData,
pub ldata: CustomData,
pub pdata: CustomData,
}
impl Default for BMesh {
fn default() -> Self {
Self::new()
}
}
impl BMesh {
pub fn new() -> Self {
Self {
vpool: MemPool::new(),
epool: MemPool::new(),
lpool: MemPool::new(),
fpool: MemPool::new(),
vdata: CustomData::default(),
edata: CustomData::default(),
ldata: CustomData::default(),
pdata: CustomData::default(),
}
}
#[inline]
pub fn totvert(&self) -> usize {
self.vpool.len()
}
#[inline]
pub fn totedge(&self) -> usize {
self.epool.len()
}
#[inline]
pub fn totface(&self) -> usize {
self.fpool.len()
}
#[inline]
pub fn elem_count(&self, elem_type: ElemType) -> usize {
match elem_type {
ElemType::Vert => self.vpool.len(),
ElemType::Edge => self.epool.len(),
ElemType::Loop => self.lpool.len(),
ElemType::Face => self.fpool.len(),
}
}
/// Creates a new vertex in the mesh, optionally copying custom data from an example vertex.
pub fn vert_create(
&mut self,
co: Option<Vec3>,
example: Option<VertHandle>,
) -> VertHandle {
let coord = co.unwrap_or(Vec3::ZERO);
let vert = BMVert {
head: BMHeader::new(ElemType::Vert),
co: coord,
no: Vec3::ZERO,
edge: None,
};
let handle = self.vpool.alloc(vert);
// Copy custom data from example if provided (per Blender upstream behavior)
if let Some(ex_handle) = example {
let keys: Vec<(u32, CustomDataType)> = self
.vdata
.float_layers
.keys()
.filter(|(idx, _)| *idx == ex_handle.index)
.cloned()
.collect();
for (idx, dt) in keys {
if let Some(&val) = self.vdata.float_layers.get(&(idx, dt)) {
self.vdata.float_layers.insert((handle.index, dt), val);
}
}
}
handle
}
/// Sets element selection flag.
pub fn vert_select_set(&mut self, handle: VertHandle, select: bool) -> bool {
if let Some(v) = self.vpool.get_mut(handle) {
v.head.flags.set(MeshElemFlags::SELECT, select);
true
} else {
false
}
}
/// Tests whether a vertex has a specific flag set.
pub fn vert_flag_test(&self, handle: VertHandle, flag: MeshElemFlags) -> bool {
if let Some(v) = self.vpool.get(handle) {
v.head.flags.contains(flag)
} else {
false
}
}
/// Creates an edge connecting two vertices.
pub fn edge_create(
&mut self,
v1: VertHandle,
v2: VertHandle,
_example: Option<EdgeHandle>,
) -> Result<EdgeHandle, BMeshError> {
if v1 == v2 {
return Err(BMeshError::DegenerateEdge);
}
let edge = BMEdge {
head: BMHeader::new(ElemType::Edge),
v1,
v2,
loop_: None,
v1_disk_prev: None,
v1_disk_next: None,
v2_disk_prev: None,
v2_disk_next: None,
};
let e_handle = self.epool.alloc(edge);
// Link into the disk cycle of both endpoints.
self.disk_edge_append(v1, e_handle);
self.disk_edge_append(v2, e_handle);
Ok(e_handle)
}
/// Creates an edge between two vertices or returns existing one if already present.
pub fn edge_find_or_create(
&mut self,
v1: VertHandle,
v2: VertHandle,
) -> Result<EdgeHandle, BMeshError> {
if let Some(e) = self.edge_exists(v1, v2) {
return Ok(e);
}
self.edge_create(v1, v2, None)
}
/// Creates a polygonal face from an ordered slice of vertices.
pub fn face_create(
&mut self,
verts: &[VertHandle],
_example: Option<FaceHandle>,
) -> Result<FaceHandle, BMeshError> {
if verts.len() < 3 {
return Err(BMeshError::DegenerateFace);
}
let face_handle = self.fpool.alloc(BMFace {
head: BMHeader::new(ElemType::Face),
l_first: LoopHandle::invalid(),
len: verts.len() as u32,
no: Vec3::ZERO,
});
// Create edges and loops around polygon perimeter
let n = verts.len();
let mut loop_handles = Vec::with_capacity(n);
for i in 0..n {
let v_curr = verts[i];
let v_next = verts[(i + 1) % n];
let edge = self.edge_find_or_create(v_curr, v_next)?;
let loop_h = self.lpool.alloc(BMLoop {
head: BMHeader::new(ElemType::Loop),
v: v_curr,
e: edge,
f: face_handle,
next: LoopHandle::invalid(),
prev: LoopHandle::invalid(),
radial_next: LoopHandle::invalid(),
radial_prev: LoopHandle::invalid(),
});
self.radial_loop_append(edge, loop_h);
loop_handles.push(loop_h);
}
// Link loop cycle
for i in 0..n {
let prev_h = loop_handles[(i + n - 1) % n];
let next_h = loop_handles[(i + 1) % n];
let curr_h = loop_handles[i];
if let Some(l) = self.lpool.get_mut(curr_h) {
l.prev = prev_h;
l.next = next_h;
}
}
// Set face first loop
if let Some(f) = self.fpool.get_mut(face_handle) {
f.l_first = loop_handles[0];
}
// Calculate face normal
if verts.len() >= 3 {
let p0 = self.vpool.get(verts[0]).map(|v| v.co).unwrap_or(Vec3::ZERO);
let p1 = self.vpool.get(verts[1]).map(|v| v.co).unwrap_or(Vec3::ZERO);
let p2 = self.vpool.get(verts[2]).map(|v| v.co).unwrap_or(Vec3::ZERO);
if let Some(normal) = normal_tri_v3(p0, p1, p2) {
if let Some(f) = self.fpool.get_mut(face_handle) {
f.no = normal;
}
}
}
Ok(face_handle)
}
/// Sets custom float layer data for an element.
pub fn elem_float_data_set(
&mut self,
handle: VertHandle,
data_type: CustomDataType,
value: f32,
) {
self.vdata.float_layers.insert((handle.index, data_type), value);
}
/// Gets custom float layer data for an element.
pub fn elem_float_data_get(
&self,
handle: VertHandle,
data_type: CustomDataType,
) -> Option<f32> {
self.vdata.float_layers.get(&(handle.index, data_type)).copied()
}
}
#[cfg(test)]
mod tests {
use super::*;
use blender_math::is_zero_v3;
/// 1:1 port of upstream Blender `BMVertCreate` test from
/// `source/blender/bmesh/tests/bmesh_core_test.cc`.
#[test]
fn test_upstream_bm_vert_create() {
let mut bm = BMesh::new();
assert_eq!(bm.totvert(), 0);
let co1 = Vec3::new(1.0, 2.0, 0.0);
let bv1 = bm.vert_create(Some(co1), None);
let v1 = bm.vpool.get(bv1).unwrap();
assert_eq!(v1.co.x, 1.0);
assert_eq!(v1.co.y, 2.0);
assert_eq!(v1.co.z, 0.0);
assert!(is_zero_v3(v1.no));
assert_eq!(v1.head.htype, ElemType::Vert);
assert_eq!(v1.head.flags, MeshElemFlags::empty());
assert_eq!(v1.head.api_flag, 0);
let bv2 = bm.vert_create(None, None);
let v2 = bm.vpool.get(bv2).unwrap();
assert!(is_zero_v3(v2.co));
// Create with example should copy custom data but not select flag
bm.vert_select_set(bv2, true);
assert!(bm.vert_flag_test(bv2, MeshElemFlags::SELECT));
bm.elem_float_data_set(bv2, CustomDataType::PropFloat, 1.5);
let bv3 = bm.vert_create(Some(co1), Some(bv2));
assert!(!bm.vert_flag_test(bv3, MeshElemFlags::SELECT));
assert_eq!(
bm.elem_float_data_get(bv3, CustomDataType::PropFloat),
Some(1.5)
);
assert_eq!(bm.elem_count(ElemType::Vert), 3);
}
#[test]
fn test_face_create_and_normal() {
let mut bm = BMesh::new();
let v1 = bm.vert_create(Some(Vec3::new(0.0, 0.0, 0.0)), None);
let v2 = bm.vert_create(Some(Vec3::new(1.0, 0.0, 0.0)), None);
let v3 = bm.vert_create(Some(Vec3::new(0.0, 1.0, 0.0)), None);
let face_handle = bm.face_create(&[v1, v2, v3], None).unwrap();
let face = bm.fpool.get(face_handle).unwrap();
assert_eq!(face.len, 3);
assert!((face.no - Vec3::Z).length() < 1e-5);
assert_eq!(bm.totedge(), 3);
assert_eq!(bm.totface(), 1);
}
}
pub mod BKE_editmesh;
pub mod bmesh_geomq;
pub mod bmesh_separate;
pub mod bmesh_cycles;
pub mod bmesh_kill;
pub mod bmesh_split;
pub mod bmesh_query;
pub mod bmesh_join;
pub mod bmesh_dissolve;
pub mod bmesh_faceops;
pub mod bmesh_vdata;
pub mod bmesh_core;
pub mod bmesh_edgeloop;
pub mod bmesh_marking;
pub mod BLI_expr_pylike_eval;
pub mod bmesh_mesh_tessellate;
pub mod node_geo_input_mesh_edge_vertices;
pub mod bmesh;
pub mod ANIM_bone_collections;
pub mod bmesh_class;
pub use bmesh_class::{
    BMElem, BMElemF, BMFlagLayer, BMDiskLink, BMVert_OFlag, BMEdge_OFlag, BMFace_OFlag,
    BMLoopList, BMLoopNorEditData, BMLoopNorEditDataArray,
    BM_VERT, BM_EDGE, BM_LOOP, BM_FACE,
    BM_ELEM_SELECT, BM_ELEM_HIDDEN, BM_ELEM_SEAM, BM_ELEM_SMOOTH,
    BM_ELEM_TAG, BM_ELEM_SELECT_UV, BM_ELEM_TAG_ALT, BM_ELEM_INTERNAL_TAG,
};






pub mod bmesh_iterators;

















pub mod bmesh_log;

pub mod bmesh_mesh;

pub mod bmesh_mesh_convert;

pub mod bmesh_operator_api;







pub mod bmesh_mesh_validate;

pub mod bmesh_mods;

pub mod bmesh_operator_api_inline;






pub mod BLI_generic_vector_array;


pub mod BKE_attribute_legacy_convert;

pub mod BKE_editmesh_bvh;

pub mod BKE_editmesh_tangent;







pub mod eevee_shadow_shared;



pub mod bmesh_callback_generic;


pub mod bmesh_construct;




pub mod bmesh_interp;











pub mod bmesh_operators;

pub mod bmesh_operators_private;









pub mod BKE_mesh_legacy_convert;





































pub mod eevee_sync;






































































































pub mod COM_domain;












pub mod bmesh_opdefines;


pub mod bmesh_polygon;

pub mod bmesh_polygon_edgenet;

pub mod bmesh_private;




pub mod COM_meta_data;





pub mod rna_animation;



pub mod wm_init_exit;
















pub mod COM_cached_image;

pub mod COM_morphological_distance_feather_weights;

pub mod bmesh_beautify;


pub mod bmesh_intersect;

pub mod bmesh_path;

pub mod bmesh_path_region;

pub mod bmesh_path_uv;












pub mod depsgraph_light_linking;



pub mod cached_image;


















pub mod pbvh_bmesh;
pub mod bmesh_delete_impl;
pub use bmesh_delete_impl::*;
pub mod bmesh_construct_impl;
pub use bmesh_construct_impl::*;
