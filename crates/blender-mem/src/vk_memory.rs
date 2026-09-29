//! Auto-transpiled C/C++ header module: vk_memory

pub type VkDeviceSize = u64;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VKMemoryExport {
    pub handle: u64,
    pub memory_size: VkDeviceSize,
    pub memory_offset: VkDeviceSize,
}
