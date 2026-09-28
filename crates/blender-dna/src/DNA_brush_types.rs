pub type eGP_FillLayerModes = i32;
pub type eBrushFlags = i32;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ID {
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MTex {
    pub texco: i16,
    pub mapto: i16,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BrushGpencilSettings {
    pub draw_smoothfac: f32,
    pub fill_factor: f32,
    pub draw_strength: f32,
    pub draw_jitter: f32,
    pub draw_angle: f32,
    pub draw_angle_factor: f32,
    pub draw_random_press: f32,
    pub draw_random_strength: f32,
    pub draw_smoothlvl: i16,
    pub draw_subdivide: i16,
    pub fill_layer_mode: eGP_FillLayerModes,
    pub fill_direction: i16,
    pub fill_threshold: f32,
    pub fill_solver: i16,
    pub caps_type: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BrushCurvesSculptSettings {
    pub add_amount: i32,
    pub points_per_curve: i32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Brush {
    pub id: ID,
    pub mtex: MTex,
    pub mask_mtex: MTex,
    pub normal_weight: f32,
    pub rake_factor: f32,
    pub blend: i16,
    pub ob_mode: i16,
    pub weight: f32,
    pub size: i32,
    pub flag: eBrushFlags,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PaletteColor {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Palette {
    pub id: ID,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PaintCurvePoint {

}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PaintCurve {
    pub id: ID,
    pub tot_points: i32,
    pub add_index: i32,
}
