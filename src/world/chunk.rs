use super::block::Block;

#[derive(Debug)]
pub struct BitArray {
    pub data: Vec<u32>,
    pub capacity: u32,
    pub bits_per_value: u32,
    pub value_mask: u32
}

#[derive(Debug)]
pub struct ChunkSection {
    pub data: BitArray,
    pub palette: Vec<u32>,
}

#[derive(Debug)]
pub struct ChunkColumn {
    pub x: i32,
    pub z: i32,
    pub sections: Vec<ChunkSection>
}

struct ChunkGrid {
    pub position: [i32; 3],
    pub blocks: [[[Block; 16]; 16]; 16]
}
