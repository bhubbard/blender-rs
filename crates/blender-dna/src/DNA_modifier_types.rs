//! Mechanically generated via blender-cluster AST zero-token fast-path

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_imports)]

#[allow(unused_imports)]
use crate::*;

#[allow(non_camel_case_types)]
type int32_t = i32;
#[allow(non_camel_case_types)]
type uint32_t = u32;
#[allow(non_camel_case_types)]
type int16_t = i16;
#[allow(non_camel_case_types)]
type uint16_t = u16;
#[allow(non_camel_case_types)]
type int64_t = i64;
#[allow(non_camel_case_types)]
type uint64_t = u64;
#[allow(non_camel_case_types)]
type int8_t = i8;
#[allow(non_camel_case_types)]
type uint8_t = u8;
#[allow(non_camel_case_types)]
type uchar = u8;
#[allow(non_camel_case_types)]
type ushort = u16;
#[allow(non_camel_case_types)]
type uint = u32;
#[allow(non_camel_case_types)]
type ulong = u64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModifierType(pub i32);

impl ModifierType {
    pub const eModifierType_None: Self = Self((0) as i32);
    pub const eModifierType_Subsurf: Self = Self((1) as i32);
    pub const eModifierType_Lattice: Self = Self((2) as i32);
    pub const eModifierType_Curve: Self = Self((3) as i32);
    pub const eModifierType_Build: Self = Self((4) as i32);
    pub const eModifierType_Mirror: Self = Self((5) as i32);
    pub const eModifierType_Decimate: Self = Self((6) as i32);
    pub const eModifierType_Wave: Self = Self((7) as i32);
    pub const eModifierType_Armature: Self = Self((8) as i32);
    pub const eModifierType_Hook: Self = Self((9) as i32);
    pub const eModifierType_Softbody: Self = Self((10) as i32);
    pub const eModifierType_Boolean: Self = Self((11) as i32);
    pub const eModifierType_Array: Self = Self((12) as i32);
    pub const eModifierType_EdgeSplit: Self = Self((13) as i32);
    pub const eModifierType_Displace: Self = Self((14) as i32);
    pub const eModifierType_UVProject: Self = Self((15) as i32);
    pub const eModifierType_Smooth: Self = Self((16) as i32);
    pub const eModifierType_Cast: Self = Self((17) as i32);
    pub const eModifierType_MeshDeform: Self = Self((18) as i32);
    pub const eModifierType_ParticleSystem: Self = Self((19) as i32);
    pub const eModifierType_ParticleInstance: Self = Self((20) as i32);
    pub const eModifierType_Explode: Self = Self((21) as i32);
    pub const eModifierType_Cloth: Self = Self((22) as i32);
    pub const eModifierType_Collision: Self = Self((23) as i32);
    pub const eModifierType_Bevel: Self = Self((24) as i32);
    pub const eModifierType_Shrinkwrap: Self = Self((25) as i32);
    pub const eModifierType_Fluidsim: Self = Self((26) as i32);
    pub const eModifierType_Mask: Self = Self((27) as i32);
    pub const eModifierType_SimpleDeform: Self = Self((28) as i32);
    pub const eModifierType_Multires: Self = Self((29) as i32);
    pub const eModifierType_Surface: Self = Self((30) as i32);
    pub const eModifierType_Smoke: Self = Self((31) as i32);
    pub const eModifierType_ShapeKey: Self = Self((32) as i32);
    pub const eModifierType_Solidify: Self = Self((33) as i32);
    pub const eModifierType_Screw: Self = Self((34) as i32);
    pub const eModifierType_Warp: Self = Self((35) as i32);
    pub const eModifierType_WeightVGEdit: Self = Self((36) as i32);
    pub const eModifierType_WeightVGMix: Self = Self((37) as i32);
    pub const eModifierType_WeightVGProximity: Self = Self((38) as i32);
    pub const eModifierType_Ocean: Self = Self((39) as i32);
    pub const eModifierType_DynamicPaint: Self = Self((40) as i32);
    pub const eModifierType_Remesh: Self = Self((41) as i32);
    pub const eModifierType_Skin: Self = Self((42) as i32);
    pub const eModifierType_LaplacianSmooth: Self = Self((43) as i32);
    pub const eModifierType_Triangulate: Self = Self((44) as i32);
    pub const eModifierType_UVWarp: Self = Self((45) as i32);
    pub const eModifierType_MeshCache: Self = Self((46) as i32);
    pub const eModifierType_LaplacianDeform: Self = Self((47) as i32);
    pub const eModifierType_Wireframe: Self = Self((48) as i32);
    pub const eModifierType_DataTransfer: Self = Self((49) as i32);
    pub const eModifierType_NormalEdit: Self = Self((50) as i32);
    pub const eModifierType_CorrectiveSmooth: Self = Self((51) as i32);
    pub const eModifierType_MeshSequenceCache: Self = Self((52) as i32);
    pub const eModifierType_SurfaceDeform: Self = Self((53) as i32);
    pub const eModifierType_WeightedNormal: Self = Self((54) as i32);
    pub const eModifierType_Weld: Self = Self((55) as i32);
    pub const eModifierType_Fluid: Self = Self((56) as i32);
    pub const eModifierType_Nodes: Self = Self((57) as i32);
    pub const eModifierType_MeshToVolume: Self = Self((58) as i32);
    pub const eModifierType_VolumeDisplace: Self = Self((59) as i32);
    pub const eModifierType_VolumeToMesh: Self = Self((60) as i32);
    pub const eModifierType_GreasePencilOpacity: Self = Self((61) as i32);
    pub const eModifierType_GreasePencilSubdiv: Self = Self((62) as i32);
    pub const eModifierType_GreasePencilColor: Self = Self((63) as i32);
    pub const eModifierType_GreasePencilTint: Self = Self((64) as i32);
    pub const eModifierType_GreasePencilSmooth: Self = Self((65) as i32);
    pub const eModifierType_GreasePencilOffset: Self = Self((66) as i32);
    pub const eModifierType_GreasePencilNoise: Self = Self((67) as i32);
    pub const eModifierType_GreasePencilMirror: Self = Self((68) as i32);
    pub const eModifierType_GreasePencilThickness: Self = Self((69) as i32);
    pub const eModifierType_GreasePencilLattice: Self = Self((70) as i32);
    pub const eModifierType_GreasePencilDash: Self = Self((71) as i32);
    pub const eModifierType_GreasePencilMultiply: Self = Self((72) as i32);
    pub const eModifierType_GreasePencilLength: Self = Self((73) as i32);
    pub const eModifierType_GreasePencilWeightAngle: Self = Self((74) as i32);
    pub const eModifierType_GreasePencilArray: Self = Self((75) as i32);
    pub const eModifierType_GreasePencilWeightProximity: Self = Self((76) as i32);
    pub const eModifierType_GreasePencilHook: Self = Self((77) as i32);
    pub const eModifierType_GreasePencilLineart: Self = Self((78) as i32);
    pub const eModifierType_GreasePencilArmature: Self = Self((79) as i32);
    pub const eModifierType_GreasePencilTime: Self = Self((80) as i32);
    pub const eModifierType_GreasePencilEnvelope: Self = Self((81) as i32);
    pub const eModifierType_GreasePencilOutline: Self = Self((82) as i32);
    pub const eModifierType_GreasePencilShrinkwrap: Self = Self((83) as i32);
    pub const eModifierType_GreasePencilBuild: Self = Self((84) as i32);
    pub const eModifierType_GreasePencilSimplify: Self = Self((85) as i32);
    pub const eModifierType_GreasePencilTexture: Self = Self((86) as i32);
    pub const NUM_MODIFIER_TYPES: Self = Self(87 as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModifierMode(pub u32);

impl ModifierMode {
    pub const eModifierMode_Realtime: Self = Self(((1 << 0)) as u32);
    pub const eModifierMode_Render: Self = Self(((1 << 1)) as u32);
    pub const eModifierMode_Editmode: Self = Self(((1 << 2)) as u32);
    pub const eModifierMode_OnCage: Self = Self(((1 << 3)) as u32);
    pub const eModifierMode_Expanded_DEPRECATED: Self = Self(((1 << 4)) as u32);
    pub const eModifierMode_Virtual: Self = Self(((1 << 5)) as u32);
    pub const eModifierMode_ApplyOnSpline: Self = Self(((1 << 6)) as u32);
    pub const eModifierMode_DisableTemporary: Self = Self(7 as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModifierFlag(pub i16);

impl ModifierFlag {
    pub const eModifierFlag_OverrideLibrary_Local: Self = Self(((1 << 0)) as i16);
    pub const eModifierFlag_SharedCaches: Self = Self(((1 << 1)) as i16);
    pub const eModifierFlag_Active: Self = Self(((1 << 2)) as i16);
    pub const eModifierFlag_UserModified: Self = Self(((1 << 3)) as i16);
    pub const eModifierFlag_PinLast: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SubsurfModifierFlag(pub i16);

impl SubsurfModifierFlag {
    pub const eSubsurfModifierFlag_Incremental: Self = Self(((1 << 0)) as i16);
    pub const eSubsurfModifierFlag_DebugIncr: Self = Self(((1 << 1)) as i16);
    pub const eSubsurfModifierFlag_ControlEdges: Self = Self(((1 << 2)) as i16);
    pub const eSubsurfModifierFlag_SubsurfUv_DEPRECATED: Self = Self(((1 << 3)) as i16);
    pub const eSubsurfModifierFlag_UseCrease: Self = Self(((1 << 4)) as i16);
    pub const eSubsurfModifierFlag_UseCustomNormals: Self = Self(((1 << 5)) as i16);
    pub const eSubsurfModifierFlag_UseRecursiveSubdivision: Self = Self(((1 << 6)) as i16);
    pub const eSubsurfModifierFlag_UseAdaptiveSubdivision: Self = Self(((1 << 7)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSubsurfAdaptiveSpace(pub i16);

impl eSubsurfAdaptiveSpace {
    pub const SUBSURF_ADAPTIVE_SPACE_PIXEL: Self = Self((0) as i16);
    pub const SUBSURF_ADAPTIVE_SPACE_OBJECT: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSubsurfModifierType(pub i16);

impl eSubsurfModifierType {
    pub const SUBSURF_TYPE_CATMULL_CLARK: Self = Self((0) as i16);
    pub const SUBSURF_TYPE_SIMPLE: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSubsurfUVSmooth(pub i16);

impl eSubsurfUVSmooth {
    pub const SUBSURF_UV_SMOOTH_NONE: Self = Self((0) as i16);
    pub const SUBSURF_UV_SMOOTH_PRESERVE_CORNERS: Self = Self((1) as i16);
    pub const SUBSURF_UV_SMOOTH_PRESERVE_CORNERS_AND_JUNCTIONS: Self = Self((2) as i16);
    pub const SUBSURF_UV_SMOOTH_PRESERVE_CORNERS_JUNCTIONS_AND_CONCAVE: Self = Self((3) as i16);
    pub const SUBSURF_UV_SMOOTH_PRESERVE_BOUNDARIES: Self = Self((4) as i16);
    pub const SUBSURF_UV_SMOOTH_ALL: Self = Self((5) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eSubsurfBoundarySmooth(pub i16);

impl eSubsurfBoundarySmooth {
    pub const SUBSURF_BOUNDARY_SMOOTH_ALL: Self = Self((0) as i16);
    pub const SUBSURF_BOUNDARY_SMOOTH_PRESERVE_CORNERS: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LatticeModifierFlag(pub i16);

impl LatticeModifierFlag {
    pub const MOD_LATTICE_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CurveModifierFlag(pub i16);

impl CurveModifierFlag {
    pub const MOD_CURVE_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CurveModifierDefaultAxis(pub i16);

impl CurveModifierDefaultAxis {
    pub const MOD_CURVE_POSX: Self = Self((1) as i16);
    pub const MOD_CURVE_POSY: Self = Self((2) as i16);
    pub const MOD_CURVE_POSZ: Self = Self((3) as i16);
    pub const MOD_CURVE_NEGX: Self = Self((4) as i16);
    pub const MOD_CURVE_NEGY: Self = Self((5) as i16);
    pub const MOD_CURVE_NEGZ: Self = Self((6) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BuildModifierFlag(pub i16);

impl BuildModifierFlag {
    pub const MOD_BUILD_FLAG_RANDOMIZE: Self = Self(((1 << 0)) as i16);
    pub const MOD_BUILD_FLAG_REVERSE: Self = Self(((1 << 1)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MaskModifierMode(pub i16);

impl MaskModifierMode {
    pub const MOD_MASK_MODE_VGROUP: Self = Self((0) as i16);
    pub const MOD_MASK_MODE_ARM: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MaskModifierFlag(pub i16);

impl MaskModifierFlag {
    pub const MOD_MASK_INV: Self = Self(((1 << 0)) as i16);
    pub const MOD_MASK_SMOOTH: Self = Self(((1 << 1)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ArrayModifierFitType(pub i32);

impl ArrayModifierFitType {
    pub const MOD_ARR_FIXEDCOUNT: Self = Self((0) as i32);
    pub const MOD_ARR_FITLENGTH: Self = Self((1) as i32);
    pub const MOD_ARR_FITCURVE: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ArrayModifierOffsetType(pub i32);

impl ArrayModifierOffsetType {
    pub const MOD_ARR_OFF_CONST: Self = Self(((1 << 0)) as i32);
    pub const MOD_ARR_OFF_RELATIVE: Self = Self(((1 << 1)) as i32);
    pub const MOD_ARR_OFF_OBJ: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ArrayModifierFlag(pub i32);

impl ArrayModifierFlag {
    pub const MOD_ARR_MERGE: Self = Self(((1 << 0)) as i32);
    pub const MOD_ARR_MERGEFINAL: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MirrorModifierFlag(pub i16);

impl MirrorModifierFlag {
    pub const MOD_MIR_CLIPPING: Self = Self(((1 << 0)) as i16);
    pub const MOD_MIR_MIRROR_U: Self = Self(((1 << 1)) as i16);
    pub const MOD_MIR_MIRROR_V: Self = Self(((1 << 2)) as i16);
    pub const MOD_MIR_AXIS_X: Self = Self(((1 << 3)) as i16);
    pub const MOD_MIR_AXIS_Y: Self = Self(((1 << 4)) as i16);
    pub const MOD_MIR_AXIS_Z: Self = Self(((1 << 5)) as i16);
    pub const MOD_MIR_VGROUP: Self = Self(((1 << 6)) as i16);
    pub const MOD_MIR_NO_MERGE: Self = Self(((1 << 7)) as i16);
    pub const MOD_MIR_BISECT_AXIS_X: Self = Self(((1 << 8)) as i16);
    pub const MOD_MIR_BISECT_AXIS_Y: Self = Self(((1 << 9)) as i16);
    pub const MOD_MIR_BISECT_AXIS_Z: Self = Self(((1 << 10)) as i16);
    pub const MOD_MIR_BISECT_FLIP_AXIS_X: Self = Self(((1 << 11)) as i16);
    pub const MOD_MIR_BISECT_FLIP_AXIS_Y: Self = Self(((1 << 12)) as i16);
    pub const MOD_MIR_BISECT_FLIP_AXIS_Z: Self = Self(((1 << 13)) as i16);
    pub const MOD_MIR_MIRROR_UDIM: Self = Self(((1 << 14)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EdgeSplitModifierFlag(pub i32);

impl EdgeSplitModifierFlag {
    pub const MOD_EDGESPLIT_FROMANGLE: Self = Self(((1 << 1)) as i32);
    pub const MOD_EDGESPLIT_FROMFLAG: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierFlag(pub i16);

impl BevelModifierFlag {
    pub const MOD_BEVEL_VERT_DEPRECATED: Self = Self(((1 << 1)) as i16);
    pub const MOD_BEVEL_INVERT_VGROUP: Self = Self(((1 << 2)) as i16);
    pub const MOD_BEVEL_ANGLE: Self = Self(((1 << 3)) as i16);
    pub const MOD_BEVEL_WEIGHT: Self = Self(((1 << 4)) as i16);
    pub const MOD_BEVEL_VGROUP: Self = Self(((1 << 5)) as i16);
    pub const MOD_BEVEL_CUSTOM_PROFILE_DEPRECATED: Self = Self(((1 << 7)) as i16);
    pub const MOD_BEVEL_OVERLAP_OK: Self = Self(((1 << 13)) as i16);
    pub const MOD_BEVEL_EVEN_WIDTHS: Self = Self(((1 << 14)) as i16);
    pub const MOD_BEVEL_HARDEN_NORMALS: Self = Self(8 as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierValFlag(pub i16);

impl BevelModifierValFlag {
    pub const MOD_BEVEL_AMT_OFFSET: Self = Self((0) as i16);
    pub const MOD_BEVEL_AMT_WIDTH: Self = Self((1) as i16);
    pub const MOD_BEVEL_AMT_DEPTH: Self = Self((2) as i16);
    pub const MOD_BEVEL_AMT_PERCENT: Self = Self((3) as i16);
    pub const MOD_BEVEL_AMT_ABSOLUTE: Self = Self((4) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierProfileType(pub i16);

impl BevelModifierProfileType {
    pub const MOD_BEVEL_PROFILE_SUPERELLIPSE: Self = Self((0) as i16);
    pub const MOD_BEVEL_PROFILE_CUSTOM: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierEdgeFlag(pub i16);

impl BevelModifierEdgeFlag {
    pub const MOD_BEVEL_MARK_SEAM: Self = Self(((1 << 0)) as i16);
    pub const MOD_BEVEL_MARK_SHARP: Self = Self(((1 << 1)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierFaceStrengthMode(pub i16);

impl BevelModifierFaceStrengthMode {
    pub const MOD_BEVEL_FACE_STRENGTH_NONE: Self = Self((0) as i16);
    pub const MOD_BEVEL_FACE_STRENGTH_NEW: Self = Self((1) as i16);
    pub const MOD_BEVEL_FACE_STRENGTH_AFFECTED: Self = Self((2) as i16);
    pub const MOD_BEVEL_FACE_STRENGTH_ALL: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierMiter(pub i16);

impl BevelModifierMiter {
    pub const MOD_BEVEL_MITER_SHARP: Self = Self((0) as i16);
    pub const MOD_BEVEL_MITER_PATCH: Self = Self((1) as i16);
    pub const MOD_BEVEL_MITER_ARC: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierVMeshMethod(pub i16);

impl BevelModifierVMeshMethod {
    pub const MOD_BEVEL_VMESH_ADJ: Self = Self((0) as i16);
    pub const MOD_BEVEL_VMESH_CUTOFF: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BevelModifierAffectType(pub i8);

impl BevelModifierAffectType {
    pub const MOD_BEVEL_AFFECT_VERTICES: Self = Self((0) as i8);
    pub const MOD_BEVEL_AFFECT_EDGES: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FluidModifierType(pub i32);

impl FluidModifierType {
    pub const MOD_FLUID_TYPE_DOMAIN: Self = Self(((1 << 0)) as i32);
    pub const MOD_FLUID_TYPE_FLOW: Self = Self(((1 << 1)) as i32);
    pub const MOD_FLUID_TYPE_EFFEC: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DisplaceModifierFlag(pub i16);

impl DisplaceModifierFlag {
    pub const MOD_DISP_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DisplaceModifierDirection(pub i32);

impl DisplaceModifierDirection {
    pub const MOD_DISP_DIR_X: Self = Self((0) as i32);
    pub const MOD_DISP_DIR_Y: Self = Self((1) as i32);
    pub const MOD_DISP_DIR_Z: Self = Self((2) as i32);
    pub const MOD_DISP_DIR_NOR: Self = Self((3) as i32);
    pub const MOD_DISP_DIR_RGB_XYZ: Self = Self((4) as i32);
    pub const MOD_DISP_DIR_CLNOR: Self = Self((5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DisplaceModifierTexMapping(pub i32);

impl DisplaceModifierTexMapping {
    pub const MOD_DISP_MAP_LOCAL: Self = Self((0) as i32);
    pub const MOD_DISP_MAP_GLOBAL: Self = Self((1) as i32);
    pub const MOD_DISP_MAP_OBJECT: Self = Self((2) as i32);
    pub const MOD_DISP_MAP_UV: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DisplaceModifierSpace(pub i32);

impl DisplaceModifierSpace {
    pub const MOD_DISP_SPACE_LOCAL: Self = Self((0) as i32);
    pub const MOD_DISP_SPACE_GLOBAL: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DecimateModifierFlag(pub i16);

impl DecimateModifierFlag {
    pub const MOD_DECIM_FLAG_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_DECIM_FLAG_TRIANGULATE: Self = Self(((1 << 1)) as i16);
    pub const MOD_DECIM_FLAG_ALL_BOUNDARY_VERTS: Self = Self(((1 << 2)) as i16);
    pub const MOD_DECIM_FLAG_SYMMETRY: Self = Self(((1 << 3)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DecimateModifierMode(pub i16);

impl DecimateModifierMode {
    pub const MOD_DECIM_MODE_COLLAPSE: Self = Self((0) as i16);
    pub const MOD_DECIM_MODE_UNSUBDIV: Self = Self((1) as i16);
    pub const MOD_DECIM_MODE_DISSOLVE: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SmoothModifierFlag(pub i16);

impl SmoothModifierFlag {
    pub const MOD_SMOOTH_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_SMOOTH_X: Self = Self(((1 << 1)) as i16);
    pub const MOD_SMOOTH_Y: Self = Self(((1 << 2)) as i16);
    pub const MOD_SMOOTH_Z: Self = Self(((1 << 3)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CastModifierFlag(pub i16);

impl CastModifierFlag {
    pub const MOD_CAST_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_CAST_X: Self = Self(((1 << 1)) as i16);
    pub const MOD_CAST_Y: Self = Self(((1 << 2)) as i16);
    pub const MOD_CAST_Z: Self = Self(((1 << 3)) as i16);
    pub const MOD_CAST_USE_OB_TRANSFORM: Self = Self(((1 << 4)) as i16);
    pub const MOD_CAST_SIZE_FROM_RADIUS: Self = Self(((1 << 5)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CastModifierType(pub i16);

impl CastModifierType {
    pub const MOD_CAST_TYPE_SPHERE: Self = Self((0) as i16);
    pub const MOD_CAST_TYPE_CYLINDER: Self = Self((1) as i16);
    pub const MOD_CAST_TYPE_CUBOID: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WaveModifierFlag(pub i16);

impl WaveModifierFlag {
    pub const MOD_WAVE_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_WAVE_X: Self = Self(((1 << 1)) as i16);
    pub const MOD_WAVE_Y: Self = Self(((1 << 2)) as i16);
    pub const MOD_WAVE_CYCL: Self = Self(((1 << 3)) as i16);
    pub const MOD_WAVE_NORM: Self = Self(((1 << 4)) as i16);
    pub const MOD_WAVE_NORM_X: Self = Self(((1 << 5)) as i16);
    pub const MOD_WAVE_NORM_Y: Self = Self(((1 << 6)) as i16);
    pub const MOD_WAVE_NORM_Z: Self = Self(((1 << 7)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HookModifierFlag(pub i8);

impl HookModifierFlag {
    pub const MOD_HOOK_UNIFORM_SPACE: Self = Self(((1 << 0)) as i8);
    pub const MOD_HOOK_INVERT_VGROUP: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HookModifierFalloff(pub i8);

impl HookModifierFalloff {
    pub const eHook_Falloff_None: Self = Self((0) as i8);
    pub const eHook_Falloff_Curve: Self = Self((1) as i8);
    pub const eHook_Falloff_Sharp: Self = Self((2) as i8);
    pub const eHook_Falloff_Smooth: Self = Self((3) as i8);
    pub const eHook_Falloff_Root: Self = Self((4) as i8);
    pub const eHook_Falloff_Linear: Self = Self((5) as i8);
    pub const eHook_Falloff_Const: Self = Self((6) as i8);
    pub const eHook_Falloff_Sphere: Self = Self((7) as i8);
    pub const eHook_Falloff_InvSquare: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BooleanModifierMaterialMode(pub i8);

impl BooleanModifierMaterialMode {
    pub const eBooleanModifierMaterialMode_Index: Self = Self((0) as i8);
    pub const eBooleanModifierMaterialMode_Transfer: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BooleanModifierOp(pub i8);

impl BooleanModifierOp {
    pub const eBooleanModifierOp_Intersect: Self = Self((0) as i8);
    pub const eBooleanModifierOp_Union: Self = Self((1) as i8);
    pub const eBooleanModifierOp_Difference: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BooleanModifierSolver(pub i8);

impl BooleanModifierSolver {
    pub const eBooleanModifierSolver_Float: Self = Self((0) as i8);
    pub const eBooleanModifierSolver_Mesh_Arr: Self = Self((1) as i8);
    pub const eBooleanModifierSolver_Manifold: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BooleanModifierFlag(pub i8);

impl BooleanModifierFlag {
    pub const eBooleanModifierFlag_Self: Self = Self(((1 << 0)) as i8);
    pub const eBooleanModifierFlag_Object: Self = Self(((1 << 1)) as i8);
    pub const eBooleanModifierFlag_Collection: Self = Self(((1 << 2)) as i8);
    pub const eBooleanModifierFlag_HoleTolerant: Self = Self(((1 << 3)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BooleanModifierBMeshFlag(pub i8);

impl BooleanModifierBMeshFlag {
    pub const eBooleanModifierBMeshFlag_BMesh_Separate: Self = Self(((1 << 0)) as i8);
    pub const eBooleanModifierBMeshFlag_BMesh_NoDissolve: Self = Self(((1 << 1)) as i8);
    pub const eBooleanModifierBMeshFlag_BMesh_NoConnectRegions: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshDeformModifierFlag(pub i16);

impl MeshDeformModifierFlag {
    pub const MOD_MDEF_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_MDEF_DYNAMIC_BIND: Self = Self(((1 << 1)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParticleSystemModifierFlag(pub i16);

impl ParticleSystemModifierFlag {
    pub const eParticleSystemFlag_Pars: Self = Self(((1 << 0)) as i16);
    pub const eParticleSystemFlag_psys_updated: Self = Self(((1 << 1)) as i16);
    pub const eParticleSystemFlag_file_loaded: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParticleInstanceModifierFlag(pub i16);

impl ParticleInstanceModifierFlag {
    pub const eParticleInstanceFlag_Parents: Self = Self(((1 << 0)) as i16);
    pub const eParticleInstanceFlag_Children: Self = Self(((1 << 1)) as i16);
    pub const eParticleInstanceFlag_Path: Self = Self(((1 << 2)) as i16);
    pub const eParticleInstanceFlag_Unborn: Self = Self(((1 << 3)) as i16);
    pub const eParticleInstanceFlag_Alive: Self = Self(((1 << 4)) as i16);
    pub const eParticleInstanceFlag_Dead: Self = Self(((1 << 5)) as i16);
    pub const eParticleInstanceFlag_KeepShape: Self = Self(((1 << 6)) as i16);
    pub const eParticleInstanceFlag_UseSize: Self = Self(((1 << 7)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ParticleInstanceModifierSpace(pub i16);

impl ParticleInstanceModifierSpace {
    pub const eParticleInstanceSpace_World: Self = Self((0) as i16);
    pub const eParticleInstanceSpace_Local: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ExplodeModifierFlag(pub i16);

impl ExplodeModifierFlag {
    pub const eExplodeFlag_CalcFaces: Self = Self(((1 << 0)) as i16);
    pub const eExplodeFlag_PaSize: Self = Self(((1 << 1)) as i16);
    pub const eExplodeFlag_EdgeCut: Self = Self(((1 << 2)) as i16);
    pub const eExplodeFlag_Unborn: Self = Self(((1 << 3)) as i16);
    pub const eExplodeFlag_Alive: Self = Self(((1 << 4)) as i16);
    pub const eExplodeFlag_Dead: Self = Self(((1 << 5)) as i16);
    pub const eExplodeFlag_INVERT_VGROUP: Self = Self(((1 << 6)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MultiresModifierFlag(pub i8);

impl MultiresModifierFlag {
    pub const eMultiresModifierFlag_ControlEdges: Self = Self(((1 << 0)) as i8);
    pub const eMultiresModifierFlag_PlainUv_DEPRECATED: Self = Self(((1 << 1)) as i8);
    pub const eMultiresModifierFlag_UseCrease: Self = Self(((1 << 2)) as i8);
    pub const eMultiresModifierFlag_UseCustomNormals: Self = Self(((1 << 3)) as i8);
    pub const eMultiresModifierFlag_UseSculptBaseMesh: Self = Self(((1 << 4)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SimpleDeformModifierFlag(pub i8);

impl SimpleDeformModifierFlag {
    pub const MOD_SIMPLEDEFORM_FLAG_INVERT_VGROUP: Self = Self(((1 << 0)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SimpleDeformModifierMode(pub i8);

impl SimpleDeformModifierMode {
    pub const MOD_SIMPLEDEFORM_MODE_TWIST: Self = Self((1) as i8);
    pub const MOD_SIMPLEDEFORM_MODE_BEND: Self = Self((2) as i8);
    pub const MOD_SIMPLEDEFORM_MODE_TAPER: Self = Self((3) as i8);
    pub const MOD_SIMPLEDEFORM_MODE_STRETCH: Self = Self((4) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SimpleDeformModifierLockAxis(pub i8);

impl SimpleDeformModifierLockAxis {
    pub const MOD_SIMPLEDEFORM_LOCK_AXIS_X: Self = Self(((1 << 0)) as i8);
    pub const MOD_SIMPLEDEFORM_LOCK_AXIS_Y: Self = Self(((1 << 1)) as i8);
    pub const MOD_SIMPLEDEFORM_LOCK_AXIS_Z: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SolidifyModifierFlag(pub i32);

impl SolidifyModifierFlag {
    pub const MOD_SOLIDIFY_RIM: Self = Self(((1 << 0)) as i32);
    pub const MOD_SOLIDIFY_EVEN: Self = Self(((1 << 1)) as i32);
    pub const MOD_SOLIDIFY_NORMAL_CALC: Self = Self(((1 << 2)) as i32);
    pub const MOD_SOLIDIFY_VGROUP_INV: Self = Self(((1 << 3)) as i32);
    pub const MOD_SOLIDIFY_RIM_MATERIAL: Self = Self(((1 << 4)) as i32);
    pub const MOD_SOLIDIFY_FLIP: Self = Self(((1 << 5)) as i32);
    pub const MOD_SOLIDIFY_NOSHELL: Self = Self(((1 << 6)) as i32);
    pub const MOD_SOLIDIFY_OFFSET_ANGLE_CLAMP: Self = Self(((1 << 7)) as i32);
    pub const MOD_SOLIDIFY_NONMANIFOLD_FLAT_FACES: Self = Self(((1 << 8)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SolidifyModifierMode(pub i8);

impl SolidifyModifierMode {
    pub const MOD_SOLIDIFY_MODE_EXTRUDE: Self = Self((0) as i8);
    pub const MOD_SOLIDIFY_MODE_NONMANIFOLD: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SolifyModifierNonManifoldOffsetMode(pub i8);

impl SolifyModifierNonManifoldOffsetMode {
    pub const MOD_SOLIDIFY_NONMANIFOLD_OFFSET_MODE_FIXED: Self = Self((0) as i8);
    pub const MOD_SOLIDIFY_NONMANIFOLD_OFFSET_MODE_EVEN: Self = Self((1) as i8);
    pub const MOD_SOLIDIFY_NONMANIFOLD_OFFSET_MODE_CONSTRAINTS: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SolidifyModifierNonManifoldBoundaryMode(pub i8);

impl SolidifyModifierNonManifoldBoundaryMode {
    pub const MOD_SOLIDIFY_NONMANIFOLD_BOUNDARY_MODE_NONE: Self = Self((0) as i8);
    pub const MOD_SOLIDIFY_NONMANIFOLD_BOUNDARY_MODE_ROUND: Self = Self((1) as i8);
    pub const MOD_SOLIDIFY_NONMANIFOLD_BOUNDARY_MODE_FLAT: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ScrewModifierFlag(pub i16);

impl ScrewModifierFlag {
    pub const MOD_SCREW_NORMAL_FLIP: Self = Self(((1 << 0)) as i16);
    pub const MOD_SCREW_NORMAL_CALC: Self = Self(((1 << 1)) as i16);
    pub const MOD_SCREW_OBJECT_OFFSET: Self = Self(((1 << 2)) as i16);
    pub const MOD_SCREW_SMOOTH_SHADING: Self = Self(((1 << 5)) as i16);
    pub const MOD_SCREW_UV_STRETCH_U: Self = Self(((1 << 6)) as i16);
    pub const MOD_SCREW_UV_STRETCH_V: Self = Self(((1 << 7)) as i16);
    pub const MOD_SCREW_MERGE: Self = Self(((1 << 8)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OceanModifierGeometryMode(pub i8);

impl OceanModifierGeometryMode {
    pub const MOD_OCEAN_GEOM_GENERATE: Self = Self((0) as i8);
    pub const MOD_OCEAN_GEOM_DISPLACE: Self = Self((1) as i8);
    pub const MOD_OCEAN_GEOM_SIM_ONLY: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OceanModifierSpectrum(pub i32);

impl OceanModifierSpectrum {
    pub const MOD_OCEAN_SPECTRUM_PHILLIPS: Self = Self((0) as i32);
    pub const MOD_OCEAN_SPECTRUM_PIERSON_MOSKOWITZ: Self = Self((1) as i32);
    pub const MOD_OCEAN_SPECTRUM_JONSWAP: Self = Self((2) as i32);
    pub const MOD_OCEAN_SPECTRUM_TEXEL_MARSEN_ARSLOE: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OceanModifierFlag(pub i8);

impl OceanModifierFlag {
    pub const MOD_OCEAN_GENERATE_FOAM: Self = Self(((1 << 0)) as i8);
    pub const MOD_OCEAN_GENERATE_NORMALS: Self = Self(((1 << 1)) as i8);
    pub const MOD_OCEAN_GENERATE_SPRAY: Self = Self(((1 << 2)) as i8);
    pub const MOD_OCEAN_INVERT_SPRAY: Self = Self(((1 << 3)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WarpModifierFlag(pub i8);

impl WarpModifierFlag {
    pub const MOD_WARP_VOLUME_PRESERVE: Self = Self(((1 << 0)) as i8);
    pub const MOD_WARP_INVERT_VGROUP: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WarpModifierFalloff(pub i8);

impl WarpModifierFalloff {
    pub const eWarp_Falloff_None: Self = Self((0) as i8);
    pub const eWarp_Falloff_Curve: Self = Self((1) as i8);
    pub const eWarp_Falloff_Sharp: Self = Self((2) as i8);
    pub const eWarp_Falloff_Smooth: Self = Self((3) as i8);
    pub const eWarp_Falloff_Root: Self = Self((4) as i8);
    pub const eWarp_Falloff_Linear: Self = Self((5) as i8);
    pub const eWarp_Falloff_Const: Self = Self((6) as i8);
    pub const eWarp_Falloff_Sphere: Self = Self((7) as i8);
    pub const eWarp_Falloff_InvSquare: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeigthVGEditModifierEditFlags(pub i16);

impl WeigthVGEditModifierEditFlags {
    pub const MOD_WVG_EDIT_WEIGHTS_NORMALIZE: Self = Self(((1 << 0)) as i16);
    pub const MOD_WVG_INVERT_FALLOFF: Self = Self(((1 << 1)) as i16);
    pub const MOD_WVG_EDIT_INVERT_VGROUP_MASK: Self = Self(((1 << 2)) as i16);
    pub const MOD_WVG_EDIT_ADD2VG: Self = Self(((1 << 3)) as i16);
    pub const MOD_WVG_EDIT_REMFVG: Self = Self(((1 << 4)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightVGMixModifierMixMode(pub i8);

impl WeightVGMixModifierMixMode {
    pub const MOD_WVG_MIX_SET: Self = Self((1) as i8);
    pub const MOD_WVG_MIX_ADD: Self = Self((2) as i8);
    pub const MOD_WVG_MIX_SUB: Self = Self((3) as i8);
    pub const MOD_WVG_MIX_MUL: Self = Self((4) as i8);
    pub const MOD_WVG_MIX_DIV: Self = Self((5) as i8);
    pub const MOD_WVG_MIX_DIF: Self = Self((6) as i8);
    pub const MOD_WVG_MIX_AVG: Self = Self((7) as i8);
    pub const MOD_WVG_MIX_MIN: Self = Self((8) as i8);
    pub const MOD_WVG_MIX_MAX: Self = Self((9) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightVGMixModifierMixSet(pub i8);

impl WeightVGMixModifierMixSet {
    pub const MOD_WVG_SET_ALL: Self = Self((1) as i8);
    pub const MOD_WVG_SET_A: Self = Self((2) as i8);
    pub const MOD_WVG_SET_B: Self = Self((3) as i8);
    pub const MOD_WVG_SET_OR: Self = Self((4) as i8);
    pub const MOD_WVG_SET_AND: Self = Self((5) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightVGMixModifierFlag(pub i8);

impl WeightVGMixModifierFlag {
    pub const MOD_WVG_MIX_INVERT_VGROUP_MASK: Self = Self(((1 << 0)) as i8);
    pub const MOD_WVG_MIX_WEIGHTS_NORMALIZE: Self = Self(((1 << 1)) as i8);
    pub const MOD_WVG_MIX_INVERT_VGROUP_A: Self = Self(((1 << 2)) as i8);
    pub const MOD_WVG_MIX_INVERT_VGROUP_B: Self = Self(((1 << 3)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilWeightProximityFlag(pub i32);

impl GreasePencilWeightProximityFlag {
    pub const MOD_GREASE_PENCIL_WEIGHT_PROXIMITY_INVERT_OUTPUT: Self = Self(((1 << 0)) as i32);
    pub const MOD_GREASE_PENCIL_WEIGHT_PROXIMITY_MULTIPLY_DATA: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightVGProximityModifierProximityMode(pub i32);

impl WeightVGProximityModifierProximityMode {
    pub const MOD_WVG_PROXIMITY_OBJECT: Self = Self((1) as i32);
    pub const MOD_WVG_PROXIMITY_GEOMETRY: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightVGProximityModifierFlag(pub i32);

impl WeightVGProximityModifierFlag {
    pub const MOD_WVG_PROXIMITY_GEOM_VERTS: Self = Self(((1 << 0)) as i32);
    pub const MOD_WVG_PROXIMITY_GEOM_EDGES: Self = Self(((1 << 1)) as i32);
    pub const MOD_WVG_PROXIMITY_GEOM_FACES: Self = Self(((1 << 2)) as i32);
    pub const MOD_WVG_PROXIMITY_INVERT_VGROUP_MASK: Self = Self(((1 << 3)) as i32);
    pub const MOD_WVG_PROXIMITY_INVERT_FALLOFF: Self = Self(((1 << 4)) as i32);
    pub const MOD_WVG_PROXIMITY_WEIGHTS_NORMALIZE: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightVGProximityFalloff(pub i16);

impl WeightVGProximityFalloff {
    pub const MOD_WVG_MAPPING_NONE: Self = Self((0) as i16);
    pub const MOD_WVG_MAPPING_CURVE: Self = Self((1) as i16);
    pub const MOD_WVG_MAPPING_SHARP: Self = Self((2) as i16);
    pub const MOD_WVG_MAPPING_SMOOTH: Self = Self((3) as i16);
    pub const MOD_WVG_MAPPING_ROOT: Self = Self((4) as i16);
    pub const MOD_WVG_MAPPING_SPHERE: Self = Self((7) as i16);
    pub const MOD_WVG_MAPPING_RANDOM: Self = Self((8) as i16);
    pub const MOD_WVG_MAPPING_STEP: Self = Self((9) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightVGProximityModifierMaskTexChannel(pub i32);

impl WeightVGProximityModifierMaskTexChannel {
    pub const MOD_WVG_MASK_TEX_USE_INT: Self = Self((1) as i32);
    pub const MOD_WVG_MASK_TEX_USE_RED: Self = Self((2) as i32);
    pub const MOD_WVG_MASK_TEX_USE_GREEN: Self = Self((3) as i32);
    pub const MOD_WVG_MASK_TEX_USE_BLUE: Self = Self((4) as i32);
    pub const MOD_WVG_MASK_TEX_USE_HUE: Self = Self((5) as i32);
    pub const MOD_WVG_MASK_TEX_USE_SAT: Self = Self((6) as i32);
    pub const MOD_WVG_MASK_TEX_USE_VAL: Self = Self((7) as i32);
    pub const MOD_WVG_MASK_TEX_USE_ALPHA: Self = Self((8) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DynamicPaintModifierType(pub i32);

impl DynamicPaintModifierType {
    pub const MOD_DYNAMICPAINT_TYPE_CANVAS: Self = Self(((1 << 0)) as i32);
    pub const MOD_DYNAMICPAINT_TYPE_BRUSH: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRemeshModifierFlags(pub i8);

impl eRemeshModifierFlags {
    pub const MOD_REMESH_FLOOD_FILL: Self = Self(((1 << 0)) as i8);
    pub const MOD_REMESH_SMOOTH_SHADING: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eRemeshModifierMode(pub i8);

impl eRemeshModifierMode {
    pub const MOD_REMESH_CENTROID: Self = Self((0) as i8);
    pub const MOD_REMESH_MASS_POINT: Self = Self((1) as i8);
    pub const MOD_REMESH_SHARP_FEATURES: Self = Self((2) as i8);
    pub const MOD_REMESH_VOXEL: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SkinModifierSymmetryAxis(pub i8);

impl SkinModifierSymmetryAxis {
    pub const MOD_SKIN_SYMM_X: Self = Self(((1 << 0)) as i8);
    pub const MOD_SKIN_SYMM_Y: Self = Self(((1 << 1)) as i8);
    pub const MOD_SKIN_SYMM_Z: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SkinModifierFlag(pub i8);

impl SkinModifierFlag {
    pub const MOD_SKIN_SMOOTH_SHADING: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TriangulateModifierFlag(pub i32);

impl TriangulateModifierFlag {
    pub const MOD_TRIANGULATE_BEAUTY: Self = Self(((1 << 0)) as i32);
    pub const MOD_TRIANGULATE_KEEP_CUSTOMLOOP_NORMALS: Self = Self((1 << 1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TriangulateModifierNgonMethod(pub i32);

impl TriangulateModifierNgonMethod {
    pub const MOD_TRIANGULATE_NGON_BEAUTY: Self = Self((0) as i32);
    pub const MOD_TRIANGULATE_NGON_EARCLIP: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TriangulateModifierQuadMethod(pub i32);

impl TriangulateModifierQuadMethod {
    pub const MOD_TRIANGULATE_QUAD_BEAUTY: Self = Self((0) as i32);
    pub const MOD_TRIANGULATE_QUAD_FIXED: Self = Self((1) as i32);
    pub const MOD_TRIANGULATE_QUAD_ALTERNATE: Self = Self((2) as i32);
    pub const MOD_TRIANGULATE_QUAD_SHORTEDGE: Self = Self((3) as i32);
    pub const MOD_TRIANGULATE_QUAD_LONGEDGE: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LaplacianSmoothModifierFlag(pub i16);

impl LaplacianSmoothModifierFlag {
    pub const MOD_LAPLACIANSMOOTH_X: Self = Self(((1 << 1)) as i16);
    pub const MOD_LAPLACIANSMOOTH_Y: Self = Self(((1 << 2)) as i16);
    pub const MOD_LAPLACIANSMOOTH_Z: Self = Self(((1 << 3)) as i16);
    pub const MOD_LAPLACIANSMOOTH_PRESERVE_VOLUME: Self = Self(((1 << 4)) as i16);
    pub const MOD_LAPLACIANSMOOTH_NORMALIZED: Self = Self(((1 << 5)) as i16);
    pub const MOD_LAPLACIANSMOOTH_INVERT_VGROUP: Self = Self(((1 << 6)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CorrectiveSmoothModifierType(pub i8);

impl CorrectiveSmoothModifierType {
    pub const MOD_CORRECTIVESMOOTH_SMOOTH_SIMPLE: Self = Self((0) as i8);
    pub const MOD_CORRECTIVESMOOTH_SMOOTH_LENGTH_WEIGHT: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CorrectiveSmoothRestSource(pub i8);

impl CorrectiveSmoothRestSource {
    pub const MOD_CORRECTIVESMOOTH_RESTSOURCE_ORCO: Self = Self((0) as i8);
    pub const MOD_CORRECTIVESMOOTH_RESTSOURCE_BIND: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CorrectiveSmoothModifierFlag(pub i16);

impl CorrectiveSmoothModifierFlag {
    pub const MOD_CORRECTIVESMOOTH_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_CORRECTIVESMOOTH_ONLY_SMOOTH: Self = Self(((1 << 1)) as i16);
    pub const MOD_CORRECTIVESMOOTH_PIN_BOUNDARY: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct UVWarpModifierFlag(pub i16);

impl UVWarpModifierFlag {
    pub const MOD_UVWARP_INVERT_VGROUP: Self = Self((1 << 0) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshCacheModifierFlag(pub i8);

impl MeshCacheModifierFlag {
    pub const MOD_MESHCACHE_INVERT_VERTEX_GROUP: Self = Self((1 << 0) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshCacheModifierType(pub i8);

impl MeshCacheModifierType {
    pub const MOD_MESHCACHE_TYPE_MDD: Self = Self((1) as i8);
    pub const MOD_MESHCACHE_TYPE_PC2: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshCacheModifierDeformMode(pub i8);

impl MeshCacheModifierDeformMode {
    pub const MOD_MESHCACHE_DEFORM_OVERWRITE: Self = Self((0) as i8);
    pub const MOD_MESHCACHE_DEFORM_INTEGRATE: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshCacheModifierInterpolation(pub i8);

impl MeshCacheModifierInterpolation {
    pub const MOD_MESHCACHE_INTERP_NONE: Self = Self((0) as i8);
    pub const MOD_MESHCACHE_INTERP_LINEAR: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshCacheModifierTimeMode(pub i8);

impl MeshCacheModifierTimeMode {
    pub const MOD_MESHCACHE_TIME_FRAME: Self = Self((0) as i8);
    pub const MOD_MESHCACHE_TIME_SECONDS: Self = Self((1) as i8);
    pub const MOD_MESHCACHE_TIME_FACTOR: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshCacheModifierPlayMode(pub i8);

impl MeshCacheModifierPlayMode {
    pub const MOD_MESHCACHE_PLAY_CFEA: Self = Self((0) as i8);
    pub const MOD_MESHCACHE_PLAY_EVAL: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshCacheModifierFlipAxis(pub i8);

impl MeshCacheModifierFlipAxis {
    pub const MOD_MESHCACHE_FLIP_AXIS_X: Self = Self((1 << 0) as i8);
    pub const MOD_MESHCACHE_FLIP_AXIS_Y: Self = Self((1 << 1) as i8);
    pub const MOD_MESHCACHE_FLIP_AXIS_Z: Self = Self((1 << 2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LaplacianDeformModifierFlag(pub i16);

impl LaplacianDeformModifierFlag {
    pub const MOD_LAPLACIANDEFORM_BIND: Self = Self((1 << 0) as i16);
    pub const MOD_LAPLACIANDEFORM_INVERT_VGROUP: Self = Self((1 << 1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WireframeModifierFlag(pub i16);

impl WireframeModifierFlag {
    pub const MOD_WIREFRAME_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_WIREFRAME_REPLACE: Self = Self(((1 << 1)) as i16);
    pub const MOD_WIREFRAME_BOUNDARY: Self = Self(((1 << 2)) as i16);
    pub const MOD_WIREFRAME_OFS_EVEN: Self = Self(((1 << 3)) as i16);
    pub const MOD_WIREFRAME_OFS_RELATIVE: Self = Self(((1 << 4)) as i16);
    pub const MOD_WIREFRAME_CREASE: Self = Self(((1 << 5)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeldModifierFlag(pub i8);

impl WeldModifierFlag {
    pub const MOD_WELD_INVERT_VGROUP: Self = Self(((1 << 0)) as i8);
    pub const MOD_WELD_LOOSE_EDGES: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeldModifierMode(pub i8);

impl WeldModifierMode {
    pub const MOD_WELD_MODE_ALL: Self = Self((0) as i8);
    pub const MOD_WELD_MODE_CONNECTED: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DataTransferModifierFlag(pub u32);

impl DataTransferModifierFlag {
    pub const MOD_DATATRANSFER_OBSRC_TRANSFORM: Self = Self((1 << 0) as u32);
    pub const MOD_DATATRANSFER_MAP_MAXDIST: Self = Self((1 << 1) as u32);
    pub const MOD_DATATRANSFER_INVERT_VGROUP: Self = Self((1 << 2) as u32);
    pub const MOD_DATATRANSFER_USE_VERT: Self = Self((1 << 28) as u32);
    pub const MOD_DATATRANSFER_USE_EDGE: Self = Self((1 << 29) as u32);
    pub const MOD_DATATRANSFER_USE_LOOP: Self = Self((1 << 30) as u32);
    pub const MOD_DATATRANSFER_USE_POLY: Self = Self(6 as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NormalEditModifierMode(pub i16);

impl NormalEditModifierMode {
    pub const MOD_NORMALEDIT_MODE_RADIAL: Self = Self((0) as i16);
    pub const MOD_NORMALEDIT_MODE_DIRECTIONAL: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NormalEditModifierFlag(pub i16);

impl NormalEditModifierFlag {
    pub const MOD_NORMALEDIT_INVERT_VGROUP: Self = Self(((1 << 0)) as i16);
    pub const MOD_NORMALEDIT_USE_DIRECTION_PARALLEL: Self = Self(((1 << 1)) as i16);
    pub const MOD_NORMALEDIT_NO_POLYNORS_FIX: Self = Self(((1 << 2)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NormalEditModifierMixMode(pub i16);

impl NormalEditModifierMixMode {
    pub const MOD_NORMALEDIT_MIX_COPY: Self = Self((0) as i16);
    pub const MOD_NORMALEDIT_MIX_ADD: Self = Self((1) as i16);
    pub const MOD_NORMALEDIT_MIX_SUB: Self = Self((2) as i16);
    pub const MOD_NORMALEDIT_MIX_MUL: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshSeqCacheModifierReadFlag(pub i8);

impl MeshSeqCacheModifierReadFlag {
    pub const MOD_MESHSEQ_READ_VERT: Self = Self(((1 << 0)) as i8);
    pub const MOD_MESHSEQ_READ_POLY: Self = Self(((1 << 1)) as i8);
    pub const MOD_MESHSEQ_READ_UV: Self = Self(((1 << 2)) as i8);
    pub const MOD_MESHSEQ_READ_COLOR: Self = Self(((1 << 3)) as i8);
    pub const MOD_MESHSEQ_INTERPOLATE_VERTICES: Self = Self(((1 << 4)) as i8);
    pub const MOD_MESHSEQ_READ_ATTRIBUTES: Self = Self(((1 << 5)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SurfaceDeformModifierFlag(pub i32);

impl SurfaceDeformModifierFlag {
    pub const MOD_SDEF_BIND: Self = Self(((1 << 0)) as i32);
    pub const MOD_SDEF_INVERT_VGROUP: Self = Self(((1 << 1)) as i32);
    pub const MOD_SDEF_SPARSE_BIND: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SurfaceDeformModifierBindMode(pub i32);

impl SurfaceDeformModifierBindMode {
    pub const MOD_SDEF_MODE_CORNER_TRIS: Self = Self((0) as i32);
    pub const MOD_SDEF_MODE_NGONS: Self = Self((1) as i32);
    pub const MOD_SDEF_MODE_CENTROID: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightedNormalModifierMode(pub i8);

impl WeightedNormalModifierMode {
    pub const MOD_WEIGHTEDNORMAL_MODE_FACE: Self = Self((0) as i8);
    pub const MOD_WEIGHTEDNORMAL_MODE_ANGLE: Self = Self((1) as i8);
    pub const MOD_WEIGHTEDNORMAL_MODE_FACE_ANGLE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightedNormalModifierFlag(pub i8);

impl WeightedNormalModifierFlag {
    pub const MOD_WEIGHTEDNORMAL_KEEP_SHARP: Self = Self(((1 << 0)) as i8);
    pub const MOD_WEIGHTEDNORMAL_INVERT_VGROUP: Self = Self(((1 << 1)) as i8);
    pub const MOD_WEIGHTEDNORMAL_FACE_INFLUENCE: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodesModifierPanelFlag(pub u32);

impl NodesModifierPanelFlag {
    pub const NODES_MODIFIER_PANEL_OPEN: Self = Self((1 << 0) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodesModifierBakeFlag(pub u32);

impl NodesModifierBakeFlag {
    pub const NODES_MODIFIER_BAKE_CUSTOM_SIMULATION_FRAME_RANGE: Self = Self((1 << 0) as u32);
    pub const NODES_MODIFIER_BAKE_CUSTOM_PATH: Self = Self((1 << 1) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodesModifierBakeTarget(pub i8);

impl NodesModifierBakeTarget {
    pub const NODES_MODIFIER_BAKE_TARGET_INHERIT: Self = Self((0) as i8);
    pub const NODES_MODIFIER_BAKE_TARGET_PACKED: Self = Self((1) as i8);
    pub const NODES_MODIFIER_BAKE_TARGET_DISK: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodesModifierBakeMode(pub u8);

impl NodesModifierBakeMode {
    pub const NODES_MODIFIER_BAKE_MODE_ANIMATION: Self = Self((0) as u8);
    pub const NODES_MODIFIER_BAKE_MODE_STILL: Self = Self((1) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometryNodesModifierPanel(pub i32);

impl GeometryNodesModifierPanel {
    pub const NODES_MODIFIER_PANEL_OUTPUT_ATTRIBUTES: Self = Self((0) as i32);
    pub const NODES_MODIFIER_PANEL_MANAGE: Self = Self((1) as i32);
    pub const NODES_MODIFIER_PANEL_BAKE: Self = Self((2) as i32);
    pub const NODES_MODIFIER_PANEL_NAMED_ATTRIBUTES: Self = Self((3) as i32);
    pub const NODES_MODIFIER_PANEL_BAKE_DATA_BLOCKS: Self = Self((4) as i32);
    pub const NODES_MODIFIER_PANEL_WARNINGS: Self = Self((5) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NodesModifierFlag(pub i8);

impl NodesModifierFlag {
    pub const NODES_MODIFIER_HIDE_DATABLOCK_SELECTOR: Self = Self(((1 << 0)) as i8);
    pub const NODES_MODIFIER_HIDE_MANAGE_PANEL: Self = Self(((1 << 1)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshToVolumeModifierResolutionMode(pub i32);

impl MeshToVolumeModifierResolutionMode {
    pub const MESH_TO_VOLUME_RESOLUTION_MODE_VOXEL_AMOUNT: Self = Self((0) as i32);
    pub const MESH_TO_VOLUME_RESOLUTION_MODE_VOXEL_SIZE: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VolumeDisplaceModifierTextureMapMode(pub i32);

impl VolumeDisplaceModifierTextureMapMode {
    pub const MOD_VOLUME_DISPLACE_MAP_LOCAL: Self = Self((0) as i32);
    pub const MOD_VOLUME_DISPLACE_MAP_GLOBAL: Self = Self((1) as i32);
    pub const MOD_VOLUME_DISPLACE_MAP_OBJECT: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VolumeToMeshResolutionMode(pub i32);

impl VolumeToMeshResolutionMode {
    pub const VOLUME_TO_MESH_RESOLUTION_MODE_GRID: Self = Self((0) as i32);
    pub const VOLUME_TO_MESH_RESOLUTION_MODE_VOXEL_AMOUNT: Self = Self((1) as i32);
    pub const VOLUME_TO_MESH_RESOLUTION_MODE_VOXEL_SIZE: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VolumeToMeshFlag(pub u32);

impl VolumeToMeshFlag {
    pub const VOLUME_TO_MESH_USE_SMOOTH_SHADE: Self = Self((1 << 0) as u32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilModifierInfluenceFlag(pub i32);

impl GreasePencilModifierInfluenceFlag {
    pub const GREASE_PENCIL_INFLUENCE_INVERT_LAYER_FILTER: Self = Self(((1 << 0)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_USE_LAYER_PASS_FILTER: Self = Self(((1 << 1)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_INVERT_LAYER_PASS_FILTER: Self = Self(((1 << 2)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_INVERT_MATERIAL_FILTER: Self = Self(((1 << 3)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_USE_MATERIAL_PASS_FILTER: Self = Self(((1 << 4)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_INVERT_MATERIAL_PASS_FILTER: Self = Self(((1 << 5)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_INVERT_VERTEX_GROUP: Self = Self(((1 << 6)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_USE_CUSTOM_CURVE: Self = Self(((1 << 7)) as i32);
    pub const GREASE_PENCIL_INFLUENCE_USE_LAYER_GROUP_FILTER: Self = Self(((1 << 8)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilModifierColorMode(pub i8);

impl GreasePencilModifierColorMode {
    pub const MOD_GREASE_PENCIL_COLOR_STROKE: Self = Self((0) as i8);
    pub const MOD_GREASE_PENCIL_COLOR_FILL: Self = Self((1) as i8);
    pub const MOD_GREASE_PENCIL_COLOR_BOTH: Self = Self((2) as i8);
    pub const MOD_GREASE_PENCIL_COLOR_HARDNESS: Self = Self((3) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilOpacityModifierFlag(pub i32);

impl GreasePencilOpacityModifierFlag {
    pub const MOD_GREASE_PENCIL_OPACITY_USE_WEIGHT_AS_FACTOR: Self = Self(((1 << 0)) as i32);
    pub const MOD_GREASE_PENCIL_OPACITY_USE_UNIFORM_OPACITY: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilSubdivideType(pub i32);

impl GreasePencilSubdivideType {
    pub const MOD_GREASE_PENCIL_SUBDIV_CATMULL: Self = Self((0) as i32);
    pub const MOD_GREASE_PENCIL_SUBDIV_SIMPLE: Self = Self((1) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilTintModifierMode(pub i8);

impl GreasePencilTintModifierMode {
    pub const MOD_GREASE_PENCIL_TINT_UNIFORM: Self = Self((0) as i8);
    pub const MOD_GREASE_PENCIL_TINT_GRADIENT: Self = Self((1) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilTintModifierFlag(pub i16);

impl GreasePencilTintModifierFlag {
    pub const MOD_GREASE_PENCIL_TINT_USE_WEIGHT_AS_FACTOR: Self = Self(((1 << 0)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGreasePencilSmooth_Flag(pub i32);

impl eGreasePencilSmooth_Flag {
    pub const MOD_GREASE_PENCIL_SMOOTH_MOD_LOCATION: Self = Self(((1 << 0)) as i32);
    pub const MOD_GREASE_PENCIL_SMOOTH_MOD_STRENGTH: Self = Self(((1 << 1)) as i32);
    pub const MOD_GREASE_PENCIL_SMOOTH_MOD_THICKNESS: Self = Self(((1 << 2)) as i32);
    pub const MOD_GREASE_PENCIL_SMOOTH_MOD_UV: Self = Self(((1 << 3)) as i32);
    pub const MOD_GREASE_PENCIL_SMOOTH_KEEP_SHAPE: Self = Self(((1 << 4)) as i32);
    pub const MOD_GREASE_PENCIL_SMOOTH_SMOOTH_ENDS: Self = Self(((1 << 5)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilOffsetModifierFlag(pub i32);

impl GreasePencilOffsetModifierFlag {
    pub const MOD_GREASE_PENCIL_OFFSET_UNIFORM_RANDOM_SCALE: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilOffsetModifierMode(pub i32);

impl GreasePencilOffsetModifierMode {
    pub const MOD_GREASE_PENCIL_OFFSET_RANDOM: Self = Self((0) as i32);
    pub const MOD_GREASE_PENCIL_OFFSET_LAYER: Self = Self((1) as i32);
    pub const MOD_GREASE_PENCIL_OFFSET_MATERIAL: Self = Self((2) as i32);
    pub const MOD_GREASE_PENCIL_OFFSET_STROKE: Self = Self((3) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilMirrorModifierFlag(pub i32);

impl GreasePencilMirrorModifierFlag {
    pub const MOD_GREASE_PENCIL_MIRROR_AXIS_X: Self = Self(((1 << 0)) as i32);
    pub const MOD_GREASE_PENCIL_MIRROR_AXIS_Y: Self = Self(((1 << 1)) as i32);
    pub const MOD_GREASE_PENCIL_MIRROR_AXIS_Z: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilThicknessModifierFlag(pub i32);

impl GreasePencilThicknessModifierFlag {
    pub const MOD_GREASE_PENCIL_THICK_NORMALIZE: Self = Self(((1 << 0)) as i32);
    pub const MOD_GREASE_PENCIL_THICK_WEIGHT_FACTOR: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilDashModifierFlag(pub i32);

impl GreasePencilDashModifierFlag {
    pub const MOD_GREASE_PENCIL_DASH_USE_CYCLIC: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilMultiplyModifierFlag(pub i32);

impl GreasePencilMultiplyModifierFlag {
    pub const MOD_GREASE_PENCIL_MULTIPLY_ENABLE_FADING: Self = Self(((1 << 2)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilWeightAngleModifierFlag(pub i32);

impl GreasePencilWeightAngleModifierFlag {
    pub const MOD_GREASE_PENCIL_WEIGHT_ANGLE_MULTIPLY_DATA: Self = Self(((1 << 5)) as i32);
    pub const MOD_GREASE_PENCIL_WEIGHT_ANGLE_INVERT_OUTPUT: Self = Self(((1 << 6)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilWeightAngleModifierSpace(pub i16);

impl GreasePencilWeightAngleModifierSpace {
    pub const MOD_GREASE_PENCIL_WEIGHT_ANGLE_SPACE_LOCAL: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_WEIGHT_ANGLE_SPACE_WORLD: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilArrayModifierFlag(pub i32);

impl GreasePencilArrayModifierFlag {
    pub const MOD_GREASE_PENCIL_ARRAY_USE_OFFSET: Self = Self(((1 << 7)) as i32);
    pub const MOD_GREASE_PENCIL_ARRAY_USE_RELATIVE: Self = Self(((1 << 8)) as i32);
    pub const MOD_GREASE_PENCIL_ARRAY_USE_OB_OFFSET: Self = Self(((1 << 9)) as i32);
    pub const MOD_GREASE_PENCIL_ARRAY_UNIFORM_RANDOM_SCALE: Self = Self(((1 << 10)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilHookFlag(pub i32);

impl GreasePencilHookFlag {
    pub const MOD_GREASE_PENCIL_HOOK_UNIFORM_SPACE: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilHookFalloff(pub i8);

impl GreasePencilHookFalloff {
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_None: Self = Self((0) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_Curve: Self = Self((1) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_Sharp: Self = Self((2) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_Smooth: Self = Self((3) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_Root: Self = Self((4) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_Linear: Self = Self((5) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_Const: Self = Self((6) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_Sphere: Self = Self((7) as i8);
    pub const MOD_GREASE_PENCIL_HOOK_Falloff_InvSquare: Self = Self((8) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGreasePencilLineartFlags(pub i32);

impl eGreasePencilLineartFlags {
    pub const LINEART_GPENCIL_BINARY_WEIGHTS: Self = Self(((1                                     << 2)) as i32);
    pub const LINEART_GPENCIL_IS_BAKED: Self = Self(((1 << 3)) as i32);
    pub const LINEART_GPENCIL_USE_CACHE: Self = Self(((1 << 4)) as i32);
    pub const LINEART_GPENCIL_OFFSET_TOWARDS_CUSTOM_CAMERA: Self = Self(((1 << 5)) as i32);
    pub const LINEART_GPENCIL_INVERT_COLLECTION: Self = Self(((1 << 6)) as i32);
    pub const LINEART_GPENCIL_INVERT_SILHOUETTE_FILTER: Self = Self(((1 << 7)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLineartModifierSource(pub i8);

impl GreasePencilLineartModifierSource {
    pub const LINEART_SOURCE_COLLECTION: Self = Self((0) as i8);
    pub const LINEART_SOURCE_OBJECT: Self = Self((1) as i8);
    pub const LINEART_SOURCE_SCENE: Self = Self((2) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLineartModifierShadowFilter(pub u8);

impl GreasePencilLineartModifierShadowFilter {
    pub const LINEART_SHADOW_FILTER_NONE: Self = Self((0) as u8);
    pub const LINEART_SHADOW_FILTER_ILLUMINATED: Self = Self((1) as u8);
    pub const LINEART_SHADOW_FILTER_SHADED: Self = Self((2) as u8);
    pub const LINEART_SHADOW_FILTER_ILLUMINATED_ENCLOSED_SHAPES: Self = Self((3) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilLineartMaskSwitches(pub u8);

impl GreasePencilLineartMaskSwitches {
    pub const MOD_LINEART_MATERIAL_MASK_ENABLE: Self = Self(((1 << 0)) as u8);
    pub const MOD_LINEART_MATERIAL_MASK_MATCH: Self = Self(((1 << 1)) as u8);
    pub const MOD_LINEART_INTERSECTION_MATCH: Self = Self(((1 << 2)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGreasePencilLineartMaskSwitches(pub i8);

impl eGreasePencilLineartMaskSwitches {
    pub const LINEART_GPENCIL_MATERIAL_MASK_ENABLE: Self = Self(((1 << 0)) as i8);
    pub const LINEART_GPENCIL_MATERIAL_MASK_MATCH: Self = Self(((1 << 1)) as i8);
    pub const LINEART_GPENCIL_INTERSECTION_MATCH: Self = Self(((1 << 2)) as i8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct eGreasePencilLineartSilhouetteFilter(pub u8);

impl eGreasePencilLineartSilhouetteFilter {
    pub const LINEART_SILHOUETTE_FILTER_NONE: Self = Self((0) as u8);
    pub const LINEART_SILHOUETTE_FILTER_GROUP: Self = Self(((1 << 0)) as u8);
    pub const LINEART_SILHOUETTE_FILTER_INDIVIDUAL: Self = Self(((1 << 1)) as u8);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilTimeModifierFlag(pub i32);

impl GreasePencilTimeModifierFlag {
    pub const MOD_GREASE_PENCIL_TIME_KEEP_LOOP: Self = Self(((1 << 0)) as i32);
    pub const MOD_GREASE_PENCIL_TIME_CUSTOM_RANGE: Self = Self(((1 << 1)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilTimeModifierMode(pub i32);

impl GreasePencilTimeModifierMode {
    pub const MOD_GREASE_PENCIL_TIME_MODE_NORMAL: Self = Self((0) as i32);
    pub const MOD_GREASE_PENCIL_TIME_MODE_REVERSE: Self = Self((1) as i32);
    pub const MOD_GREASE_PENCIL_TIME_MODE_FIX: Self = Self((2) as i32);
    pub const MOD_GREASE_PENCIL_TIME_MODE_PINGPONG: Self = Self((3) as i32);
    pub const MOD_GREASE_PENCIL_TIME_MODE_CHAIN: Self = Self((4) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilTimeModifierSegmentMode(pub i32);

impl GreasePencilTimeModifierSegmentMode {
    pub const MOD_GREASE_PENCIL_TIME_SEG_MODE_NORMAL: Self = Self((0) as i32);
    pub const MOD_GREASE_PENCIL_TIME_SEG_MODE_REVERSE: Self = Self((1) as i32);
    pub const MOD_GREASE_PENCIL_TIME_SEG_MODE_PINGPONG: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilEnvelopeModifierMode(pub i32);

impl GreasePencilEnvelopeModifierMode {
    pub const MOD_GREASE_PENCIL_ENVELOPE_DEFORM: Self = Self((0) as i32);
    pub const MOD_GREASE_PENCIL_ENVELOPE_SEGMENTS: Self = Self((1) as i32);
    pub const MOD_GREASE_PENCIL_ENVELOPE_FILLS: Self = Self((2) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilOutlineModifierFlag(pub i32);

impl GreasePencilOutlineModifierFlag {
    pub const MOD_GREASE_PENCIL_OUTLINE_KEEP_SHAPE: Self = Self(((1 << 0)) as i32);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilBuildMode(pub i16);

impl GreasePencilBuildMode {
    pub const MOD_GREASE_PENCIL_BUILD_MODE_SEQUENTIAL: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_MODE_CONCURRENT: Self = Self((1) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_MODE_ADDITIVE: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilBuildTransition(pub i16);

impl GreasePencilBuildTransition {
    pub const MOD_GREASE_PENCIL_BUILD_TRANSITION_GROW: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_TRANSITION_SHRINK: Self = Self((1) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_TRANSITION_VANISH: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilBuildTimeAlignment(pub i16);

impl GreasePencilBuildTimeAlignment {
    pub const MOD_GREASE_PENCIL_BUILD_TIMEALIGN_START: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_TIMEALIGN_END: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilBuildTimeMode(pub i16);

impl GreasePencilBuildTimeMode {
    pub const MOD_GREASE_PENCIL_BUILD_TIMEMODE_FRAMES: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_TIMEMODE_PERCENTAGE: Self = Self((1) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_TIMEMODE_DRAWSPEED: Self = Self((2) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilBuildFlag(pub i16);

impl GreasePencilBuildFlag {
    pub const MOD_GREASE_PENCIL_BUILD_RESTRICT_TIME: Self = Self(((1 << 0)) as i16);
    pub const MOD_GREASE_PENCIL_BUILD_USE_FADING: Self = Self(((1 << 14)) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilSimplifyModifierMode(pub i16);

impl GreasePencilSimplifyModifierMode {
    pub const MOD_GREASE_PENCIL_SIMPLIFY_FIXED: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_SIMPLIFY_ADAPTIVE: Self = Self((1) as i16);
    pub const MOD_GREASE_PENCIL_SIMPLIFY_SAMPLE: Self = Self((2) as i16);
    pub const MOD_GREASE_PENCIL_SIMPLIFY_MERGE: Self = Self((3) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilTextureModifierFit(pub i16);

impl GreasePencilTextureModifierFit {
    pub const MOD_GREASE_PENCIL_TEXTURE_FIT_STROKE: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_TEXTURE_CONSTANT_LENGTH: Self = Self((1) as i16);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GreasePencilTextureModifierMode(pub i16);

impl GreasePencilTextureModifierMode {
    pub const MOD_GREASE_PENCIL_TEXTURE_STROKE: Self = Self((0) as i16);
    pub const MOD_GREASE_PENCIL_TEXTURE_FILL: Self = Self((1) as i16);
    pub const MOD_GREASE_PENCIL_TEXTURE_STROKE_AND_FILL: Self = Self((2) as i16);
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ModifierData {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
    pub r#type: ModifierType,
    pub mode: ModifierMode,
}

impl Default for ModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MappingInfoModifierData {
    pub modifier: ModifierData,
    pub texture: *mut core::ffi::c_void,
    pub map_object: *mut core::ffi::c_void,
    pub map_bone: [u8; 64],
    pub uvlayer_name: [u8; 68],
    pub _pad1: [u8; 4],
}

impl Default for MappingInfoModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SubsurfModifierData {
    pub modifier: ModifierData,
    pub subdivType: i16,
    pub levels: i16,
    pub renderLevels: i16,
    pub flags: SubsurfModifierFlag,
    pub uv_smooth: eSubsurfUVSmooth,
    pub quality: i16,
    pub boundary_smooth: eSubsurfBoundarySmooth,
    pub adaptive_space: eSubsurfAdaptiveSpace,
    pub adaptive_pixel_size: f32,
    pub adaptive_object_edge_length: f32,
}

impl Default for SubsurfModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LatticeModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub strength: f32,
    pub flag: LatticeModifierFlag,
}

impl Default for LatticeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CurveModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub name: [u8; 64],
    pub defaxis: CurveModifierDefaultAxis,
    pub flag: CurveModifierFlag,
}

impl Default for CurveModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BuildModifierData {
    pub modifier: ModifierData,
    pub start: f32,
    pub length: f32,
    pub flag: BuildModifierFlag,
}

impl Default for BuildModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MaskModifierData {
    pub modifier: ModifierData,
    pub ob_arm: *mut core::ffi::c_void,
    pub vgroup: [u8; 64],
    pub mode: MaskModifierMode,
    pub flag: MaskModifierFlag,
}

impl Default for MaskModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ArrayModifierData {
    pub modifier: ModifierData,
    pub start_cap: *mut core::ffi::c_void,
    pub end_cap: *mut core::ffi::c_void,
    pub curve_ob: *mut core::ffi::c_void,
    pub offset_ob: *mut core::ffi::c_void,
    pub offset: [f32; 3],
    pub _0: f32,
    pub _0_1: f32,
}

impl Default for ArrayModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MirrorModifierData {
    pub flag: MirrorModifierFlag,
    pub tolerance: f32,
    pub bisect_threshold: f32,
    pub use_correct_order_on_merge: u8,
    pub _pad: [u8; 3],
}

impl Default for MirrorModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct EdgeSplitModifierData {
    pub modifier: ModifierData,
    pub split_angle: f32,
    pub flags: EdgeSplitModifierFlag,
}

impl Default for EdgeSplitModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BevelModifierData {
    pub modifier: ModifierData,
    pub value: f32,
    pub res: i32,
    pub flags: BevelModifierFlag,
}

impl Default for BevelModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FluidModifierData {
    pub modifier: ModifierData,
    pub domain: *mut core::ffi::c_void,
    pub flow: *mut core::ffi::c_void,
    pub effector: *mut core::ffi::c_void,
    pub time: f32,
    pub r#type: FluidModifierType,
}

impl Default for FluidModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DisplaceModifierData {
    pub modifier: ModifierData,
    pub texture: *mut core::ffi::c_void,
    pub map_object: *mut core::ffi::c_void,
    pub map_bone: [u8; 64],
    pub uvlayer_name: [u8; 68],
    pub _pad1: [u8; 4],
}

impl Default for DisplaceModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UVProjectModifierData {
    pub modifier: ModifierData,
    pub projectors: [Object; 10],
}

impl Default for UVProjectModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DecimateModifierData {
    pub modifier: ModifierData,
    pub percent: f32,
    pub iter: i16,
    pub delimit: i8,
    pub symmetry_axis: i8,
    pub angle: f32,
    pub defgrp_name: [u8; 64],
    pub defgrp_factor: f32,
    pub flag: DecimateModifierFlag,
}

impl Default for DecimateModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SmoothModifierData {
    pub modifier: ModifierData,
    pub fac: f32,
    pub defgrp_name: [u8; 64],
    pub flag: SmoothModifierFlag,
    pub repeat: i16,
}

impl Default for SmoothModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CastModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub fac: f32,
    pub radius: f32,
    pub size: f32,
    pub defgrp_name: [u8; 64],
    pub flag: CastModifierFlag,
    pub r#type: CastModifierType,
    pub _pad1: *mut core::ffi::c_void,
}

impl Default for CastModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WaveModifierData {
    pub modifier: ModifierData,
    pub texture: *mut core::ffi::c_void,
    pub map_object: *mut core::ffi::c_void,
    pub map_bone: [u8; 64],
    pub uvlayer_name: [u8; 68],
    pub _pad1: [u8; 4],
}

impl Default for WaveModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ArmatureModifierData {
    pub modifier: ModifierData,
    pub deformflag: i16,
    pub multi: i16,
    pub _pad2: [u8; 4],
}

impl Default for ArmatureModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct HookModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub subtarget: [u8; 64],
    pub flag: HookModifierFlag,
}

impl Default for HookModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SoftbodyModifierData {
    pub modifier: ModifierData,
}

impl Default for SoftbodyModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ClothModifierData {
    pub modifier: ModifierData,
    pub clothObject: *mut core::ffi::c_void,
    pub sim_parms: *mut core::ffi::c_void,
    pub coll_parms: *mut core::ffi::c_void,
    pub point_cache: *mut core::ffi::c_void,
    pub ptcaches: ListBaseT<PointCache>,
    pub nullptr: ListBaseT<PointCache>,
}

impl Default for ClothModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CollisionModifierData {
    pub modifier: ModifierData,
    pub mvert_num: u32,
    pub tri_num: u32,
    pub time_x: f32,
    pub time_xnew: f32,
    pub is_static: i8,
    pub _pad: [u8; 7],
}

impl Default for CollisionModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SurfaceModifierData_Runtime {

}

impl Default for SurfaceModifierData_Runtime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SurfaceModifierData {
    pub modifier: ModifierData,
    pub runtime: SurfaceModifierData_Runtime,
}

impl Default for SurfaceModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct BooleanModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub collection: *mut core::ffi::c_void,
    pub double_threshold: f32,
    pub operation: BooleanModifierOp,
    pub solver: BooleanModifierSolver,
    pub material_mode: BooleanModifierMaterialMode,
    pub flag: BooleanModifierFlag,
    pub bm_flag: BooleanModifierBMeshFlag,
}

impl Default for BooleanModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MDefInfluence {
    pub vertex: i32,
    pub weight: f32,
}

impl Default for MDefInfluence {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MDefCell {
    pub offset: i32,
    pub influences_num: i32,
}

impl Default for MDefCell {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MeshDeformModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub defgrp_name: [u8; 64],
    pub gridsize: i16,
    pub flag: MeshDeformModifierFlag,
}

impl Default for MeshDeformModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleSystemModifierData {
    pub modifier: ModifierData,
    pub psys: *mut core::ffi::c_void,
    pub mesh_final: *mut core::ffi::c_void,
    pub mesh_original: *mut core::ffi::c_void,
    pub totdmvert: i32,
    pub totdmedge: i32,
    pub totdmface: i32,
    pub flag: ParticleSystemModifierFlag,
}

impl Default for ParticleSystemModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ParticleInstanceModifierData {
    pub modifier: ModifierData,
    pub ob: *mut core::ffi::c_void,
    pub psys: i16,
    pub flag: ParticleInstanceModifierFlag,
    pub axis: i16,
    pub space: ParticleInstanceModifierSpace,
    pub position: f32,
    pub random_position: f32,
    pub rotation: f32,
    pub random_rotation: f32,
    pub particle_amount: f32,
    pub particle_offset: f32,
    pub index_layer_name: [u8; 68],
    pub value_layer_name: [u8; 68],
    pub _pad1: *mut core::ffi::c_void,
}

impl Default for ParticleInstanceModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ExplodeModifierData {
    pub modifier: ModifierData,
    pub facepa: *mut core::ffi::c_void,
    pub flag: ExplodeModifierFlag,
    pub vgroup: i16,
    pub protect: f32,
    pub uvname: [u8; 68],
    pub _pad1: [u8; 4],
}

impl Default for ExplodeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MultiresModifierData {
    pub lvl: i8,
    pub sculptlvl: i8,
    pub renderlvl: i8,
    pub totlvl: i8,
    pub flags: MultiresModifierFlag,
    pub _pad: [u8; 2],
}

impl Default for MultiresModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct FluidsimModifierData {
    pub modifier: ModifierData,
    pub fss: *mut core::ffi::c_void,
    pub _pad1: *mut core::ffi::c_void,
}

impl Default for FluidsimModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SmokeModifierData {
    pub modifier: ModifierData,
    pub r#type: i32,
    pub _pad: i32,
}

impl Default for SmokeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ShrinkwrapModifierData {
    pub modifier: ModifierData,
    pub target: *mut core::ffi::c_void,
    pub auxTarget: *mut core::ffi::c_void,
    pub vgroup_name: [u8; 64],
    pub keepDist: f32,
    pub shrinkType: i16,
    pub shrinkOpts: i8,
    pub shrinkMode: i8,
    pub projLimit: f32,
    pub projAxis: i8,
    pub subsurfLevels: i8,
    pub _pad: [u8; 2],
}

impl Default for ShrinkwrapModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SimpleDeformModifierData {
    pub modifier: ModifierData,
    pub origin: *mut core::ffi::c_void,
    pub vgroup_name: [u8; 64],
    pub factor: f32,
    pub limit: [f32; 2],
    pub _1: f32,
}

impl Default for SimpleDeformModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ShapeKeyModifierData {
    pub modifier: ModifierData,
}

impl Default for ShapeKeyModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SolidifyModifierData {
    pub modifier: ModifierData,
    pub defgrp_name: [u8; 64],
    pub shell_defgrp_name: [u8; 64],
    pub rim_defgrp_name: [u8; 64],
    pub offset: f32,
    pub offset_fac: f32,
    pub offset_fac_vg: f32,
    pub offset_clamp: f32,
    pub mode: SolidifyModifierMode,
    pub nonmanifold_offset_mode: SolifyModifierNonManifoldOffsetMode,
    pub nonmanifold_boundary_mode: SolidifyModifierNonManifoldBoundaryMode,
    pub _pad: i8,
}

impl Default for SolidifyModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct ScrewModifierData {
    pub modifier: ModifierData,
    pub ob_axis: *mut core::ffi::c_void,
    pub steps: u32,
    pub render_steps: u32,
    pub iter: u32,
    pub screw_ofs: f32,
    pub angle: f32,
    pub merge_dist: f32,
    pub flag: ScrewModifierFlag,
    pub axis: i8,
    pub _pad: [u8; 5],
}

impl Default for ScrewModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct OceanModifierData {
    pub modifier: ModifierData,
    pub ocean: *mut core::ffi::c_void,
    pub oceancache: *mut core::ffi::c_void,
    pub resolution: i32,
    pub viewport_resolution: i32,
    pub spatial_size: i32,
    pub wind_velocity: f32,
    pub damp: f32,
    pub smallest_wave: f32,
    pub depth: f32,
    pub wave_alignment: f32,
    pub wave_direction: f32,
    pub wave_scale: f32,
    pub chop_amount: f32,
    pub foam_coverage: f32,
    pub time: f32,
    pub spectrum: OceanModifierSpectrum,
    pub fetch_jonswap: f32,
    pub sharpen_peak_jonswap: f32,
    pub bakestart: i32,
    pub bakeend: i32,
    pub cachepath: [u8; 1024],
    pub foamlayername: [u8; 68],
    pub spraylayername: [u8; 68],
    pub cached: i8,
    pub geometry_mode: OceanModifierGeometryMode,
    pub flag: OceanModifierFlag,
}

impl Default for OceanModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WarpModifierData {
    pub modifier: ModifierData,
    pub texture: *mut core::ffi::c_void,
    pub map_object: *mut core::ffi::c_void,
    pub map_bone: [u8; 64],
    pub uvlayer_name: [u8; 68],
    pub _pad1: [u8; 4],
}

impl Default for WarpModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WeightVGEditModifierData {
    pub modifier: ModifierData,
    pub defgrp_name: [u8; 64],
    pub edit_flags: WeigthVGEditModifierEditFlags,
}

impl Default for WeightVGEditModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WeightVGMixModifierData {
    pub modifier: ModifierData,
    pub defgrp_name_a: [u8; 64],
    pub defgrp_name_b: [u8; 64],
    pub default_weight_a: f32,
    pub default_weight_b: f32,
    pub mix_mode: WeightVGMixModifierMixMode,
    pub mix_set: WeightVGMixModifierMixSet,
    pub _pad0: [u8; 6],
}

impl Default for WeightVGMixModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WeightVGProximityModifierData {
    pub modifier: ModifierData,
    pub defgrp_name: [u8; 64],
    pub cmap_curve: *mut core::ffi::c_void,
    pub proximity_mode: WeightVGProximityModifierProximityMode,
    pub proximity_flags: WeightVGProximityModifierFlag,
    pub proximity_ob_target: *mut core::ffi::c_void,
    pub mask_constant: f32,
    pub mask_defgrp_name: [u8; 64],
    pub mask_tex_use_channel: WeightVGProximityModifierMaskTexChannel,
    pub mask_texture: *mut core::ffi::c_void,
    pub mask_tex_map_obj: *mut core::ffi::c_void,
    pub mask_tex_map_bone: [u8; 64],
    pub mask_tex_mapping: DisplaceModifierTexMapping,
    pub mask_tex_uvlayer_name: [u8; 68],
    pub _pad1: [u8; 4],
}

impl Default for WeightVGProximityModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DynamicPaintModifierData {
    pub modifier: ModifierData,
    pub canvas: *mut core::ffi::c_void,
    pub brush: *mut core::ffi::c_void,
    pub r#type: DynamicPaintModifierType,
    pub _pad: [u8; 4],
}

impl Default for DynamicPaintModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RemeshModifierData {
    pub modifier: ModifierData,
    pub threshold: f32,
    pub scale: f32,
    pub hermite_num: f32,
    pub depth: i8,
    pub flag: eRemeshModifierFlags,
    pub mode: eRemeshModifierMode,
    pub _pad: i8,
}

impl Default for RemeshModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SkinModifierData {
    pub modifier: ModifierData,
    pub branch_smoothing: f32,
    pub flag: SkinModifierFlag,
}

impl Default for SkinModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct TriangulateModifierData {
    pub modifier: ModifierData,
    pub flag: TriangulateModifierFlag,
}

impl Default for TriangulateModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LaplacianSmoothModifierData {
    pub modifier: ModifierData,
    pub lambda: f32,
    pub lambda_border: f32,
    pub _pad1: [u8; 4],
}

impl Default for LaplacianSmoothModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CorrectiveSmoothDeltaCache {

}

impl Default for CorrectiveSmoothDeltaCache {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct CorrectiveSmoothModifierData {
    pub modifier: ModifierData,
    pub bind_coords_sharing_info: *mut core::ffi::c_void,
    pub bind_coords_num: u32,
    pub lambda: f32,
    pub scale: f32,
    pub repeat: i16,
    pub flag: CorrectiveSmoothModifierFlag,
}

impl Default for CorrectiveSmoothModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct UVWarpModifierData {
    pub modifier: ModifierData,
    pub axis_u: i8,
    pub axis_v: i8,
    pub flag: UVWarpModifierFlag,
}

impl Default for UVWarpModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MeshCacheModifierData {
    pub modifier: ModifierData,
    pub flag: MeshCacheModifierFlag,
}

impl Default for MeshCacheModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct LaplacianDeformModifierData {
    pub modifier: ModifierData,
    pub anchor_grp_name: [u8; 64],
    pub verts_num: i32,
    pub repeat: i32,
    pub vertexco: *mut core::ffi::c_void,
    pub vertexco_sharing_info: *mut core::ffi::c_void,
    pub cache_system: *mut core::ffi::c_void,
    pub flag: LaplacianDeformModifierFlag,
}

impl Default for LaplacianDeformModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WireframeModifierData {
    pub modifier: ModifierData,
    pub defgrp_name: [u8; 64],
    pub offset: f32,
    pub offset_fac: f32,
    pub offset_fac_vg: f32,
    pub crease_weight: f32,
    pub flag: WireframeModifierFlag,
    pub mat_ofs: i16,
    pub _pad: [u8; 4],
}

impl Default for WireframeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WeldModifierData {
    pub modifier: ModifierData,
    pub merge_dist: f32,
    pub defgrp_name: [u8; 64],
    pub mode: WeldModifierMode,
    pub flag: WeldModifierFlag,
}

impl Default for WeldModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct DataTransferModifierData {
    pub modifier: ModifierData,
    pub ob_source: *mut core::ffi::c_void,
    pub data_types: i32,
    pub vmap_mode: i32,
    pub emap_mode: i32,
    pub lmap_mode: i32,
    pub pmap_mode: i32,
    pub map_max_distance: f32,
    pub map_ray_radius: f32,
    pub islands_precision: f32,
    pub _pad1: [u8; 4],
}

impl Default for DataTransferModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NormalEditModifierData {
    pub modifier: ModifierData,
    pub defgrp_name: [u8; 64],
    pub target: *mut core::ffi::c_void,
    pub mode: NormalEditModifierMode,
    pub flag: NormalEditModifierFlag,
}

impl Default for NormalEditModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MeshSeqCacheModifierData {
    pub modifier: ModifierData,
    pub cache_file: *mut core::ffi::c_void,
    pub object_path: [u8; 1024],
    pub read_flag: MeshSeqCacheModifierReadFlag,
    pub _pad: [u8; 3],
}

impl Default for MeshSeqCacheModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SDefBind {
    pub vert_inds: *mut core::ffi::c_void,
    pub verts_num: u32,
    pub mode: SurfaceDeformModifierBindMode,
    pub vert_weights: *mut core::ffi::c_void,
    pub normal_dist: f32,
    pub influence: f32,
}

impl Default for SDefBind {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SDefVert {
    pub binds: *mut core::ffi::c_void,
    pub binds_num: u32,
    pub vertex_idx: u32,
}

impl Default for SDefVert {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct SurfaceDeformModifierData {
    pub modifier: ModifierData,
    pub depsgraph: *mut core::ffi::c_void,
    pub target: *mut core::ffi::c_void,
    pub verts: *mut core::ffi::c_void,
    pub verts_sharing_info: *mut core::ffi::c_void,
    pub falloff: f32,
    pub mesh_verts_num: u32,
    pub bind_verts_num: u32,
    pub target_verts_num: u32,
    pub target_polys_num: u32,
    pub flags: SurfaceDeformModifierFlag,
}

impl Default for SurfaceDeformModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct WeightedNormalModifierData {
    pub modifier: ModifierData,
    pub defgrp_name: [u8; 64],
    pub mode: WeightedNormalModifierMode,
    pub flag: WeightedNormalModifierFlag,
}

impl Default for WeightedNormalModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodesModifierSettings {
    pub properties: *mut core::ffi::c_void,
}

impl Default for NodesModifierSettings {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodesModifierDataBlock {
    pub id_name: *mut core::ffi::c_void,
    pub lib_name: *mut core::ffi::c_void,
    pub id: *mut core::ffi::c_void,
    pub id_type: i32,
    pub _pad: [u8; 4],
}

impl Default for NodesModifierDataBlock {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodesModifierBakeFile {
    pub name: *mut core::ffi::c_void,
    pub packed_file: *mut core::ffi::c_void,
}

impl Default for NodesModifierBakeFile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodesModifierPackedBake {
    pub meta_files_num: i32,
    pub blob_files_num: i32,
    pub meta_files: *mut core::ffi::c_void,
    pub blob_files: *mut core::ffi::c_void,
}

impl Default for NodesModifierPackedBake {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodesModifierBake {
    pub id: i32,
    pub flag: NodesModifierBakeFlag,
}

impl Default for NodesModifierBake {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct NodesModifierData {
    pub modifier: ModifierData,
    pub node_group: *mut core::ffi::c_void,
    pub settings_legacy: NodesModifierSettings,
    pub bake_directory: *mut core::ffi::c_void,
    pub flag: NodesModifierFlag,
}

impl Default for NodesModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct MeshToVolumeModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub resolution_mode: MeshToVolumeModifierResolutionMode,
    pub voxel_size: f32,
    pub voxel_amount: i32,
    pub interior_band_width: f32,
    pub density: f32,
    pub _pad2: [u8; 4],
}

impl Default for MeshToVolumeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct VolumeDisplaceModifierData {
    pub modifier: ModifierData,
    pub texture: *mut core::ffi::c_void,
    pub texture_map_object: *mut core::ffi::c_void,
    pub texture_map_mode: VolumeDisplaceModifierTextureMapMode,
    pub strength: f32,
    pub texture_mid_level: [f32; 3],
}

impl Default for VolumeDisplaceModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct VolumeToMeshModifierData {
    pub modifier: ModifierData,
    pub object: *mut core::ffi::c_void,
    pub threshold: f32,
    pub adaptivity: f32,
    pub flag: VolumeToMeshFlag,
}

impl Default for VolumeToMeshModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilModifierInfluenceData {
    pub flag: GreasePencilModifierInfluenceFlag,
}

impl Default for GreasePencilModifierInfluenceData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilOpacityModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilOpacityModifierFlag,
}

impl Default for GreasePencilOpacityModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilSubdivModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub r#type: GreasePencilSubdivideType,
    pub level: i32,
    pub _pad: [u8; 8],
}

impl Default for GreasePencilSubdivModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilColorModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub color_mode: GreasePencilModifierColorMode,
    pub _pad1: [u8; 3],
}

impl Default for GreasePencilColorModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilTintModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilTintModifierFlag,
}

impl Default for GreasePencilTintModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilSmoothModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: eGreasePencilSmooth_Flag,
    pub factor: f32,
    pub step: i32,
    pub _pad: [u8; 4],
}

impl Default for GreasePencilSmoothModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilOffsetModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilOffsetModifierFlag,
}

impl Default for GreasePencilOffsetModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilNoiseModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: i32,
    pub factor: f32,
    pub factor_strength: f32,
    pub factor_thickness: f32,
    pub factor_uvs: f32,
    pub noise_scale: f32,
    pub noise_offset: f32,
    pub noise_mode: i16,
    pub _pad: [u8; 2],
}

impl Default for GreasePencilNoiseModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilMirrorModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub object: *mut core::ffi::c_void,
    pub flag: GreasePencilMirrorModifierFlag,
    pub _pad: [u8; 4],
}

impl Default for GreasePencilMirrorModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilThickModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilThicknessModifierFlag,
}

impl Default for GreasePencilThickModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLatticeModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub object: *mut core::ffi::c_void,
    pub strength: f32,
    pub _pad: [u8; 4],
}

impl Default for GreasePencilLatticeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilDashModifierSegment {
    pub name: [u8; 64],
    pub dash: i32,
    pub gap: i32,
    pub radius: f32,
    pub opacity: f32,
    pub mat_nr: i32,
    pub flag: GreasePencilDashModifierFlag,
}

impl Default for GreasePencilDashModifierSegment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilDashModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub segments_array: *mut core::ffi::c_void,
    pub segments_num: i32,
    pub segment_active_index: i32,
    pub dash_offset: i32,
    pub _pad: [u8; 4],
}

impl Default for GreasePencilDashModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilMultiModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilMultiplyModifierFlag,
}

impl Default for GreasePencilMultiModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLengthModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: i32,
    pub start_fac: f32,
    pub end_fac: f32,
    pub rand_start_fac: f32,
    pub rand_end_fac: f32,
    pub rand_offset: f32,
    pub overshoot_fac: f32,
    pub seed: i32,
    pub step: i32,
    pub mode: i32,
    pub _pad: [u8; 4],
}

impl Default for GreasePencilLengthModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilWeightAngleModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilWeightAngleModifierFlag,
}

impl Default for GreasePencilWeightAngleModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilArrayModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub object: *mut core::ffi::c_void,
    pub count: i32,
    pub flag: GreasePencilArrayModifierFlag,
    pub offset: [f32; 3],
    pub _0: f32,
    pub _0_1: f32,
}

impl Default for GreasePencilArrayModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilWeightProximityModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilWeightProximityFlag,
}

impl Default for GreasePencilWeightProximityModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilHookModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub object: *mut core::ffi::c_void,
    pub subtarget: [u8; 64],
    pub _pad: [u8; 4],
}

impl Default for GreasePencilHookModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilLineartModifierData {
    pub modifier: ModifierData,
    pub edge_types: u16,
    pub source_type: GreasePencilLineartModifierSource,
    pub use_multiple_levels: i8,
    pub level_start: i16,
    pub level_end: i16,
    pub source_camera: *mut core::ffi::c_void,
    pub light_contour_object: *mut core::ffi::c_void,
    pub source_object: *mut core::ffi::c_void,
    pub source_collection: *mut core::ffi::c_void,
    pub target_material: *mut core::ffi::c_void,
    pub target_layer: [u8; 64],
    pub source_vertex_group: [u8; 64],
    pub vgname: [u8; 64],
    pub overscan: f32,
    pub shadow_camera_fov: f32,
    pub shadow_camera_size: f32,
    pub shadow_camera_near: f32,
    pub shadow_camera_far: f32,
    pub opacity: f32,
    pub radius: f32,
    pub thickness_legacy: i16,
    pub mask_switches: eGreasePencilLineartMaskSwitches,
}

impl Default for GreasePencilLineartModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilArmatureModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub object: *mut core::ffi::c_void,
    pub deformflag: i16,
    pub _pad: [u8; 6],
}

impl Default for GreasePencilArmatureModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilTimeModifierSegment {
    pub name: [u8; 64],
    pub segment_start: i32,
    pub segment_end: i32,
    pub segment_mode: GreasePencilTimeModifierSegmentMode,
    pub segment_repeat: i32,
}

impl Default for GreasePencilTimeModifierSegment {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilTimeModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub flag: GreasePencilTimeModifierFlag,
    pub offset: i32,
    pub frame_scale: f32,
    pub mode: GreasePencilTimeModifierMode,
    pub sfra: i32,
    pub efra: i32,
    pub segments_array: *mut core::ffi::c_void,
    pub segments_num: i32,
    pub segment_active_index: i32,
}

impl Default for GreasePencilTimeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilEnvelopeModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub mode: GreasePencilEnvelopeModifierMode,
    pub mat_nr: i32,
    pub thickness: f32,
    pub strength: f32,
    pub skip: i32,
    pub spread: i32,
}

impl Default for GreasePencilEnvelopeModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilOutlineModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub object: *mut core::ffi::c_void,
    pub flag: GreasePencilOutlineModifierFlag,
    pub thickness: i32,
    pub sample_length: f32,
    pub subdiv: i32,
    pub outline_material: *mut core::ffi::c_void,
}

impl Default for GreasePencilOutlineModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilShrinkwrapModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub target: *mut core::ffi::c_void,
    pub aux_target: *mut core::ffi::c_void,
    pub keep_dist: f32,
    pub shrink_type: i16,
    pub shrink_opts: i8,
    pub shrink_mode: i8,
    pub proj_limit: f32,
    pub proj_axis: i8,
    pub subsurf_levels: i8,
    pub _pad: [u8; 2],
}

impl Default for GreasePencilShrinkwrapModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilBuildModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub start_frame: f32,
    pub end_frame: f32,
    pub start_delay: f32,
    pub length: f32,
    pub flag: GreasePencilBuildFlag,
}

impl Default for GreasePencilBuildModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilSimplifyModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub mode: GreasePencilSimplifyModifierMode,
    pub _pad: [u8; 4],
}

impl Default for GreasePencilSimplifyModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct GreasePencilTextureModifierData {
    pub modifier: ModifierData,
    pub influence: GreasePencilModifierInfluenceData,
    pub uv_offset: f32,
    pub uv_scale: f32,
    pub fill_rotation: f32,
    pub fill_offset: [f32; 2],
    pub _0: f32,
}

impl Default for GreasePencilTextureModifierData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

