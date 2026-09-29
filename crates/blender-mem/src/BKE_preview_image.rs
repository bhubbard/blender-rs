//! Auto-transpiled C/C++ header module: BKE_preview_image

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewImageRuntime {
    pub icon_id: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewImageRenderEndStatus {
    PRV_RENDER_STATUS_FINISHED,
    PRV_RENDER_STATUS_FAILED,
    PRV_RENDER_STATUS_CANCELLED,
}
