use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BakeImage {
    pub image: *mut Image,
    pub tile_number: i32,
    pub uv_offset: [f32; 2],
    pub width: i32,
    pub height: i32,
    pub offset: usize,
    pub render_layer_name: [i8; 255], // Define RE_MAXNAME
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeTargets {
    pub images: *mut BakeImage,
    pub images_num: i32,
    pub material_to_image: *mut *mut Image,
    pub materials_num: i32,
    pub result: *mut f32,
    pub pixels_num: i32,
    pub channels_num: i32,
    pub is_noncolor: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakePixel {
    pub primitive_id: i32,
    pub object_id: i32,
    pub seed: i32,
    pub uv: [f32; 2],
    pub du_dx: f32,
    pub du_dy: f32,
    pub dv_dx: f32,
    pub dv_dy: f32,
    pub is_margin: bool,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BakeHighPolyData {
    pub ob: *mut Object,
    pub ob_eval: *mut Object,
    pub mesh: *mut Mesh,
    pub is_flip_object: bool,
    pub obmat: [[f32; 4]; 4],
    pub imat: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Depsgraph {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImBuf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Render {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Image {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    pub _opaque: [u8; 0],
}
