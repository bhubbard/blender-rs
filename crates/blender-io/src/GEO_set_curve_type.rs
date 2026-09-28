//! Auto-transpiled C/C++ header module: GEO_set_curve_type

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConvertCurvesOptions {
    pub convert_bezier_handles_to_poly_points: bool,
    pub convert_bezier_handles_to_catmull_rom_points: bool,
    pub keep_bezier_shape_as_nurbs: bool,
    pub keep_catmull_rom_shape_as_nurbs: bool,
}
