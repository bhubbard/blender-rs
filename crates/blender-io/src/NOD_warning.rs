//! Safe Rust port of Blender's `source/blender/nodes/NOD_warning.hh`.

/// Node warning type mirroring Blender's `blender::nodes::NodeWarningType`.
/// Values are serialized into `.blend` files.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NodeWarningType {
    #[default]
    Error = 0,
    Warning = 1,
    Info = 2,
}

impl NodeWarningType {
    pub fn severity(self) -> i32 {
        match self {
            Self::Error => 3,
            Self::Warning => 2,
            Self::Info => 1,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Info => "Info",
        }
    }
}

/// Node warning record mirroring `blender::nodes::NodeWarning`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NodeWarning {
    pub warning_type: NodeWarningType,
    pub message: String,
}

impl NodeWarning {
    pub fn new(warning_type: NodeWarningType, message: impl Into<String>) -> Self {
        Self {
            warning_type,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(NodeWarningType::Error, message)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(NodeWarningType::Warning, message)
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(NodeWarningType::Info, message)
    }
}
