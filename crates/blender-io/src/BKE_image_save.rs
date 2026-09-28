//! Auto-transpiled C/C++ header module: BKE_image_save

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
