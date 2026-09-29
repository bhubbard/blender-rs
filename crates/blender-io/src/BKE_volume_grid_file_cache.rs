use crate::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GridsFromFile {
    pub grids: Vec<GVolumeGrid>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GVolumeGrid {
    pub volume: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeGrid {
    pub volume: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Grid {
    pub volume: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GridCollection {
    pub grids: Vec<Grid>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VolumeCollection {
    pub volumes: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GridCollectionVolume {
    pub grids: Vec<Grid>,
    pub volumes: Vec<f64>,
}
