//! Auto-transpiled C/C++ header module: BKE_attribute

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrDomainMask {
    ATTR_DOMAIN_MASK_POINT = (1 << 0),
    ATTR_DOMAIN_MASK_EDGE = (1 << 1),
    ATTR_DOMAIN_MASK_FACE = (1 << 2),
    ATTR_DOMAIN_MASK_CORNER = (1 << 3),
    ATTR_DOMAIN_MASK_CURVE = (1 << 4),
    ATTR_DOMAIN_MASK_GREASE_PENCIL_LAYER = (1 << 6),
    ATTR_DOMAIN_MASK_ALL = (1 << 7) - 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeOwnerType {
    Mesh,
    PointCloud,
    Curves,
    GreasePencil,
    GreasePencilDrawing,
}
