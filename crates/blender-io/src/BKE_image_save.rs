//! Auto-transpiled C/C++ header module: BKE_image_save

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageFormatData {
    pub media_type: i32,
    pub imtype: i8,
    pub depth: i8,
    pub quality: i8,
    pub compress: i8,
    pub planes: i8,
    pub flag: i8,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageSaveOptions {
    pub im_format: ImageFormatData,
    pub relative: bool,
    pub save_copy: bool,
    pub save_as_render: bool,
    pub do_newpath: bool,
    pub orig_imtype: i32,
    pub prev_save_as_render: bool,
    pub prev_imtype: i32,
}
