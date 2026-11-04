//! Blender makesdna compiler and parser implementation ported to Rust
//! Corresponds to:
//! dna_genfile.cc, dna_parse.cc, dna_utils.cc, makesdna.cc

use blender_dna::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DnaStructMember {
    pub type_name: String,
    pub name: String,
    pub size: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DnaStructDef {
    pub name: String,
    pub size: usize,
    pub members: Vec<DnaStructMember>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DnaDatabase {
    pub structs: Vec<DnaStructDef>,
    pub type_names: Vec<String>,
}

impl DnaDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_struct(&mut self, def: DnaStructDef) {
        self.structs.push(def);
    }

    pub fn find_struct(&self, name: &str) -> Option<&DnaStructDef> {
        self.structs.iter().find(|s| s.name == name)
    }
}
