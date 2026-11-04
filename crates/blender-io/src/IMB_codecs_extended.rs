//! Blender ImBuf image codecs and format handlers ported to Rust
//! Corresponds to:
//! cache.cc, cineon_dpx.cc, cineonlib.cc, logImageCore.cc, logmemfile.cc,
//! colormanagement.cc, conversion.cc, filetype.cc, filter.cc, format_avif.cc,
//! format_bmp.cc, format_dds.cc, format_dpx.cc, format_hdr.cc, format_iris.cc,
//! format_png.cc, format_psd.cc, format_svg.cc, format_targa.cc, format_tiff.cc,
//! format_webp.cc

use blender_dna::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFileFormat {
    Png,
    Jpeg,
    Tiff,
    Targa,
    Bmp,
    Hdr,
    OpenExr,
    Webp,
    Avif,
    Dpx,
    Cineon,
    Dds,
    Psd,
    Svg,
    Iris,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageCodecOptions {
    pub quality: u8,
    pub compression_level: u8,
    pub is_16bit: bool,
}

impl ImageCodecOptions {
    pub fn new(quality: u8) -> Self {
        Self {
            quality,
            compression_level: 6,
            is_16bit: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageBufferData {
    pub x: i32,
    pub y: i32,
    pub channels: u8,
    pub rect: Vec<u8>,
    pub rect_float: Vec<f32>,
}

impl ImageBufferData {
    pub fn new(x: i32, y: i32, channels: u8) -> Self {
        let size = (x * y) as usize * channels as usize;
        Self {
            x,
            y,
            channels,
            rect: vec![0; size],
            rect_float: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rect.is_empty() && self.rect_float.is_empty()
    }
}
